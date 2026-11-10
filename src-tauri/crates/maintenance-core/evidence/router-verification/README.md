# 原生目标态验证

只读 AiMaMi.arm64 SHA256 b2a4b1b2b4babd2518e02c0f0d6203ad19e779909a47b3b643844ef403fe95de。原始 C 位于 `/Users/Zhuanz/Documents/New project 5/relay-recovered/maintenance-full/codex-direct/decompiled`。实现`src/router_verification.rs`，测试`tests/router_verification_native.rs`直接import实际公共module。

## 具体分支

- `verify_invariants_with_depth.c @100363040`：实际读取config，NotFound视空、其它I/O/UTF8错误产生Config issue；用已恢复RouterConfigSurface检查目标模式。NativeOff检查takeover/legacy虚拟auth marker、实际LiveAuthState和unlock sidecars，Full检查openai provider收敛。RouterOn实际读取relay_root/codex_router_catalog.json，检查顶层relay模型、typed requires_openai_auth与文件推断mode反向一致、auth残留、文件存在/JSON/models/no-account命名空间。不是读取snapshot中的presence bool当成文件验证。
- 路径由显式VerificationPaths分开home/config_path/relay_root/databases。原生catalog根是CodexPaths+0x248/+0x250，即home/codexmate/relay。数据库清单由已有explicit source discovery提供，检查复用实际SQLite/rollout/core consistency模块；没有虚构DB结果。
- `InvariantDim_serialize @100362c54`实测0=config/1=auth/2=catalog/3=thread/4=proxy，保存dimensions.asm。
- native depth=1只跳过provider convergence；RouterOn的thread-model/catalog实际DB检查仍执行。Full=0。原生verify_thread_provider gate @100362e6c..e88只含db_mismatch/rollout_mismatch/missing_rollouts/errors，**不含corrupt-only**，但输出会打印unreadable_rollouts。实现独立保留此已确认差异，未用更宽的诊断severity替代；单独corrupt-only测试直接验证该原版限制。
- `is_relay_model_slug @100367e50`：第一处::之前必须以aimami_relay_开头且通过实际portable provider ID校验，后缀trim非空；不是任意provider::model都合法。槽位别名复用原生五个canonical allowlist。
- `verify_target_state_at_with_depth.c @10068704c`先收集上述issues；NativeOff直接返回，不检查proxy。RouterOn真实Mutex锁poison/missing handle各加Proxy issue。存在任意Config issue时跳过额外HTTP配置契约；其它维度issue不阻止它；端口测试始终执行。
- `verify_router_on_http_contract.c @1003642b8`读取实际config与catalog mode，读取mode失败时expected requires_openai_auth=true。`router_http_contract_issues.c @10078ca44`只做TOML字段验证：aimai1表、base_url去尾slash后须以/codex/router/v1结尾（不是一般URL解析），有期望时精确比对去尾slash字符串；wire_api=responses、requires_openai_auth typed bool及期望、supports_websockets=false。原生不检查env_key。解析/读取失败都返回具体Config issue；多字段问题按原顺序join为一项。
- HTTP命名helper本身没有网络请求。真正网络检查`proxy_handle_port_accepts @10068ef50`先读真实Arc AtomicBool，再连接127.0.0.1:handle.port；300ms timeout、失败后50ms pause、最多重试一次。LocalPortTransport具体用TcpStream执行，测试真实loopback listener及关闭端口。仅snapshot.proxy.running=true无法代替owned handle和端口连接。
- `has_confirmed_target_state.c @10068273c`先checked JournalStore::commit_pending，Error或true直接false；只有无pending且Full真实issues.empty才true。
- run_diagnostics_with_mode @10037e464的router_postcondition不同于Manager确认：committing时报告待提交，不跑invariants；否则depth=1加config_stale。原native诊断丢弃journal读错后继续leaf；本typed RouterPostcondition保留Unreadable(error)且不称其为完整satisfied，Manager确认仍按原checked guard fail closed。它不作proxy/TCP确认。

## 复用与边界

公开ExpectedConfig::native、ExpectedConfig::atelier_responses与config_contract_issues。Atelier flavor明确实际provider+/v1+Responses+false auth+ATELIER_GATEWAY_TOKEN；不伪造aimai1、不修改文件来满足原生契约。公开实际read_catalog_slugs/read_catalog_no_account_mode和PortTransport/LocalPortTransport/proxy_handle_port_accepts供Host组合真实检查。原生target verifier默认不接纳Gateway raw IDs或/v1；Host必须显式选择自己的契约和库存/DB规则。

RelayStateDocument是Manager真实snapshot输入，实际catalog mode和auth残留仍取文件。没有CallerValid bool，没有Unsupported伪成功。此验证确认已恢复的原生目标态维度，不代表真实上游生成、账户额度或完整Manager执行已经验收。

## 已执行验证

`cargo test --offline --manifest-path src-tauri/crates/maintenance-core/Cargo.toml --test router_verification_native`：11 passed，无警告。覆盖真实temp config/catalog/auth sidecar、SQLite/rollout、depth差异、no-account namespace/typed auth、实际TCP listener与关闭端口、真实poison mutex、精确重试参数、Config issue抑制HTTP字段check但不抑制端口、checked journal pending/坏JSON、postcondition与Manager确认差别、显式Atelier contract真实字段比较。

所有测试使用合成根、端口和密钥；没有读取真实Codex配置/会话/凭据，没有调用Keychain或运行原应用。未做原二进制动态差分；TOML错误文案由本地缓存toml版本生成，极少见语法跨版本等价仍是精确边界。
