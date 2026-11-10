# 订阅 API 网络配置与部分成功

## 作用范围

本设置仅控制 Atelier 自己发出的订阅元数据请求（固定官方 HTTPS 端点 `chatgpt.com/backend-api/subscriptions`），不修改系统代理、Codex配置、官方账号直连、模型网关转发、OAuth登录或账号额度RPC。

账号页显示分开的“额度最近更新”和“订阅 API 尚未验证/最近正常/查询失败”；点击订阅状态打开网络配置。成功状态仅来自实际订阅响应解析及账号校验，不由额度成功或无令牌测试制造。状态在进程内保存，重启后重新显示未验证；旧订阅缓存仍可显示来源时间，不伪更新。

## 模式

- **跟随系统（默认）**：先核对已有 HTTPS/ALL 代理环境变量，再读取macOS `scutil --proxy` 的HTTPS/SOCKS配置与例外列表；显式关闭reqwest隐式代理后按解析结果构造本订阅客户端。不借启用全局system-proxy特性改变其他HTTP客户端。
- **直连**：客户端明确禁用代理，不代表绕过VPN、TUN、透明代理或其他网络层。
- **手动代理**：HTTP、HTTPS、SOCKS5、SOCKS5H。SOCKS5H用于代理端DNS；地址不接受账号密码、路径、query、fragment或控制字符。设置保存在Atelier自有账号目录的私有 `api-network.json`，具有版本及随机代际/CAS修订。

系统检测读取已有配置并填入草稿，不扫描端口、不自动保存、不自动发连接测试。系统HTTP-only候选可供用户手选，但不会在跟随系统模式中擅自把它用于HTTPS。PAC/自动发现不下载、不执行，跟随系统发现PAC会明确要求手动模式。Windows/Linux尚未实现系统代理读取，仅环境代理和手动模式可用，不宣称全平台系统设置支持。系统例外列表映射到客户端NoProxy，完整PAC脚本、复杂系统通配模式和排除简单主机等策略不等同于原生网络栈，需按实际环境手工核对。

## 测试与实际订阅通信不同

“测试”仅由用户主动点击触发，访问固定官方端点，不带账号令牌、账号ID或认证请求头，不接受任意测试URL。

- HTTP 200：收到HTTP响应，不证明账号/订阅有效。
- HTTP 401/403等：网络到达HTTP响应阶段，但鉴权/访问未通过；同样不表示订阅读取成功。
- 超时/连接失败：保留结构化原因，不输出原始URL、代理密码或令牌；不能仅凭connect错误硬判DNS或TLS根因。

客户端连接超时6秒、整体15秒、禁止重定向；订阅响应沿已有256KiB限制读取。测试不保存设置、不更新“订阅API正常”；保存不发网络请求。关闭弹窗/修改草稿后旧测试回包不能覆盖新草稿。

设置修订改变时清除旧通信状态；旧在途订阅结果不采纳。相同设置重复保存也产生新代际，避免旧请求跨保存续用。不同账号并发请求按启动序列保护最新全局观察，不让旧成功覆盖新失败；各账号资料仍按其查询结果独立保存。

## 部分成功

额度RPC和订阅HTTP使用不同链路。刷新先保存成功读取的新额度；若订阅失败，保留旧订阅资料及其来源时间，并返回 `ok=true, partial=true` 的结构化结果。结果弹窗分别统计成功/部分成功/失败，不再把“额度已更新，订阅失败”算作整体失败。

后台定时刷新仍仅请求额度，不暗增订阅HTTP请求；额度时间变新不代表订阅接口本轮成功。代理保存后需主动刷新账号资料才会产生新的订阅通信观察。

## 验证（2026-11-08）

均由主agent亲自执行，未使用子智能体，未读取真实凭据发请求或更改系统网络设置。

- `cargo test --offline --manifest-path src-tauri/Cargo.toml accounts:: -- --test-threads=2`：最终 **22通过、1忽略、0失败**。忽略项为既有真实一分钟定时测试。新增网络7项测试包含URL/凭据拒绝、系统配置解析、读写与CAS、同设置新代际、旧在途/旧序列结果拒绝、无令牌HTTP状态、显式代理和NoProxy、超时脱敏、部分成功矩阵。日志 `/tmp/atelier-api-accounts-final.log`。
- `cargo test --offline --manifest-path src-tauri/Cargo.toml maintenance:: -- --test-threads=2`：**452通过**；`gateway::`：**5通过**。为最后请求序列加固前的兼容检查点，不冒充最后所有文件变更后再次运行。日志 `/tmp/atelier-api-maintenance.log`、`/tmp/atelier-api-gateway.log`。
- `pnpm test`：最终 **92通过**，含新增代理弹窗5项及部分成功UI用例。
- `pnpm build`：通过，包含TypeScript检查。
- `cargo check --offline --manifest-path src-tauri/Cargo.toml --bin codex-atelier`：最终通过，75条既有/未清理警告。日志 `/tmp/atelier-api-check-final.log`；本轮未做release打包。
- `git diff --check`、本轮Rust文件rustfmt check、前端限定文件Prettier check：通过。
- 浏览器用临时Vite合成页面显示实际Vue弹窗，未读本机账号/代理/发外网请求；核对无令牌测试说明和HTTP403非订阅成功的区分。1280×720截图发现select文字截断，修正height/padding后复看通过。未声称原生Desktop实测。

未验证真实订阅连通性、真实HTTP/SOCKS代理、系统PAC或Windows系统代理；没有使用用户账号测试，没有替换运行应用、提交或推送。网络设置只影响订阅链路，不表示所有账号请求都被此代理接管。

参考AiMaMi公开 `src/components/runtime/api-proxy-dialog.tsx`、`src-tauri/src/platform/proxy.rs`、`core/repository.rs`、`core/models.rs`。其api_client/account_metadata主体未在公开树中，未冒称复制该未公开实现。
