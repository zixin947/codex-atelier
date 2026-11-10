# 官方账号切换与直连

## 行为与范围

点击官方账号卡片的「切换账号」后，预览明确显示凭据、连接方式及配置改动；用户确认后才执行。添加、登录、导入和自动识别账号仅登记，不自动接管当前连接。

- 使用 Codex 内置 OpenAI Provider，不通过 Atelier 的模型转发网关。不因额度耗尽自动切换到中转。
- 撤销当前自定义 Provider 选择、顶层 `openai_base_url` 和本地受管模型目录引用；离开中转时清除中转模型与推理档位，交回官方默认选择，不猜官方模型 ID。
- 保留旧 Provider 定义、无关配置、项目 trust 与注释，不修改会话数据库，不保证旧会话热切换。
- 不停止 Gateway：它可能仍服务其他客户端，但本次官方账号配置不再选用它。
- 不改系统代理、VPN、环境变量或其他客户端的启动参数。「官方直连」在此特指绕过 Atelier 模型网关，不是绕过所有网络代理。

## 拒绝自动切换的情况

- 非 file 认证、命名 profile、强制其他登录方式、TOML 损坏/非UTF8。
- 存在 `[model_providers.openai]` 覆盖，不能仅凭名称断言使用官方内置连接。
- 存在非受管 `model_catalog_json`；不能确认其是否覆盖官方模型发现。
- 外部 Native 路由仍启用、残留认证接管资料、未完成或损坏的转换日志。
- `router_model_restore.json` / `router_no_account_model_restore.json`（Native 与 Atelier 两侧）仅作为只读依赖；不因存在而拦截、不解析或改写，预览提示原样保留。预览后这些文件变化仍使旧票据失效。
- Atelier 进程中的已知连接环境覆盖：`OPENAI_BASE_URL`、`CODEX_MODEL_PROVIDER`、`CODEX_PROFILE`。
- 预览后凭据、配置、路由意图、账号源、目录对象或相关路由资料变化。

未知或受限制的情况明确拒绝，不把仍指向本地地址的配置显示成已完成直连。不能观察其他进程所有环境/命令行和系统级管理配置，因此不宣称完成真实流量验证。

## 切换和回退

`accounts/direct.rs` 负责纯配置计划及显式路径依赖观察；`accounts/switch.rs` 生成5分钟票据，并复用维护模块的配置/线程写锁。执行时先保留私有原件，再分别发布 auth.json、config.toml 和 Atelier router-intent。意图明确为 NativeOff，operation ID 为空：这是账号的路由偏好，不伪造完整 Manager 健康回执。

多文件发布不是操作系统原子事务。每一步前重核获批状态，发布后读回；失败时仅恢复仍与本次预期效果一致的目标，冲突不覆盖。原件保留在账号管理目录下 `switch-<ticket>.json`；该文件包含凭据，只应保留在受保护本地目录，不应提交、上传或分享。

新版 last-switch 记录 version=2，覆盖全部三个目标，并以 completed 标记区分完整成功与未完成切换。未完成记录阻止下一次切换覆盖恢复材料。回退前核对当前三个目标仍等于上次效果；旧版记录仅恢复原先授权的 auth.json，不能借兼容迁移修改配置。

目录身份和父目录/符号链接检查降低被替换的风险，但多文件CAS与补偿不是跨进程OS级事务；最后核验与路径发布之间的瞬时竞态并非完全消除。其他应用仍需退出或停止写入该目录。

## Magpie 参考与差异

亲自读取的参考：

- https://github.com/yetone/magpie/blob/main/internal/agent/codex.go
- https://github.com/yetone/magpie/blob/main/internal/agent/codex_route_test.go
- https://github.com/yetone/magpie/blob/main/internal/agent/codex_ccswitch_official_test.go
- https://github.com/yetone/magpie/blob/main/internal/agent/codex_openai_auth_test.go

Magpie 根据模型选择撤回自己的地址覆盖，并保留 Provider 定义；已登录官方账号仍可能通过 Magpie 模型走本地网关，不能把登录状态等同于直连。Atelier 此次实现的是更明确的产品规则：确认切换官方账号即选官方直连，不复制 Magpie 的额度耗尽接管、线程迁移或整套实现。

## 2026-11-08 验证

仅临时目录、合成凭据和依赖替身；未读写真实 Codex 配置/凭据，不启动或停止客户端，不执行真实推理/登录。

- `cargo test --offline --manifest-path src-tauri/Cargo.toml accounts:: -- --test-threads=2`：15通过、0失败、1忽略（既有真实一分钟计时用例，本次未跑）。其中12项切换专项覆盖预览无写入、三个目标切换/回退、已直连/配置不存在、覆盖拒绝、外部变化、各发布阶段前/后故障注入、条件补偿冲突、目录身份、共享锁、symlink、旧版回退。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2`：452通过、0失败。与路由意图消费者及已有维护流程保持兼容。
- `pnpm test`：86通过；新增确认直连改动及取消不执行的界面用例。
- `pnpm build`：通过，包含TypeScript检查。
- `cargo build --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：最终加固后再次通过（9.14秒），75条警告；`cargo check`亦通过。账号回归在最后父目录/回退记录CAS加固后已重跑，仍15通过、1忽略。维护452项为此前检查点，最后修改仅涉及账号切换模块，未再次重跑维护套件。
- `git diff --check`、限定新增/修改文件格式检查：通过。

浏览器未完成官方切换弹窗视觉验收：普通浏览器账号服务不连接本机；尝试注入纯合成页面服务时浏览器运行时不支持动态模块导入（`importModule is not defined`），未继续绕过、也未改产品代码开启假原生模式。前端交互已由Vue组件测试验证，原生弹窗及真实Desktop仍待用户验收。

上述初次实现检查点没有提交、推送或打包替换运行应用；后续应用更新另行获用户授权，已有维护迁移的未提交改动原样保留。

## 模型恢复记录兼容修正（2026-11-08）

取消上述两类模型恢复文件的存在性拦截，继续有界只读观察及预览后变化检测；不解析、不删除、不覆盖恢复记录。其他实际路由/转换/认证接管检查保留。

- 最终 `cargo test --offline --manifest-path src-tauri/Cargo.toml accounts:: -- --test-threads=2`：25通过、0失败、1忽略。新增用例覆盖四处恢复记录存在时切换与回退均保留原字节、各文件新建/修改/删除使旧预览失效，以及损坏和进行中的转换日志仍拒绝。
- `pnpm test`：92通过。
- 限定Rust文件rustfmt检查、`git diff --check`：通过。
- 测试仅临时目录与合成凭据；未执行用户的真实账号切换。日志 `/tmp/atelier-restore-record-accounts-final.log`、`/tmp/atelier-restore-record-frontend.log`。
