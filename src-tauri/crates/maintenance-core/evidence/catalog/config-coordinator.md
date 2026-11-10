# 已审阅的共享 config 组合器

实现：`src-tauri/src/maintenance/diagnostics/native_coordinator.rs`，测试：同目录`native_coordinator_tests.rs`。这是具体配置修复子流程；不冒充完整RelayManager或路由intent迁移。

## 真实组合

支持九项：auth_integrity、config_global_validity、config_profile_conflict、config_stale、router_http_contract、router_unlock_auth_residue、takeover_backup_orphan、catalog_path_validity、catalog_integrity。其它项明确拒绝，不用任意回调代表“支持”。

准备阶段复用`native_auth::prepare_sequence/planned_config`（底层core takeover/unlock真实纯计划）、`core::config::fix_config_global_validity_text/fix_config_profile_conflict_text`以及`native_catalog::prepare`实际构造器。

配置组合顺序：takeover恢复缺失原键/区段→global删除真实无效引用→profile删除legacy顶层profile→Atelier连接字段→catalog路径单键效果。最终只形成一份具体config after bytes，不从某份基于旧before的完整after覆盖另一项已恢复内容。

Atelier连接字段来自当前实际gateway::types::Catalog/settings.port，范围与Store读取契约一致；effective provider通过现有inspect::provider从组合后的配置选择，只修改已有Atelier所属Provider表：base_url=http://127.0.0.1:{port}/v1、wire_api=responses、env_key=ATELIER_GATEWAY_TOKEN、requires_openai_auth=false、supports_websockets=false。不借此接管自定义Provider，不创建缺失Provider表。native source仍用其原生普通/NoAccount规则，Atelier Gateway用实际静态目录路径。

## 依赖和阶段

- 初始union含config原始bytes stamp、auth全部已观察/将变更文件的length+SHA256、真实catalog来源/输出、CLI签名和core global实际查询的metadata结果；Atelier连接字段使用的gateway rawbytes同样进入union。单改gateway revision/空白也会CAS拒绝。
- 全union在首个修改前验证。prepare无写入。begin在父级codex-write.lock内创建private原始config备份、absence manifest并读回，再次验证。standalone apply自行取得相同维护锁。
- AuthIntegrity prefix按原生fix_all顺序最先执行，从实际accounts/backups恢复合格登录；保留keyring/ephemeral排除和真实route gate。新增候选路径（即便同bytes）在initial membership校验拒绝。Catalog/config stage会自动确保该prefix已经完成，避免先修改config后才执行原本首项Integrity。
- Catalog stage对同一真实来源fresh prepare，必须精确等于已批准document/revision/输出路径；实际发布内容并保留备份，不写config。ledger只更新到已批准输出stamp。
- Config stage从预审takeover效果重新组合真实core计划，fresh metadata/gateway输入必须属于ledger，final bytes必须精确等于预审最终bytes，才唯一发布config并读回。
- Auth stage仅调用具体`apply_after_approved_config`：config原dependency→唯一批准final的重基，其余依赖不动；fresh非config changes中每个保留项的after必须精确等于原批准after，允许原scope自然缩减，禁止新增target/观察依赖。fresh takeover即使再次提出恢复被global/profile删除的键，其config change也已被组合流程消费，不能重写回来；专用core apply仍执行fsync、quarantine验证、删除备份和实际残留复检。
- Auth ledger使用approved_effects的预审after stamp；绝不将执行后的任意实际文件值吸收为“批准结果”。

Session暴露auth_integrity_stage/catalog_stage/config_stage/auth_stage/postcheck。每阶段返回真实receipt或失败记录；stage-results.json逐步持久化。后续失败保留已完成目录/config/认证阶段、备份与failed stage，不把整个组合说成未执行，也不冒充全量回滚。

postcheck重新读取实际subset报告：core global/profile、catalog、auth与Atelier字段比较。router_http_contract这里只核字段，不发送真实HTTP请求。Outcome含diagnostics和requires_manager_reconcile；profile/stale/HTTP明确还需上层完整Manager重收敛，叶级通过不能取代该步骤。

## 已执行验证

`cargo test --offline --manifest-path src-tauri/Cargo.toml native_coordinator::tests --no-fail-fast`：12 passed，0 failed。coordinator无编译警告，其它并发模块有unused/dead_code警告。

使用真实临时文件验证：takeover恢复profile/空effort/外部路径后与global/profile/catalog组合且保留当前/恢复的无关字段；unlock+takeover同批实际认证恢复；Atelier stale/HTTP+catalog强类型组合；最初auth/config/metadata变化阻止全部写入；目录成功后外部config变化保留目录成功receipt且不删takeover备份；config成功后新增未批准备份阻止后续清理；gateway raw revision变化拒绝；启用中的真实mode不会被强制伪装Off。预览与结果不含key bytes。

## 上层接线边界

- 所有handled项交给同一Prepared；generic事务必须排除该config FileChange，否则仍会再次写旧after。
- 父级在任何thread/auth/config动作前调用validate_initial；已持维护锁时用begin_under_existing_gate，按实际stage顺序调用并最终postcheck。
- auth_integrity已纳入handles，generic旧auth_source/auth.json FileChange必须一起排除。只允许已审阅Integrity→Unlock对auth.json的这一处重叠；其它非config目标重叠仍明确拒绝。
- 完整Manager开关路由intent、动态HTTP验证、真实应用运行和用户数据迁移均未在此执行。所有测试只用合成目录和密钥，未调用真实Keychain、Context::native或Desktop。

## auth_integrity / unlock 原生执行次序证据

`fix_all @1003a69d8` 的原始ASM在`1003a6d54`先调用wrapper`10049f188`，该wrapper的`10049f1e0`直接调用`fix_auth_integrity @100373af8`；之后`1003a720c`调用wrapper`10049fee4`，其`10049ff3c`直接调用`fix_router_unlock_auth_residue @100393114`。三份独立短ASM已保存为`fix-all-integrity-before-unlock.asm`、`fix-integrity-wrapper.asm`、`fix-unlock-wrapper.asm`。输入`pseudocode/decomp-fix_all.c:259/411`同时有长度14/26的已知id匹配，但裁定顺序以直接call链为证。

因此AuthIntegrity与Unlock共享auth.json时必须先执行Integrity，再按真实新LiveAuthState重建Unlock；不能为了签名相同而把当前合格登录回写旧unlock备份。本轮已加入native_auth/coordinator并通过真实组合测试。

`native_auth::apply_after_approved_effects`接受仅由前序成功stage组成的before→批准after fingerprint prefix。配置另用原始/唯一final bytes重基；其余依赖只能保持initial或精确等于已批准prefix。fresh依赖路径必须属于原审阅集合，fresh target只能缩减；返回真实fresh effects，coordinator逐个核实际hash后才更新ledger。不会把原unlock的stale auth.json after(B)误录入已经由Integrity写成A的ledger。

新增四项coordinator测试：Missing auth.json + accounts/backups/A.json与不同unlock旧备份B，强制反序输入仍先Integrity，最终精确保留A、仅删B/marker且takeover/global/profile/catalog其余效果完成；Integrity后外部写入User C保留已完成prefix receipt且不覆盖C；初审后加入同bytes candidate也在任何原件写入前拒绝；keyring/ephemeral/真实route启用时不恢复文件认证。日志/预览无key bytes。
