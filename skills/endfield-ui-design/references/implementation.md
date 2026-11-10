# 实现约定

## 保留目标栈

本包的示例是独立 HTML/CSS，不要求迁移框架。Vue、React、原生桌面或现有组件库都可以借鉴语义与视觉。不要为了使用第三方 React 库把 Vue 产品重做。

## 令牌层次

建议使用：原始色 → 语义角色 → 组件特例。统一颜色、字体、间距、边框、半径、层级、动效时间和幅度。已有主题系统先复用其机制。

示例 `--ef-*` 是命名空间，不是必须的新项目架构。可按以下关系映射：

| 示例                             | 项目常见角色      |
| -------------------------------- | ----------------- |
| `--ef-canvas`                    | 应用背景          |
| `--ef-surface` / `--ef-raised`   | 面板 / 浮层       |
| `--ef-ink` / `--ef-muted`        | 正文 / 次要文字   |
| `--ef-signal` / `--ef-on-signal` | 主行动底色 / 前景 |
| `--ef-focus`                     | 键盘焦点          |
| `--ef-duration-*`                | 快慢动效          |

## 主题隔离

用限定属性，例如 `[data-ef-theme]`。不要用 `* { border-radius:0 !important }` 或覆盖所有 `button` 来套主题。组件样式、主题覆盖和业务状态分层，避免千禧、终末地等皮肤互相污染。

减少动效、强制色模式、系统字体放大也需要生效；不要通过更强选择器把它们覆盖掉。

## 组件与数据

业务状态由服务/适配器提供，组件只映射展示。`loading`、`unknown`、`error`、`stale`、`ready` 是不同状态，不从动画是否播放推断请求结果。

示例类型可参考：

```ts
type Reading =
  | { kind: 'unknown'; reason: string }
  | { kind: 'ready'; remaining: number; checkedAt: string; resetsAt?: string }
  | { kind: 'error'; message: string; previous?: Exclude<Reading, { kind: 'error' }> }
```

根据项目简化类型，避免为了样式建立不必要业务框架。未知数值不靠 `value || 0` 转成零；非有限数值不能进入弧长计算。

## Vue 落地

采用现有 Composition API/script setup 规范。局部点击反馈可用计数键，只重建装饰节点；不要借此重建整页。状态文字从实际数据派生，生命周期清理事件、计时器和观察器。

`RouterLink` 保留路由与键盘语义，点击装饰不拦截跳转。开关先执行真实或明确 Mock 的服务，再读回状态；视觉成功只跟随读回。

## SVG 与 CSS

圆环的数值与可访问标签一致，装饰部分不接受指针。裁切装饰内层，交互外层保留焦点。图标用当前颜色，不在各个组件写一组孤立的硬编码颜色。

## 预览与生产

[preview.html](../assets/preview.html) 仅在内存中切换演示状态，不连接账号或系统。复制示例到生产时替换数据适配器，保留演示标记直到真实服务被验证。不要把预览成功描述成后端成功。
