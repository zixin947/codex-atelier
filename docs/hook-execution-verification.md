# Hook 实际执行验收

日期：2026-10-27，Asia/Shanghai。

结论：插件总开关关闭后，已经打开且加载过插件的会话仍会执行 Hook；关闭后新建的会话不再执行这些 Hook。CLI 和本机桌面内置引擎的隔离测试结果一致。该结果针对未桥接的旧开关路径；新版可选桥接见 [Desktop 使用与验收](desktop-bridge.md)。补充实测确认：底层支持由所属宿主热重载，详见 [热停用可行性验证](hook-hot-reload-verification.md)。

## 测试对象与证据

- Codex CLI `0.164.0`：[原始执行证据](evidence/hook-execution-cli-20261027.json)。
- 当前桌面应用内置 Codex 引擎 `0.164.0-alpha.11.1`：[原始执行证据](evidence/hook-execution-desktop-engine-20261027.json)。
- Ponytail `5.3.2`、STS `0.2.5`，使用 Atelier 下载的原始上游脚本；测试前核对固定提交、干净工作树和插件声明。证据文件记录提交。
- 依据是宿主的 `hook/completed` 通知，包含执行 ID、所属会话、事件、完成状态和耗时。没有用 `hooks/list` 的空清单代替执行验证。

## 操作与结果

先在临时 `CODEX_HOME` 安装并信任两个插件，建立开启对照。保持该会话和宿主进程打开，再由另一个短时 app-server 使用与 Atelier 相同的 `config/value/write`、版本检查和 `plugins."<id>".enabled=false` 写入路径关闭两者。没有调用运行时热重载。

每个测试回合由本地 Responses 测试服务返回固定工具调用，实际执行一次 `true` 并确认退出码 0，随后正常结束回合。没有使用真实模型、账号或 API Key。

| 场景                               | CLI                          | 桌面内置引擎                 |
| ---------------------------------- | ---------------------------- | ---------------------------- |
| 开启对照，首回合                   | 4 次 Hook 完成               | 4 次 Hook 完成               |
| 关闭后，同一个已打开会话继续一回合 | **3 次 Hook 仍完成**         | **3 次 Hook 仍完成**         |
| 关闭后，同一宿主新建会话           | **0 次**                     | **0 次**                     |
| 关闭后，新宿主进程新建会话         | **0 次**                     | **0 次**                     |
| 关闭后结束原先已打开会话           | STS `SessionEnd` 仍执行 1 次 | STS `SessionEnd` 仍执行 1 次 |

开启对照包括 Ponytail `SessionStart`、Ponytail `UserPromptSubmit`、STS `UserPromptSubmit`、STS `PreToolUse`。关闭后旧会话仍执行后面三项，且原会话结束时仍执行 STS `SessionEnd`。

每套引擎共完成 4 个回合、4 次真实无副作用命令和 8 次 Hook 回调。测试脚本有相应断言，失败以非零退出码结束。

## 范围与限制

- “已打开会话”是测试中先开启插件、完成正向对照后一直保持打开的会话，不是直接订阅本条日常聊天的实时执行流。
- 桌面测试运行了当前应用实际捆绑的引擎，但测试客户端是 stdio app-server；没有把另一个版本的系统 CLI 结果冒充桌面结果，也没有把桌面 UI 客户端端到端行为列为通过。
- 对本条日常桌面聊天，仅通过只读接口检查近期项目，未取得可直接订阅的 `hook/completed` 执行流；没有把“未见 hookPrompt”当作 Hook 未执行的证据。
- 未创建子代理，因此没有触发 Ponytail `SubagentStart` 或只匹配代理工具的 STS `PostToolUse`；没有声称全部事件逐项覆盖。
- 新会话测试没有项目级覆盖、额外来源副本或单独重启用插件的操作；这些情况仍需单独处理。
- 测试只在临时环境批准 Hook，未改变本机日常信任记录、账号、代理或现有聊天。最终复核本机两个受管插件仍停用。

## 复现

需要 macOS、Python 3、Node、Git、对应 Codex 可执行文件，以及 Atelier 已下载的干净固定版本仓库。脚本仅使用 Python 标准库；在临时目录运行，结束时回收宿主、本地响应服务与临时状态。

```sh
python3 scripts/verify-hook-execution.py \
  --output /tmp/atelier-hook-cli-evidence.json

python3 scripts/verify-hook-execution.py \
  --codex '/Applications/ChatGPT.app/Contents/Resources/codex-cli/CodexCLI.app/Contents/MacOS/codex' \
  --output /tmp/atelier-hook-desktop-engine-evidence.json
```

桌面引擎路径因安装方式而异；`--codex` 可指定实际路径，`--source` 可指定包含 `ponytail` 和 `stop-that-shit` 干净固定版本仓库的目录。

## 对用户的含义

关闭成功表示不让后续新会话加载该插件及其 Hook。需要停止正在使用这些 Hook 的工作时，应先结束原会话，再新建会话；刷新 Atelier 或仅关闭总开关，不能当作旧会话已经卸载 Hook。

![原生页面明确提示只影响新会话，不热停用旧会话](screenshots/behavior-hooks-follow-switch-native.png)

参考：[OpenAI app-server 执行事件](https://learn.chatgpt.com/docs/app-server)、[Hook 生命周期与信任](https://learn.chatgpt.com/docs/hooks)。
