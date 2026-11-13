/** 正式导航的唯一定义处。
 *
 * 侧栏与原生环形 HUD 共用这份顺序：Swift 侧 `AtelierHud.modules` 按数组下标把
 * 标签写进对应模块，点击后也只回传下标。所以这里的条目顺序必须与
 * `src-tauri/native-hud/AtelierHud.swift` 的 modules 数组逐项对应，
 * 任一侧增删都要同步，否则原生 HUD 会路由到错误的页面。
 * `tests/navigation.test.ts` 会校验两侧数量一致。
 */
export interface NavItem {
  path: string
  label: string
  icon: string
}

export const nav: NavItem[] = [
  { path: '/', label: '主工作台', icon: 'grid' },
  { path: '/accounts', label: '账号与额度', icon: 'user' },
  { path: '/connections', label: '模型与连接', icon: 'chip' },
  { path: '/providers', label: '供应商', icon: 'plug' },
  { path: '/routing', label: '路由', icon: 'arrow' },
  { path: '/routing-groups', label: '路由组', icon: 'grid' },
  { path: '/gateway', label: '网关', icon: 'terminal' },
  { path: '/plugins', label: '行为插件', icon: 'shield' },
  { path: '/threads', label: '线程管理', icon: 'history' },
  { path: '/desktop-pets', label: '桌宠', icon: 'heart' },
  { path: '/maintenance', label: '维护工具', icon: 'settings' },
]
