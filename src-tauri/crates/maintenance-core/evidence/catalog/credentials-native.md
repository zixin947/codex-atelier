# 原生凭据 schema、迁移与真实存储

输入只读 binary SHA256 `b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。
下列 C 文件均位于 `/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/codex-direct/decompiled`，短 ASM 位于本文同目录。实现为 `src/catalog/credentials.rs`；不读取环境/home，也不自动调用任何 AiMaMi service。所有根目录、service、混淆 domain/host/user 均由 Atelier 显式传入。

## Schema 与原生分支

| 输入/规则 | 原始证据 | 实现 |
|---|---|---|
| Credential 是内部 tagged enum `kind`，`legacy`、`none`、`revision`；revision 字段名为 `id`，不是 `revision` | `RelayCredential_deserialize{,_alt}.c @10079d4ec/10079d6d4`；`credential-kind.asm @10079c2e0` 逐分支 serialize；`RelayProvider_serialize.c:26,86-89` Legacy 不序列化 | `RelayCredential`；Provider 缺字段默认 Legacy，显式 null 拒绝 |
| revision 恰好32个ASCII小写hex，既不要求UUID分隔符，也不检查UUID版本 | `credential-revision.asm @10079f9a4..10079fd90`，字节扫描与长度32比较 | `CredentialRevision::try_from` |
| Legacy 总允许；None 仅schema>=16且apiKeyStored=false；Revision仅schema>=16、有效provider ID且apiKeyStored=true | `validate_relay_credentials.c:20-97 @1002053ec` | `validate_credentials`，读取state时也执行；保留native冲突错误文本 |
| legacy ID允许旧格式，但必须非空、非点/双点，无控制字符、slash/backslash | `legacy_id_is_safe_identifier.c @1008e2674` + `legacy-id-safe.asm`，Unicode控制区间含U+007F..009F | `legacy_id_is_safe_identifier`；current ID仍使用既有严格validator |

`ProviderCredential`只投影id/apiKeyStored/credential；非凭据字段不因投影而丢失。当前credential未知kind、缺revision.id、错类型均报错，不用调用者布尔值代替schema判断。

## 具体存储与顺序

- `keychain_get_api_key.c @1008e0144`：先validate current ID并lock，读 `root/keys/{providerId}`。现存空文件是删除屏障，返回None且不回退；文件缺失才读 `secrets.json`，解码非空则写private key文件并返回；legacy项为空才继续macOS keychain。损坏legacy map/base64/UTF8直接报错，不能跳过损坏后碰巧从下一来源取值。
- `keychain_read_key_file.c @1008e064c`：真实文件内容trim；present-empty与NotFound分离。`keychain_write_key_file.c @1008e0a4c`：keys目录、尝试mode0700、private原子文件0600。`keychain_set_api_key.c @1008e0504`当前set只写文件，不写真实Keychain。
- `keychain_read_legacy_secret.c @1008e0da4`、`keychain_read_legacy_secrets.c @1008e14ec`：JSON string map，STANDARD base64，循环XOR。`legacy-xor.asm @100539234`直接核对 `key[index % keylen]`。SHA256 salt原生为 `aimami-relay::{hostname}::{username}`，格式池`101531ea0`。本实现domain/host/user全显式参数化，使用调用方自己的namespace。
- `mac_keychain_get.c @1007a2854`、`mac_keychain_delete.c @1007a2a38`：`PasswordOptions::new_generic_password`后`use_protected_keychain`；get使用generic password，item-not-found(-25300)为None，其余Err，成功bytes须UTF8。service池`1013b2df0`明确为`dev.aimami.relay`，account为传入provider ID。实现`MacKeychain`以Security/CoreFoundation FFI调用`SecItemCopyMatching/SecItemDelete`，query含genericPassword/service/account/data-protection，read含returnData/matchLimitOne。无第三方新依赖；没有该service的默认值，没有执行真实查询。
- `MemoryKeychain`是具体内存实现，记录get/delete并支持单次I/O错误；所有测试只用它。`MacKeychain`在macOS本机只编译链接，未执行任何get/delete。
- `keychain_delete_api_key.c @1008e08ec`：写空文件屏障，随后由显式cleanup单独清legacy存储。`keychain_cleanup_deleted.c @1008e1d30`重新读取屏障，实际仍为空才清；内容变化拒绝清理。`keychain_cleanup_legacy_stores.c @1008e17d8`移除旧map项并private发布，再尝试protected keychain delete；失败返回cleanup error kinds，已发布状态不回滚成“未发布”。

## Provider ID 与旧明文迁移

`migrate_and_publish_state(state, original_bytes, destination, store)`是实际事务入口；保留原JSON未知字段，使用已恢复ID planner、protocol、slot迁移，不依赖调用方标记“可迁移”。

1. 校验输入schema/catalog/proxy/credential；存在无效ID计划时，使用已有private writer保存原始bytes备份并读回memcmp。
2. 按`apply_provider_id_migrations.c:64-145 @100208ca0`，仅apiKeyStored=true才实际copy。`keychain_copy_legacy.c @1008e2234`先读新ID文件：非空直接成功，空屏障直接false。不存在时安全旧ID依次读旧文件、旧legacy map、protected keychain，只有真实非空key写入新ID后才返回true。原来源在state发布前保留。
3. 查找不到时清apiKeyStored并设原生固定lastError `旧 Provider ID 已安全迁移；原 API Key 无法自动定位，请重新输入`；copy I/O失败也清标记，lastError使用原生前缀及error。两者仍迁ID和引用，不伪装copy成功。成功复制的旧ID才记录待清理。
4. `relay_storage_load.c:775-825 @10020a704`：原schema<=1，原JSON apiKey为非空string且最终ID非空时，先调用实际set_api_key，成功后标记apiKeyStored；写key失败立即返回，状态文件和调用方document保留。Provider原生Deserialize不保留apiKey，输出删除该旧字段，绝不写回明文。
5. 执行官方协议/slot迁移、schema升16并validate，真实原子发布state；rename后目录sync失败按原ack语义返回已发布且携带warning。
6. 仅发布成功才替换调用方document，并按`relay_storage_load.c:884-914`清理已成功copy的旧ID。`keychain_cleanup_legacy_provider_key.c @1008e1f14`先为旧ID写空屏障，成功后再清legacy map/keychain。状态发布失败保留原document与旧来源；可能留下已复制的新私有key供恢复，不清旧key。

原生load的诸多错误日志/返回包装没有逐字模拟；公开入口用Result准确报告实际提交结果。原始bytes由上层从自己的state读取后传入，不会自行定位真实AiMaMi数据。

## Revision 文件与发布补偿

- `revision_key_path.c @1008e50c4`：`root/key-revisions/{providerId}.{revision}`，无扩展名。
- `revision_prepare.c @1008e40b8`：key byte长度<=0x100000且trim非空；随机UUID simple revision；真实目录需是非symlink目录、mode0700；create_new mode0600；读取FileIdentity；写实际JSON/sync/close/同步目录及root。
- `StoredKey`四个必填字段为 `format/providerId/revision/apiKey`，deny_unknown_fields：field visitor `@1008f166c`的未知字段直接error（`stored-key-fields.asm`）；format固定`aimami-relay-key-v1`。
- `revision_read_stored.c @1008e2794`：symlink_metadata文件regular且single-link；打开后核对identity，文件读取上限0x600400；JSON/format/provider/revision/key限制真实检查。`revision_read.c @1008e3c94`还要求文件owner的provider/revision等于请求者。实现`read_credential_revision`直接读文件并验证，不凭apiKeyStored就返回成功。
- `revision_publish.c @1008e4c34`及alt `@1008e4e80`：准备文件先存在，state commit成功再保留；失败恢复标记并清理未发布准备文件。`PreparedCredential_drop @100879758`仅删除owned inode。实现`PreparedCredential` RAII和`publish_credential_revisions`，真实state写失败时Drop清理新文件、保留原state；已替换成外来inode/symlink时不删除外来文件。

## 验证和边界

`cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test catalog_native`：29 passed，0 failed，无编译警告。覆盖全部三类schema、缺字段/null/冲突、真实文件优先级和删除屏障、base64/XOR回退、损坏legacy终止、安全旧ID、stored-key复制成功/缺失/失败、状态失败保留旧来源、发布后cleanup失败、schema1明文先存后去除及写key失败、revision prepare/读回/commit/drop补偿、0600/0700、owner不匹配/unknown字段/limit/symlink。所有输入和输出均是临时合成数据。

revision批量垃圾清理已恢复，详见下节。macOS FFI只有编译验证；真实Keychain和原二进制动态差分未执行。Manager自身commit ID/事务编排由上层使用已恢复prepare/reference/publish接口接入；模块不会自动遍历真实用户目录。

## Revision sweep

`cleanup_unreferenced_revisions(root,state_path,expected)`来自`revision_cleanup_unreferenced.c @1008e2f40`，真实读取state并操作revision文件。目录不存在直接Ok；目录检查用symlink_metadata，非真实目录报错。保存state读取/JSON失败直接拒绝。

`revision-sweep-state-compare.asm @1008fb60c`证明比较仅含schemaVersion、Provider数量和**原顺序**的id/credential kind，Revision再比id bytes；不比较apiKeyStored、名字或proxy。本实现按该序列比较，不把整个state Value相等冒充原生判断。`revision-sweep-references.asm @1008fbb64`只对Revision构造保留路径，不由apiKeyStored布尔值决定。

枚举上限4096条，超过时报`relay credential cleanup budget exceeded; remaining files retained`；预算之前可能已有实际删除，不能当作整个事务未执行。被引用路径直接跳过。未引用项调用真实read_stored_revision验证regular/single-link/identity/JSON/limits，失败或内部provider/revision重建路径不等于该文件名时保留，最终报告`unrecognized relay credential files retained`。

每个删除前重新核对revision目录device/inode；重新读state并与首次读取的**原始bytes**比较，任一变化终止；最后remove_owned再验证文件identity。`revision-remove-owned.asm @100325c80`补充metadata已经NotFound是成功，存在外来inode/链接则报错，绝不删除外来文件。cleanup closure仅释放io::Error包装，不是另一个隐含删除算法。

新增三项实际文件分支测试：保留引用和未知/错名文件、只清除owned未引用版本；schema/provider序列/credential变化拒绝但非凭据字段不影响原生序列比较；4097个真实临时文件验证预算且全部未知文件保留。运行中逐步替换state/目录的竞态注入尚未执行，删除前的实际读取和identity代码保留原生guard，未用调用方valid bool替代。
