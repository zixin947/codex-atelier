# Catalog 原生状态映射与发布

目标只读arm64 SHA256：`b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de`。输入材料根：`/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full`。本说明的C文件位于该根`codex-direct/decompiled`；新增asm位于本目录。未读取真实用户配置/会话/凭据，未运行任何原应用。

## 已实现

- `src/catalog/state.rs`：保留原始object中的opaque字段，实际将native持久化字段投影为既有构造器输入。`RelayModelEntry_serialize.c @10079ce6c` 逐项证实model:string、wireApi枚举、contextWindow:Option<i64>、displayName:Option<String>、suffix:String。`CodexNoAccountSlot_serialize.c @10079f0a0`证实providerId/model/catalogSlug，catalogSlug为Option且None不序列化。
- `RelayState_serialize{,_alt}.c @10079a2bc/10079a51c`证实schemaVersion/providers/activeByIde/proxy/codexRouterEnabled/routerCommitId/codexRouterNoAccountMode/codexNoAccountSlots/claudeWebSearchCompat。`RelayProvider_serialize.c @10079b898`证实id/name/baseUrl/model/wireApi/brand/models及apiKeyStored/credential/healthScore/latencyMs/lastTestedAt/createdAt/updatedAt/lastError/network等非catalog字段。非catalog字段按原始JSON保留，未冒充已恢复其完整内部结构或迁移默认。
- `native_default()`是纯构造函数，按`relay_storage_load.c:145-210` metadata失败分支及state/proxy/active serializer生成schemaVersion16、空providers/active codex/slots、关闭router/no-account、空proxy字符串/port0/runningfalse/lastError null、claudeWebSearchCompat true；routerCommitId为None并省略。不会自动把读取失败的现存状态替换为空状态。
- 顶层Map visitor `@1004831f8` 的跳表位于`10131872e`，只有tag0..10属于字段/unknown/end。Ghidra巨大switch的更大tag是失去枚举边界后的伪分支，未用于推断。end分支`100483750`、`state-defaults.asm`证实缺schemaVersion→16，providers/activeByIde/slots→空，router/no-account→false，compat→true。`active-defaults.asm:10047ca94..10047cab4`证实activeByIde内部缺codex也→空。catalog投影应用已确认默认，保留raw object不补写。显式null/错类型不当作缺字段。schemaVersion真实类型是i32（`100318ccc`调用deserialize_i32），load guard `relay_storage_load.c:725-739`允许所有<=16的i32，因此schema API返回i32并接受负值。
- `RouterOperationId`按TryFrom<String> `@100773820`恢复：uuid parse后检查version高4位=4，随后调用`uuid::LowerHex @101065e94`再与原输入比较（长度、memcmp）；后者`uuid-lowerhex.asm`明确调用format_hyphenated并写36字节。因此输入只接受canonical lowercase hyphenated UUIDv4，错误文本为`router operation id must be a canonical UUID v4`。ASM未加variant位判断，本实现也未加；显式null/缺省为None。setter现在只接受这个已验证类型，不接受任意JSON。
- Provider visitor `@10048e998`真实tag0..19的end `10048f2ac`（`provider-defaults.asm`）：catalog字符串id/name/baseUrl/model缺省空；models空；wireApi=0、brand=0。两个serializer（`provider-{wire,brand}-default.asm`）证实0分别是openai-chat/generic。Model visitor `@100489798` end `100489b18`（`model-defaults.asm`）明确model、wireApi必填，contextWindow/displayName缺省None，suffix缺省空；没有对Model整个struct添加猜测的Default。
- Proxy visitor `@10047f424`的end `10047f770`（`proxy-defaults.asm`）证实RelayProxyStatus全部缺省值，port为u16，running bool、baseUrl/codexBaseUrl字符串、lastError Option<String>。typed读取和setter均已实现；setter只更新已知五字段，保留proxy扩展。目录修复、状态写回均有真实读回测试证明proxy保留。
- `relay_storage_load.c:296-400` 在任何schema版本都把activeByIde.codex string→[string]、null→[]；`from_json/read`应用该形状迁移，`from_value`保持直接投影入口。load在缺schema时用0做迁移分支（serde物化缺schema则是16），因此不混用这两个默认。`relay_storage_load.c:688-705` schema<=14有非null routerCommitId直接拒绝，15/16继续验证UUID。本模块恢复了此guard和claudeWebSearchCompat缺省true/type检查，不自动升schemaVersion。
- `migrate_legacy_minimax_m3_official_protocol.c @1002092dc`、`migrate_legacy_deepseek_v4_flash_official_protocol.c @10020aec0`及load的<10/<13版本门槛已实现为纯迁移。用url::Url实际解析：trim后的https、官方host、空username/无password/query/fragment、path去尾部slash后为空或/v1；不额外限制port。模型trim后最后一个/段ASCII不区分大小写匹配minimax-m3或deepseek-v4-flash，只有Chat→Responses。非空models列表中provider fallback更新不计入native返回计数。
- `migrate_legacy_no_account_slots.c @100370394`、load schema<14门槛已恢复：先过滤inactive/失效target、最多五对去重，保留首次合法且未占用的canonical alias，缺alias按sol/terra/luna/5.5/5.4首次空位补齐。过滤后存在任一valid旧slot时，别名补齐后原生直接返回这些旧slots（ASM 100370968直接b.eq 100370eac）；只有过滤结果为空且no-account=true时，才按active/provider.exposed_models顺序去重自动生成最多五条；此差异有回归测试。各迁移API显式调用，不冒充完整原生storage::load。
- Provider ID planner `plan_provider_id_migrations.c @100208530`：先拒绝重复原ID，预留全部有效ID；无效ID以SHA256原始UTF8前8字节的16位小写hex加前缀aimami_relay_legacy_，碰撞依次加_1/_2。摘要callee `100222a48`实际调用sha256::compress256；`100208748..100208758`传digest[0..8]；LowerHex<u8> formatter调用与格式width2已静态核对。完整stored-key复制、查找/写入失败降级、发布后旧ID清理已在`credentials.rs`恢复，并移除早期无stored-key受限mutator。更新provider ID、active稳定去重、slot providerId时保留未知provider/proxy/alias扩展字段。详见[credentials-native.md](credentials-native.md)。
- 枚举wire值来自原产品前端`aimami-unpacked/assets/relay-page-CeTGPc56.js`与序列化字符串池：wireApi=`openai-chat|openai-responses|anthropic`；brand=`generic|deepseek|kimi|minimax|xiaomi|zhipu|openrouter`。分别映射已有builder的Auto/DeepSeek/Kimi/MiniMax/MiMo/Glm/OpenRouter，OpenRouter不会触发Auto的名称推断。不能把MiMo当成wire字符串`mimo`。
- `publication.rs`实际调用既有构造器生成目录、pretty编码、读取CLI signature并执行真实原子发布，不要求调用方给JSON或valid bool。CLI resolver @1007c03b0的优先路径为`Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex`，失败后`Contents/Resources/codex`；只检查文件，不执行CLI。调用方显式传bundle路径，未自行探测真实应用或home。
- `current_bundled_cli_signature.c @10036ff60`与`signature.asm`：路径无法解析为Unavailable；已解析路径保留path，即便metadata随后失败；metadata成功记录len和可选modified。不是哈希，也不是执行`--version`。
- `write_atomic_report.c @1008a5300`：创建父目录→同目录唯一`.stem.tmp.pid.uuid-simple`临时文件（stem取文件名的前48字符，非UTF8/空时`aimami`）→create_new，mode取目标metadata的0777位，失败时0666→FileIdentity→write_all→sync_all→关闭→verify_owned→rename→同步父目录。UUIDv4由本实现使用OS随机源生成，未引入额外库。
- `staging_FileIdentity_read.c @100325be8`、`staging_verify_owned.c @100325db8`：regular file且nlink=1，记录device/inode；发布时symlink_metadata验证同一identity。替换成symlink/其他inode/多hardlink时拒绝发布，也不删除外来路径。`TempFileCleanup_drop.c @100915638`证实只清理已确认owner的stage。
- `replace_staged_file_report.c @1008a6c30`：rename失败为Err；rename成功后parent open/sync失败依然返回Publication成功并携带directory_sync_error。`publication_acknowledge.c @1008a4e78`仅记录error kind并消费，不把已发布内容误报为未写入。本实现以stderr warning和可读receipt保留这一区别。
- require_sync @1008a4fd0保留原error kind，并使用原固定错误文本`ATOMIC_WRITE_PUBLISHED_SYNC_UNCONFIRMED: replacement exists; retain recovery record\n`。
- `write_atomic_private_report.c @1008a6d40:230-285`强制replacement创建mode0600，忽略原目标mode；另提供实际private writer。`preserve_state_before_provider_id_migration.c @10020975c`恢复原始bytes备份：root/state-migration-backups/state-before-provider-id-migration-{epochMillis}-{attempt}.json.bak，attempt0..99，已存在则下一项，private发布后read+memcmp确认完全相同。测试实际从0644状态迁移到0600，备份保持原始格式/字节不变。

## 验证

`cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test catalog_native`：29 tests通过，无警告。全部路径位于tempfile目录；覆盖真实JSON读回、metadata signature及bundle优先级、普通/private替换与mode、owner stage清理/symlink/hardlink/rename失败、proxy/opaque字段保留、OpenRouter映射、new/default/缺字段/类型/schema/commit guard、官方协议URL/model/version/计数边界、别名筛选/重建、Provider ID哈希碰撞/引用更新/实际stored-key复制与失败补偿，以及原始bytes备份验证。ack目录sync失败分支以类型化Publication错误receipt测试，未宣称真实磁盘fsync故障已注入。

## 精确边界

- 顶层/active/Provider/Model/Proxy的catalog相关schema和缺字段规则已恢复。credential内部schema/校验、stored-key Provider ID迁移与schema<=1明文apiKey导入已恢复；`migrate_and_publish_state`实际先存key、再发布schema16状态、发布成功后清理已复制旧ID。独立纯mutator仍不自动升schema。详见新增凭据证据文档。
- health/timestamps等非catalog字段仍保留原值，未推测其完整内部schema。proxy、routerCommitId及credential已有typed规则；keychain操作只由显式namespace的具体adapter执行。测试从未调用真实keychain。由原生slots重建逻辑新建的alias只包含原生三个已知字段。
- 平台查找bundle路径由上层显式路径adapter承担；此模块已恢复该bundle内CLI resolver和signature，未调用系统应用发现。
- 文件identity实现使用Unix metadata；非Unix返回Unsupported，未声称Windows原子替换等价。原native目标为macOS arm64。
- 未启动原二进制动态差分。验证针对恢复实现的真实文件系统操作与输入分支。
