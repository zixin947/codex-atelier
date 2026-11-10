# Atelier catalog app adapter

实现为 `src-tauri/src/maintenance/diagnostics/native_catalog.rs` 与 `native_catalog_tests.rs`。只使用显式路径，不发现真实home/app，不启动任何程序。

## 两种真实来源

- `Source::Native {state_path,catalog_root,bundle}`读取真实JSON bytes，经`RelayStateDocument::from_json`恢复版本/credential/proxy/active规则，再投影目录输入。检测到Atelier的`profiles/hostId`形状会明确拒绝，不能让native缺字段默认值吞掉异构gateway对象。
- 原生状态路径经静态证据核实：输入`codex-direct/decompiled/CodexPaths_from_home.c @1008efb84:116-130`依次home/codexmate/relay，`:246-249`以该relay目录join `state.json`，长度0xa。`native_state_path(home)`只拼出这个路径，不读真实home，不把不存在的文件自动变成可用状态。
- `Source::Gateway {catalog_path,catalog_root,bundle,route_enabled}`强类型反序列化实际`gateway::types::Catalog`，并核对`profiles<=64/非空hostId`，与`gateway/storage.rs::read_inner`相同。不会反序列化成RelayState再借native默认值猜wire/brand。

## Gateway 能连通的模型身份

`gateway/server.rs:284-296`的`/v1/models`只列`settings.preferredProfileId`对应的首profile的models原始顺序，不合并auto-switch候选、不根据瞬时auth/cooldown删除目录模型。适配器使用相同首选profile及模型列表；没有匹配或model列表为空时，实际目录为空，不生成fallback空字符串模型。

`gateway/server.rs:488-518`的当前生成契约只转发`responses`/`responses/compact`，故CatalogProvider/每个ModelEntry明确使用`WireApi::Responses`、`Brand::Auto`；name/baseUrl/id与model.display_name来自实际typed profile。没有宣称此gateway支持ChatCompletions/Anthropic转发。

原生builder在`builder.rs:196`生成`provider::model`，但gateway的`eligible`和forward直接匹配并转发原始model ID（`server.rs:255-262,410-425`）。因此Atelier分支显式把最终descriptor的slug转换回原始ID、稳定保留首次出现；其它原生能力字段保持构造器输出。直接发布native前缀slug会产生无法路由的模型，本实现没有这样做。Native来源仍保留原生slug。

完整性检查仍复用已恢复的core真实分支。GatewayInventoryFs只将读取的JSON slug投影为`preferredProvider::rawModel`，以和core期望集合比较；读取/JSON损坏/models缺失/空文件/缺模型均由原算法判断，不输入valid bool。Gateway与原版Native账号的消费契约不同：Atelier Responses当前需要静态目录路径，因此Gateway的路径检查使用引用归属规则，要求model_catalog_json指向实际存在的受管目录；完整性集合仍是Gateway原始model ID投影。真正Native来源按真实state区分普通Native（移除静态覆盖）与NoAccount（写入实际目录路径）。这是显式产品适配，不宣称二者消费方式相同。

## 预览与提交

`inspect(source,config_text,writable)`返回catalog_path_validity/catalog_integrity两项。writable只门控fixable，不影响schema、文件和集合判断。

`prepare(source)`读取实际来源并构造完整目录，返回私有`PreparedAction`，公开只读document/destination/revision（原始来源bytes SHA256，与Gateway Store读取revision语义一致）。保存来源bytes、现有输出bytes或缺失、捆绑CLI元数据签名。

`apply(plan)`对Gateway取得`gateway/catalog.lock`，即`gateway::storage::Store::read/update`使用的同一文件锁；持锁重新比对来源bytes、现有输出和CLI签名，再实际原子发布目录并acknowledge。Native分支由上层Manager持有自己的状态gate，模块仍重核全部快照。外部变化返回重新预览，不覆盖新文件。不执行CLI。

目录重建不隐式迁移凭据。`migrate_native_source(source,store)`为单独显式阶段，调用已恢复的真实secret-before-state事务；Gateway凭据属于其现有不同存储，明确拒绝误用native迁移，未访问keychain。`prepare_item(source,config_path,item_id)`现已实现catalog_path_validity/catalog_integrity的目录与TOML具体联合计划，详见下节。旧prepare保留只重建目录行为；上层repair使用prepare_item提交完整路径修复。

## 验证

`cargo test --offline --manifest-path src-tauri/Cargo.toml native_catalog::tests --no-fail-fast`：11 passed；全部测试通过。最后构建的其余维护模块unused/dead_code警告不属于本模块。覆盖实际Gateway Catalog/Profile类型、selected inventory/raw slug/稳定去重/完整descriptor；原生NoAccount路径和完整性缺模型分支；来源/输出/CLI快照变化及真实gateway file lock冲突；异构schema/缺来源拒绝；MemoryKeychain显式namespace的实际plaintext key迁移与gateway拒绝。

所有输入输出为随机临时目录和合成key，未读取真实Gateway/原生状态/配置/钥匙串，也未执行任何Desktop E2E。尚未宣称gateway支持原生no-account兼容alias、ChatCompletions或Anthropic转换；这些需要上游转发契约实际实现后再扩展目录适配。

## 目录路径与配置联合修复

`prepare_item`只接受catalog_path_validity/catalog_integrity，不接Provider孤儿等其它操作。预先真实读取并验证来源、credentials/schema、builder/CLI signature以及UTF8/TOML语法；准备过程中没有写文件。二次来源读取必须和计划原始bytes一致，防止同一次预览混入两版mode/source。

Native普通模式用toml_edit移除**顶层**model_catalog_json；Native NoAccount和Atelier Gateway Responses写`catalog_root/codex_router_catalog.json`的实际绝对路径。profile表内同名字段、其它配置值、无关注释/格式保留。原config不存在且需要引用时创建具体TOML；不存在且普通Native无需引用时不凭空创建空文件。`PreparedAction`可Clone，暴露item_id/config_path/config_after只读内容及既有document/destination/revision；`validate`逐项核对来源、配置、目录、CLI签名快照。

`apply`在实际gateway跨进程锁或上层Native Manager gate内先CAS；再创建private maintenance-backups/catalog-{UUID}目录，逐份保存source原始bytes、旧catalog和config原始bytes，private0600原子写并同步、逐份读回完全相等；manifest为原来不存在的文件记录present=false/backup=null，不伪造空备份。备份后再次CAS再做任何原文件修改。

目录先实际原子发布并读回；写config前再核config/来源快照，原子发布config后也逐字节读回。配置失败时，仅在目录仍逐字节等于本次发布结果时恢复旧目录内容或旧缺失状态；若外部已改变目录，则保留外部变化并明确返回补偿未完成及原始备份位置。配置发布后发生外部变化导致读回失败时，不覆写外部config；同样显式Err并保留备份，不声称联合提交完成。正常Outcome含destination、directory_sync_error、backup_path、可选config_publication。目录同步warning沿用原生ack语义，rename已成功不误报未发布。

新增5项合成测试覆盖Native普通/NoAccount/Gateway三种实际path修改、profile内同名key与注释保留、source/catalog/config原始bytes私有备份与读回、错误TOML及configCAS先验阻断、缺config创建及absence manifest、Unix只读config父目录导致真实写失败后的目录字节补偿，以及显式Context路径选择和坏native source拒绝。

`context_source(ctx)`只使用ctx.home/app_root/config/runtime；原生state存在且启用优先Native；native关闭而实际Atelier配置或runtime启用则选Gateway；损坏/不可读native source直接Err。默认bundle=None，不执行任何发现。`context_source_with_bundle(ctx, bundle)`供Context::native入口把已发现路径显式传入，测试仅传合成路径。此模块从不调用process::discover或真实应用。

Gateway没有preferred profile时保持原生允许无active inventory的语义，不人为禁止构造空目录。diagnostics detail明确写“仅目录一致性，不代表整个路由已可生成”，并指出无preferred inventory的实际可用性由router_postcondition/Manager判断。新增合成测试确认空inventory leaf可为ok，但detail不宣称生成或路由验收完成。

原生Context目录root后续核正：verify_invariants_with_depth.c @100363040读取paths+0x248/+0x250的relay_root并join codex_router_catalog.json，故context_source的Native.catalog_root现为ctx.home/codexmate/relay，Gateway仍保持Atelier自己的ctx.home输出。显式Source::Native调用者所传catalog_root不会被适配器改写；合成显式Source fixtures可以自选目录。Context resolver测试断言原生发现路径。native_threads::input已使用同一relay目录作未配置catalog的默认值，with_catalog仅将该实际文件复制到scratch后供core读取，无需再改。
