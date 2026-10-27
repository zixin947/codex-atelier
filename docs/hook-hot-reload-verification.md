# 当前会话 Hook 热停用可行性

日期：2026-10-27。结论：底层支持，无需新建会话；但必须向实际持有该会话的 app-server 发送热重载。该结论最初针对仅另起辅助进程的旧路径。现已实现可选的 Desktop 启动桥接与热重载开关，参见 [Desktop 桥接使用与验收](desktop-bridge.md)。你正在使用的真实 Desktop 尚未切换启动方式，客户端验收交由用户执行。

## 实际执行验证

使用与[前次验收](hook-execution-verification.md)相同的隔离环境、固定上游原始脚本及本地 Responses fixture。CLI `0.164.0` 与桌面内置引擎 `0.164.0-alpha.11.1` 结果一致。[执行证据](evidence/hook-hot-reload-20261027.json)记录六组对照、各事件完成状态及回合中途实验。

每组先开启插件，完成首回合的 4 次 Hook 正向对照，再停用并在同一个会话执行下一回合；每回合实际执行一次 `true`，退出码为 0。

| 停用方式                                                  | 后续回合 Hook 次数 | 结束会话 Hook 次数 |
| --------------------------------------------------------- | ------------------ | ------------------ |
| 辅助进程 `config/value/write` 关闭插件，即原 Atelier 路径 | 3                  | 1                  |
| 辅助进程 `config/batchWrite` 关闭插件并重载               | 3                  | 1                  |
| **所属宿主** `config/batchWrite` 关闭插件并重载           | **0**              | **0**              |
| 辅助进程写单条 Hook 的 `enabled=false`                    | 3                  | 1                  |
| **所属宿主** 写单条 Hook 的 `enabled=false` 并重载        | **0**              | **0**              |
| 所属宿主下一次 `turn/start` 指定 `disabledPluginIds`      | **0**              | **0**              |

每套引擎的六组测试共 12 个成功回合、12 次成功命令、36 次已完成 Hook。重点不是配置是否落盘，而是后续实际执行事件是否消失。

另外在同一个正在进行的回合中，fixture 在第一次真实命令完成后暂停，等所属宿主完成 `config/batchWrite` 与 `reloadUserConfig=true`，再返回第二次命令调用。两套引擎均得到：重载前 3 次 Hook；重载后第二次命令成功，Hook 为 0；结束会话 Hook 为 0。每套中途实验另有首回合 4 次 Hook 对照、总计 3 次成功命令。断言同时检查重载没有被阻塞到回合结束。

## 接口与接入边界

对应本机生成的协议类型，`ConfigBatchWriteParams` 包含 `expectedVersion` 和 `reloadUserConfig`；后者用于写入后把运行时配置热重载到已加载的会话。`ConfigValueWriteParams` 不包含该选项。静态模型、推理强度等配置不在该热重载保证之内。参考 [OpenAI app-server 文档](https://learn.chatgpt.com/docs/app-server)。

Atelier 需要先连接实际宿主、确认目标会话确实由它加载，再执行带版本检查的批量写入和重载。只对新辅助进程发送相同请求不起作用。即使另有 daemon 在运行，也不能据此认定它持有当前 Desktop 会话。

本次只读检查未找到已验证的 Desktop 宿主控制连接；对已有 daemon 控制 socket 的代理初始化未在限定时间内成功，也没有确认其加载了当前会话。没有启动新常驻服务、启用远程访问、向内部管道注入请求或改写上游已信任脚本。这说明已经运行的 stdio 宿主不能直接附加控制连接，后续实现采用首次启动时建立可共享的本地 socket；未改写现有客户端文件。

## 语义与未验证项

- “立即停用”在已测试条件下指重载完成后不再执行后续 Hook，不代表取消已经在运行的脚本；已注入上下文也不会撤回。
- 测试客户端是隔离 stdio app-server；桌面内置引擎通过不等于正在使用的 Desktop UI 端到端连接已经通过。
- 未创建子代理，没有逐项触发 Ponytail `SubagentStart`、STS 代理工具 `PostToolUse`；没有覆盖多宿主、多项目覆盖、插件重新开启或任意 Hook 执行中的竞争情况。
- 没有修改日常插件开关或信任记录，没有读取真实凭据、会话数据库或更改系统代理。

## 复现

前置条件同前次验收：macOS、Python 3、Git、Node、指定 Codex 引擎、Atelier 已下载的固定版本干净仓库。

```sh
python3 scripts/verify-hook-execution.py --hot-reload-probe \
  --output /tmp/atelier-hook-hot-reload.json
python3 scripts/verify-hook-execution.py --mid-turn-probe \
  --output /tmp/atelier-hook-mid-turn.json
```

两条命令均可用 `--codex` 指定桌面内置引擎，使用 `--source` 指定干净上游仓库目录。不带实验选项会复测当前开关的旧会话/新会话行为。任何断言失败均返回非零退出码。
