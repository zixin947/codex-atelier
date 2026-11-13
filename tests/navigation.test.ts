import { expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { nav } from '../src/app/navigation'

/* 原生环形 HUD 只回传模块下标，标签也按下标写进 Swift 的 modules 数组。
   两侧数量或顺序一旦错位就会路由到错误页面，这里锁死二者的一致性。 */

const swiftSource = readFileSync(
  join(process.cwd(), 'src-tauri/native-hud/AtelierHud.swift'),
  'utf8',
)

function swiftModules(): string[] {
  const match = swiftSource.match(/let modules: \[HUDModule\] = \[([\s\S]*?)\]/)
  expect(match, '未找到 AtelierHud.swift 的 modules 数组').toBeTruthy()
  return [...match![1]!.matchAll(/\.([A-Za-z]+)/g)].map((entry) => entry[1]!)
}

it('keeps one native HUD module per sidebar destination', () => {
  expect(swiftModules()).toHaveLength(nav.length)
})

it('maps the documented module order onto the sidebar order', () => {
  // 6 个固定位（左上/底部）+ 右侧前 5 张卡，与 nav 的 11 项一一对应。
  expect(swiftModules()).toEqual([
    'system',
    'display',
    'hotkeys',
    'about',
    'storage',
    'activityMonitor',
    'notes',
    'fileShelf',
    'clipboard',
    'archive',
    'mediaAssembly',
  ])
  expect(nav.map((item) => item.path)).toEqual([
    '/',
    '/accounts',
    '/connections',
    '/providers',
    '/routing',
    '/routing-groups',
    '/gateway',
    '/plugins',
    '/threads',
    '/desktop-pets',
    '/maintenance',
  ])
})

it('no longer carries the demo extension, diagnostics or persona entries', () => {
  const paths = nav.map((item) => item.path)
  for (const removed of ['/extensions', '/diagnostics', '/planned/companions'])
    expect(paths).not.toContain(removed)
  const labels = nav.map((item) => item.label)
  for (const removed of ['扩展管理', '诊断与回滚', '人设与陪伴'])
    expect(labels).not.toContain(removed)
  // 真实维护与插件入口必须保留。
  expect(paths).toContain('/maintenance')
  expect(paths).toContain('/plugins')
})

it('keeps Shift+Tab stepping backwards by one module', () => {
  expect(swiftSource).toContain('modules.count - 1 : 1')
})

it('keeps the Rust bridge bounds aligned with navigation', () => {
  const rust = readFileSync(join(process.cwd(), 'src-tauri/src/hud_native.rs'), 'utf8')
  expect(rust).toContain(`labels.len() != ${nav.length}`)
  expect(rust).toContain(`0..${nav.length}`)
})
