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
  previewDirectoryProbe: vi.fn(),
  applyDirectoryProbe: vi.fn(),
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
async function openDialog() {
  await open()
  await button('运行诊断').trigger('click')
  await flushPromises()
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
  api.previewDirectoryProbe.mockResolvedValue({
    ticket: 'probe-ticket',
    itemIds: ['codex_home_writable'],
    title: '目录写入检查预览',
    changes: ['仅在 /demo/.codex 独占创建临时文件，写入并删除'],
    warnings: ['不修改配置、凭据或数据库'],
    requiresDesktopClosed: false,
  })
  api.applyDirectoryProbe.mockResolvedValue({
    fixedIds: [],
    failedIds: [],
    details: ['临时文件写入与清理完成'],
    backupPath: null,
    report: report(),
  })
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

describe('maintenance page', () => {
  it('loads the nine original rows without executing anything', async () => {
    await open()
    expect(api.snapshot).toHaveBeenCalledOnce()
    expect(api.preview).not.toHaveBeenCalled()
    expect(api.apply).not.toHaveBeenCalled()
    expect(api.runDiagnostics).not.toHaveBeenCalled()
    expect(wrapper!.findAll('.mnt-row')).toHaveLength(9)
    for (const t of [
      '运行诊断',
      '导出日志',
      '发送诊断',
      '开始清理',
      '开始重建',
      '强制关闭',
      '重启',
    ])
      expect(button(t).exists()).toBe(true)
    expect(wrapper!.get('[aria-label="Claude 联网搜索"]').attributes('disabled')).toBeDefined()
  })

  it('runs clean as a single click, using the preview ticket internally', async () => {
    await open()
    await button('开始清理').trigger('click')
    await flushPromises()
    expect(api.preview).toHaveBeenCalledWith('clean')
    expect(api.apply).toHaveBeenCalledWith('review-ticket')
    expect(wrapper!.text()).toContain('清理完成')
  })

  it('never executes when the preview fails and shows the cause', async () => {
    api.preview.mockRejectedValueOnce(Error('目录已变化，请重新预览'))
    await open()
    await button('开始重建').trigger('click')
    await flushPromises()
    expect(api.apply).not.toHaveBeenCalled()
    expect(wrapper!.text()).toContain('目录已变化')
  })

  it('force kill runs in one click like the original', async () => {
    api.preview.mockResolvedValueOnce({ ...plan, action: 'forceKill', title: '结束预览' })
    await open()
    await button('强制关闭').trigger('click')
    await flushPromises()
    expect(api.preview).toHaveBeenCalledWith('forceKill')
    expect(api.apply).toHaveBeenCalledWith('review-ticket')
  })

  it('restart asks for confirmation before touching the client', async () => {
    await open()
    await button('重启').trigger('click')
    await flushPromises()
    expect(api.preview).not.toHaveBeenCalled()
    await button('取消').trigger('click')
    expect(api.apply).not.toHaveBeenCalled()
    await button('重启').trigger('click')
    await flushPromises()
    await wrapper!.get('.mnt-confirm .mnt-primary-button').trigger('click')
    await flushPromises()
    expect(api.preview).toHaveBeenCalledWith('restart')
    expect(api.apply).toHaveBeenCalledWith('review-ticket')
  })

  it('swap the image switch in one step and keep the old state on failure', async () => {
    await open()
    const control = wrapper!.get('[aria-label="Image 声明过滤"]')
    expect(control.attributes('aria-checked')).toBe('false')
    await control.trigger('click')
    await flushPromises()
    expect(api.previewImageCompat).toHaveBeenCalledWith(true)
    expect(api.applyImageCompat).toHaveBeenCalledWith('image-ticket')
    expect(control.attributes('aria-checked')).toBe('true')
  })

  it('keeps the image switch off when apply fails', async () => {
    api.applyImageCompat.mockRejectedValueOnce(Error('配置已变化'))
    await open()
    await wrapper!.get('[aria-label="Image 声明过滤"]').trigger('click')
    await flushPromises()
    expect(wrapper!.get('[aria-label="Image 声明过滤"]').attributes('aria-checked')).toBe('false')
    expect(wrapper!.text()).toContain('配置已变化')
  })

  it('exports through the native save dialog and reports the real path', async () => {
    api.saveBundle = vi
      .fn()
      .mockResolvedValueOnce({ saved: false, path: null, summary: '已取消' })
      .mockResolvedValueOnce({ saved: true, path: '/demo/report.zip', summary: '已保存诊断包' })
    await open()
    await button('导出日志').trigger('click')
    await flushPromises()
    expect(api.exportBundle).not.toHaveBeenCalled()
    expect(wrapper!.text()).not.toContain('已保存诊断包')
    await button('导出日志').trigger('click')
    await flushPromises()
    expect(wrapper!.text()).toContain('/demo/report.zip')
  })

  it('sending requires an explicit destination and confirmation', async () => {
    await open()
    await button('发送诊断').trigger('click')
    await flushPromises()
    expect(button('确认发送').attributes('disabled')).toBeDefined()
    await wrapper!.get('input[type="url"]').setValue('https://support.example/diagnostics')
    await wrapper!.get('.mnt-check input').setValue(true)
    await button('确认发送').trigger('click')
    await flushPromises()
    expect(api.sendBundle).toHaveBeenCalledWith('https://support.example/diagnostics', '')
    expect(wrapper!.text()).toContain('demo-report')
  })
})

describe('system diagnostic dialog', () => {
  it('opens and automatically checks all 35 items in four groups', async () => {
    await openDialog()
    expect(api.runDiagnostics).toHaveBeenCalledOnce()
    expect(wrapper!.findAll('.mnt-item')).toHaveLength(35)
    expect(wrapper!.findAll('.mnt-group')).toHaveLength(4)
    expect(wrapper!.text()).toContain('全部修复（2 项）')
    expect(wrapper!.text()).toContain('配置')
    expect(wrapper!.text()).toContain('运行时与线程')
    expect(wrapper!.text()).toContain('环境与磁盘')
  })

  it('expands a row to show its detail', async () => {
    await openDialog()
    await wrapper!.get('.mnt-item-label').trigger('click')
    expect(wrapper!.find('.mnt-item-detail').exists()).toBe(true)
  })

  it('fixes a single item in one click', async () => {
    await openDialog()
    await wrapper!.get('.mnt-fix').trigger('click')
    await flushPromises()
    expect(api.previewRepair).toHaveBeenCalled()
    expect(api.applyRepair).toHaveBeenCalledWith('repair-ticket')
    expect(wrapper!.text()).toContain('已备份并修复 2 项')
  })

  it('routes the writable-directory item through the probe ticket', async () => {
    api.runDiagnostics.mockImplementation(async () => ({
      ...report(),
      items: report().items.map((i) =>
        i.id === 'codex_home_writable'
          ? { ...i, status: 'error' as const, fixable: true }
          : { ...i, fixable: false },
      ),
    }))
    await openDialog()
    await wrapper!.get('.mnt-fix').trigger('click')
    await flushPromises()
    expect(api.previewDirectoryProbe).toHaveBeenCalledOnce()
    expect(api.applyDirectoryProbe).toHaveBeenCalledWith('probe-ticket')
    expect(api.applyRepair).not.toHaveBeenCalled()
  })

  it('fixes everything with one click and refreshes the rows', async () => {
    await openDialog()
    await button('全部修复').trigger('click')
    await flushPromises()
    expect(api.previewRepair).toHaveBeenCalledWith('all')
    expect(api.applyRepair).toHaveBeenCalledWith('repair-ticket')
    expect(wrapper!.text()).toContain('已备份并修复 2 项')
    expect(wrapper!.findAll('.mnt-item')).toHaveLength(35)
  })

  it('shows the all-clear line instead of the list when nothing is wrong', async () => {
    api.runDiagnostics.mockImplementation(async () => ({
      ...report(),
      hasIssues: false,
      items: report().items.map((i) => ({ ...i, status: 'ok' as const, fixable: false })),
    }))
    await openDialog()
    expect(wrapper!.text()).toContain('恭喜您，目前诊断一切正常。')
    expect(wrapper!.findAll('.mnt-item')).toHaveLength(0)
    expect(wrapper!.findAll('button').some((b) => b.text().includes('全部修复'))).toBe(false)
  })

  it('wraps labels during a route transition and blocks fixing', async () => {
    api.runDiagnostics.mockImplementation(async () => ({
      ...report(),
      transitionInFlight: true,
    }))
    await openDialog()
    expect(wrapper!.text()).toContain('原始检测：')
    expect(wrapper!.findAll('.mnt-fix')).toHaveLength(0)
    expect(wrapper!.findAll('button').some((b) => b.text().includes('全部修复'))).toBe(false)
  })

  it('keeps the stale report visible when a recheck fails', async () => {
    await openDialog()
    api.runDiagnostics.mockRejectedValueOnce(Error('线程库正忙，请重试'))
    await button('重新检测').trigger('click')
    await flushPromises()
    expect(wrapper!.text()).toContain('线程库正忙')
    expect(wrapper!.findAll('.mnt-item')).toHaveLength(35)
  })

  it('shows the loading state until the first scan resolves', async () => {
    let resolve: (r: DiagnosticReport) => void = () => {}
    api.runDiagnostics.mockReturnValueOnce(new Promise<DiagnosticReport>((r) => (resolve = r)))
    await openDialog()
    expect(wrapper!.text()).toContain('正在检测…')
    expect(wrapper!.findAll('.mnt-item')).toHaveLength(0)
    resolve(report())
    await flushPromises()
    expect(wrapper!.findAll('.mnt-item')).toHaveLength(35)
  })

  it('does not let a stale scan overwrite a reopened dialog', async () => {
    let resolve: (r: DiagnosticReport) => void = () => {}
    api.runDiagnostics.mockReturnValueOnce(new Promise<DiagnosticReport>((r) => (resolve = r)))
    wrapper = mount(DiagnosticDialog, { attachTo: document.body })
    await flushPromises()
    wrapper.unmount()
    wrapper = mount(DiagnosticDialog, { attachTo: document.body })
    await flushPromises()
    resolve({ ...report(), items: [] })
    await flushPromises()
    expect(wrapper.findAll('.mnt-item')).toHaveLength(35)
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
