import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount, type VueWrapper } from '@vue/test-utils'
import Maintenance from '../src/features/maintenance/Maintenance.vue'
import DiagnosticDialog from '../src/features/maintenance/DiagnosticDialog.vue'
import { createDiagnosticDemo, demoZipBytes } from '../src/services/maintenanceDemoData'
import { createMaintenanceMock } from '../src/services/maintenanceMock'
import type {
  DiagnosticReport,
  MaintenanceSnapshot,
  MaintenancePlan,
  MaintenanceResult,
} from '../src/types/maintenance'
const api = vi.hoisted(() => ({
  snapshot: vi.fn(),
  diagnose: vi.fn(),
  preview: vi.fn(),
  apply: vi.fn(),
  runDiagnostics: vi.fn(),
  previewRepair: vi.fn(),
  applyRepair: vi.fn(),
  utilitySettings: vi.fn(),
  previewImageCompat: vi.fn(),
  applyImageCompat: vi.fn(),
  exportBundle: vi.fn(),
  saveBundle: undefined as undefined | ReturnType<typeof vi.fn>,
  sendBundle: vi.fn(),
}))
vi.mock('../src/services/maintenance', () => ({ maintenanceService: api }))
const snapshot: MaintenanceSnapshot = {
  mode: 'demo',
  platform: 'macos',
  arch: 'aarch64',
  version: '0.1.0',
  accountCount: 3,
  managedRoot: '/demo/atelier/codex-accounts',
  desktopPath: '/demo/Codex.app',
  checks: [],
}
const plan: MaintenancePlan = {
  ticket: 'review-ticket',
  action: 'clean',
  title: '清理预览',
  changes: ['移除 1 条孤立账号记录'],
  warnings: ['仅作用于演示数据'],
  count: 1,
}
const result: MaintenanceResult = {
  action: 'clean',
  completedAt: 1700000000000,
  summary: '清理完成',
  changed: 1,
  warnings: [],
  checks: [],
  backupPath: '/demo/backup.json',
}
const report = (): DiagnosticReport => ({
  mode: 'demo',
  items: createDiagnosticDemo(),
  hasIssues: true,
  transitionInFlight: false,
  checkedAt: 1700000000000,
  root: '/demo/.codex',
})
let wrapper: VueWrapper | undefined
function button(text: string) {
  const b = wrapper!.findAll('button').find((b) => b.text().includes(text))
  expect(b, `missing button ${text}`).toBeTruthy()
  return b!
}
async function open() {
  wrapper = mount(Maintenance, { attachTo: document.body })
  await flushPromises()
  return wrapper
}
beforeEach(() => {
  vi.clearAllMocks()
  api.saveBundle = undefined
  api.snapshot.mockResolvedValue(structuredClone(snapshot))
  api.utilitySettings.mockResolvedValue({ imageCompat: false, diagnosticEndpoint: null })
  api.preview.mockResolvedValue(structuredClone(plan))
  api.apply.mockResolvedValue(structuredClone(result))
  api.runDiagnostics.mockImplementation(async () => report())
  api.previewRepair.mockResolvedValue({
    ticket: 'repair-ticket',
    itemIds: ['config_profile_conflict', 'thread_provider_consistency'],
    title: '全部修复预览',
    changes: ['移除旧的 profile 引用', '对齐 2 个线程的 Provider'],
    warnings: ['演示操作'],
    requiresDesktopClosed: true,
  })
  api.applyRepair.mockImplementation(async () => ({
    fixedIds: ['config_profile_conflict', 'thread_provider_consistency'],
    failedIds: [],
    details: ['已备份并修复 2 项'],
    backupPath: '/demo/repair-backup',
    report: {
      ...report(),
      items: report().items.map((i) => (i.fixable ? { ...i, status: 'ok', fixable: false } : i)),
    },
  }))
  api.previewImageCompat.mockResolvedValue({
    ticket: 'image-ticket',
    enabled: true,
    changes: ['关闭生图声明'],
    warnings: [],
  })
  api.applyImageCompat.mockResolvedValue({ imageCompat: true, diagnosticEndpoint: null })
  api.exportBundle.mockResolvedValue({
    fileName: 'demo.zip',
    bytes: demoZipBytes(),
    summary: '演示诊断包',
    included: ['系统版本'],
    excluded: ['凭据', '会话正文'],
  })
  api.sendBundle.mockResolvedValue({ reportId: 'demo-report', message: '演示发送完成' })
  HTMLDialogElement.prototype.showModal = function () {
    this.setAttribute('open', '')
  }
  HTMLDialogElement.prototype.close = function () {
    this.removeAttribute('open')
  }
})
afterEach(() => {
  wrapper?.unmount()
  document.body.innerHTML = ''
  wrapper = undefined
})

describe('maintenance menu and confirmation', () => {
  it('loads overview without executing diagnostics or maintenance', async () => {
    await open()
    expect(api.snapshot).toHaveBeenCalledOnce()
    expect(api.preview).not.toHaveBeenCalled()
    expect(api.apply).not.toHaveBeenCalled()
    expect(api.runDiagnostics).not.toHaveBeenCalled()
    expect(wrapper!.findAll('.maintenance-action')).toHaveLength(8)
    for (const t of [
      '开始诊断',
      '生成诊断包',
      '填写并发送',
      '预览清理',
      '预览重建',
      '预览结束',
      '预览重启',
    ])
      expect(button(t).exists()).toBe(true)
    expect(wrapper!.get('[aria-label="Claude 联网兼容"]').attributes('disabled')).toBeDefined()
  })
  it('cancelled preview never executes', async () => {
    await open()
    await button('预览清理').trigger('click')
    await flushPromises()
    expect(api.preview).toHaveBeenCalledWith('clean')
    expect(wrapper!.text()).toContain('移除 1 条孤立账号记录')
    await button('取消').trigger('click')
    expect(api.apply).not.toHaveBeenCalled()
  })
  it('executes only the reviewed ticket after explicit confirmation', async () => {
    await open()
    await button('预览清理').trigger('click')
    await flushPromises()
    expect(button('确认并执行').attributes('disabled')).toBeDefined()
    await wrapper!.get('dialog input[type="checkbox"]').setValue(true)
    await button('确认并执行').trigger('click')
    await flushPromises()
    expect(api.apply).toHaveBeenCalledWith('review-ticket')
    expect(wrapper!.text()).toContain('清理完成')
  })
  it('failed preview displays cause and never executes', async () => {
    api.preview.mockRejectedValueOnce(Error('目录已变化，请重新预览'))
    await open()
    await button('预览重建').trigger('click')
    await flushPromises()
    expect(api.apply).not.toHaveBeenCalled()
    expect(wrapper!.text()).toContain('目录已变化')
  })
  it('force kill uses its own preview and cancellation is harmless', async () => {
    api.preview.mockResolvedValueOnce({ ...plan, action: 'forceKill', title: '结束预览' })
    await open()
    await button('预览结束').trigger('click')
    await flushPromises()
    expect(api.preview).toHaveBeenCalledWith('forceKill')
    expect(wrapper!.text()).toContain('强制结束上述 Desktop 进程')
    await button('取消').trigger('click')
    expect(api.apply).not.toHaveBeenCalled()
  })
  it('image switch remains unchanged until reviewed apply succeeds', async () => {
    await open()
    const control = wrapper!.get('[aria-label="Image 声明过滤"]')
    await control.trigger('click')
    await flushPromises()
    expect(api.previewImageCompat).toHaveBeenCalledWith(true)
    expect(control.attributes('aria-checked')).toBe('false')
    await wrapper!.get('dialog input[type="checkbox"]').setValue(true)
    await button('确认并执行').trigger('click')
    await flushPromises()
    expect(api.applyImageCompat).toHaveBeenCalledWith('image-ticket')
    expect(control.attributes('aria-checked')).toBe('true')
  })
  it('image apply failure preserves old switch and blocks ticket reuse', async () => {
    api.applyImageCompat.mockRejectedValueOnce(Error('配置已变化'))
    await open()
    await wrapper!.get('[aria-label="Image 声明过滤"]').trigger('click')
    await flushPromises()
    await wrapper!.get('dialog input[type="checkbox"]').setValue(true)
    await button('确认并执行').trigger('click')
    await flushPromises()
    expect(wrapper!.get('[aria-label="Image 声明过滤"]').attributes('aria-checked')).toBe('false')
    expect(wrapper!.text()).toContain('配置已变化')
    expect(button('确认并执行').attributes('disabled')).toBeDefined()
  })
  it('export shows actual included/excluded contents before download', async () => {
    await open()
    await button('生成诊断包').trigger('click')
    await flushPromises()
    expect(api.exportBundle).toHaveBeenCalledOnce()
    expect(wrapper!.text()).toContain('demo.zip')
    expect(wrapper!.text()).toContain('会话正文')
    expect(button('保存诊断包').exists()).toBe(true)
  })
  it('native export reports success only after a saved file is returned', async () => {
    api.saveBundle = vi
      .fn()
      .mockResolvedValueOnce({ saved: false, path: null, summary: '已取消' })
      .mockResolvedValueOnce({ saved: true, path: '/demo/report.zip', summary: '已保存诊断包' })
    await open()
    await button('生成诊断包').trigger('click')
    await flushPromises()
    expect(api.exportBundle).not.toHaveBeenCalled()
    expect(wrapper!.text()).not.toContain('已保存诊断包')
    await button('生成诊断包').trigger('click')
    await flushPromises()
    expect(wrapper!.text()).toContain('已保存诊断包')
    expect(wrapper!.text()).toContain('/demo/report.zip')
  })
  it('sending requires an explicit destination and confirmation', async () => {
    await open()
    await button('填写并发送').trigger('click')
    await flushPromises()
    expect(button('演示发送').attributes('disabled')).toBeDefined()
    await wrapper!
      .get('#diagnostic-endpoint')
      .setValue('https://support.example/diagnostics?token=secret')
    await wrapper!.get('dialog input[type="checkbox"]').setValue(true)
    expect(button('演示发送').attributes('disabled')).toBeDefined()
    await wrapper!.get('#diagnostic-endpoint').setValue('https://support.example/diagnostics')
    await wrapper!.get('#diagnostic-note').setValue('测试说明')
    await wrapper!.get('form').trigger('submit')
    await flushPromises()
    expect(api.sendBundle).toHaveBeenCalledWith('https://support.example/diagnostics', '测试说明')
    expect(wrapper!.text()).toContain('demo-report')
  })
})

describe('system diagnostic dialog', () => {
  it('opens and automatically checks all 35 items in four groups', async () => {
    await open()
    await button('开始诊断').trigger('click')
    await flushPromises()
    expect(api.runDiagnostics).toHaveBeenCalledOnce()
    expect(wrapper!.findAll('.diagnostic-item')).toHaveLength(35)
    expect(wrapper!.findAll('.diagnostic-group')).toHaveLength(4)
    expect(button('全部修复（2 项）').exists()).toBe(true)
    expect(wrapper!.text()).toContain('未确认')
    await wrapper!.get('.diagnostic-item-label').trigger('click')
    expect(wrapper!.find('.diagnostic-detail').exists()).toBe(true)
  })
  it('serializes actions during detection and supports closing the loading dialog', async () => {
    let resolve: (r: DiagnosticReport) => void = () => {}
    api.runDiagnostics.mockReturnValueOnce(new Promise<DiagnosticReport>((r) => (resolve = r)))
    await open()
    await button('开始诊断').trigger('click')
    await flushPromises()
    expect(button('预览清理').attributes('disabled')).toBeDefined()
    await wrapper!.get('dialog [aria-label="关闭弹窗"]').trigger('click')
    await flushPromises()
    resolve(report())
    await flushPromises()
    expect(wrapper!.find('dialog').exists()).toBe(false)
    expect(wrapper!.find('.maintenance-latest-scan').exists()).toBe(false)
  })
  it('requests the selected issue ID instead of all', async () => {
    await open()
    await button('开始诊断').trigger('click')
    await flushPromises()
    await wrapper!.get('[aria-label="修复 顶层 profile 冲突"]').trigger('click')
    await flushPromises()
    expect(api.previewRepair).toHaveBeenCalledWith('config_profile_conflict')
    expect(api.applyRepair).not.toHaveBeenCalled()
  })
  it('moves keyboard focus into preview, back to its source, then to the repair result', async () => {
    await open()
    await button('开始诊断').trigger('click')
    await flushPromises()
    const sourceSelector = '[data-repair-id="config_profile_conflict"]'
    const source = wrapper!.get(sourceSelector)
    ;(source.element as HTMLButtonElement).focus()
    expect(document.activeElement).toBe(source.element)
    await source.trigger('click')
    await flushPromises()
    expect(document.activeElement).toBe(wrapper!.get('.diagnostic-intro h2').element)
    await button('返回检查结果').trigger('click')
    await flushPromises()
    expect(document.activeElement).toBe(wrapper!.get(sourceSelector).element)
    await wrapper!.get(sourceSelector).trigger('click')
    await flushPromises()
    await wrapper!.get('dialog input[type="checkbox"]').setValue(true)
    ;(button('确认修复').element as HTMLButtonElement).focus()
    await button('确认修复').trigger('click')
    await flushPromises()
    expect(document.activeElement).toBe(wrapper!.get('.diagnostic-fix-log h3').element)
    expect(wrapper!.get('.diagnostic-fix-log').text()).toContain('已备份并修复 2 项')
  })
  it('bulk repair previews first, then uses returned fresh report and logs', async () => {
    await open()
    await button('开始诊断').trigger('click')
    await flushPromises()
    await button('全部修复').trigger('click')
    await flushPromises()
    expect(api.previewRepair).toHaveBeenCalledWith('all')
    expect(wrapper!.text()).toContain('需要 Desktop 已退出')
    await wrapper!.get('dialog input[type="checkbox"]').setValue(true)
    await button('确认修复').trigger('click')
    await flushPromises()
    expect(api.applyRepair).toHaveBeenCalledWith('repair-ticket')
    expect(wrapper!.text()).toContain('已备份并修复 2 项')
    expect(wrapper!.findAll('.diagnostic-fix-one')).toHaveLength(0)
    expect(wrapper!.text()).toContain('/demo/repair-backup')
  })
  it('keeps stale report visible with error after recheck failure', async () => {
    await open()
    await button('开始诊断').trigger('click')
    await flushPromises()
    api.runDiagnostics.mockRejectedValueOnce(Error('线程库正忙，请重试'))
    await button('重新检测').trigger('click')
    await flushPromises()
    expect(wrapper!.text()).toContain('线程库正忙')
    expect(wrapper!.findAll('.diagnostic-item')).toHaveLength(35)
  })
  it('does not let old loading result overwrite a reopened dialog', async () => {
    let resolve: (r: DiagnosticReport) => void = () => {}
    api.runDiagnostics.mockReturnValueOnce(new Promise<DiagnosticReport>((r) => (resolve = r)))
    wrapper = mount(DiagnosticDialog, { attachTo: document.body })
    await flushPromises()
    wrapper.unmount()
    wrapper = mount(DiagnosticDialog, { attachTo: document.body })
    await flushPromises()
    resolve({ ...report(), items: [] })
    await flushPromises()
    expect(wrapper.findAll('.diagnostic-item')).toHaveLength(35)
  })
})

describe('browser maintenance adapter', () => {
  it('repairs demo state and rejects a consumed ticket', async () => {
    const mock = createMaintenanceMock()
    expect((await mock.runDiagnostics()).items).toHaveLength(35)
    const preview = await mock.previewRepair('all')
    const result = await mock.applyRepair(preview.ticket)
    expect(result.fixedIds).toHaveLength(2)
    expect(result.report.items.filter((i) => i.fixable)).toHaveLength(0)
    await expect(mock.applyRepair(preview.ticket)).rejects.toThrow('过期')
  })
  it('exports a real zip payload containing only demonstration text', async () => {
    const bundle = await createMaintenanceMock().exportBundle()
    expect(bundle.bytes.slice(0, 4)).toEqual([0x50, 0x4b, 3, 4])
    expect(new TextDecoder().decode(new Uint8Array(bundle.bytes))).toContain('DEMO ONLY')
  })
})
