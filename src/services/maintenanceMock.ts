import { createDiagnosticDemo, demoZipBytes } from './maintenanceDemoData'
import type {
  MaintenanceAction,
  MaintenanceCheck,
  MaintenancePlan,
  MaintenanceResult,
  MaintenanceService,
  DiagnosticReport,
  DiagnosticRepairPlan,
} from '../types/maintenance'

type Operation =
  | 'snapshot'
  | 'diagnose'
  | 'preview'
  | 'apply'
  | 'runDiagnostics'
  | 'previewRepair'
  | 'applyRepair'
  | 'previewDirectoryProbe'
  | 'applyDirectoryProbe'
  | 'utilitySettings'
  | 'previewImageCompat'
  | 'applyImageCompat'
  | 'exportBundle'
  | 'sendBundle'
export interface MaintenanceMockOptions {
  accountCount?: number
  staleAccounts?: number
  backupCount?: number
  orphanProfiles?: number
  failures?: Partial<Record<Operation, string>>
}

// Browser-only state. No file, credential, process or network access.
export function createMaintenanceMock(options: MaintenanceMockOptions = {}): MaintenanceService {
  let accountCount = options.accountCount ?? 4
  let staleAccounts = Math.min(accountCount, options.staleAccounts ?? 1)
  let backupCount = options.backupCount ?? 3
  let orphanProfiles = options.orphanProfiles ?? 1
  let revision = 0
  let diagnosticRevision = 0
  let diagnosticItems = createDiagnosticDemo()
  const repairTickets = new Map<string, { plan: DiagnosticRepairPlan; revision: number }>()
  const probeTickets = new Set<string>()
  let imageCompat = false
  const imageTickets = new Map<string, { enabled: boolean; previous: boolean }>()
  let diagnosticEndpoint: string | null = null
  const report = (): DiagnosticReport => ({
    mode: 'demo',
    items: structuredClone(diagnosticItems),
    hasIssues: diagnosticItems.some(
      (i) => i.status === 'warning' || i.status === 'error',
    ),
    transitionInFlight: false,
    checkedAt: Date.now(),
    root: '/demo/.codex',
  })
  const tickets = new Map<string, { plan: MaintenancePlan; revision: number }>()
  function fail(operation: Operation) {
    const message = options.failures?.[operation]
    if (message) throw new Error(message)
  }
  function checks(): MaintenanceCheck[] {
    return [
      { label: '管理目录', status: 'pass', detail: '示例管理目录可读；演示环境不访问本机文件。' },
      {
        label: '账号注册表',
        status: staleAccounts ? 'warning' : 'pass',
        detail: staleAccounts
          ? `${staleAccounts} 条示例账号缺少对应档案。`
          : `${accountCount} 条示例账号记录一致。`,
      },
      {
        label: '可恢复档案',
        status: orphanProfiles ? 'warning' : 'pass',
        detail: orphanProfiles ? `${orphanProfiles} 个示例档案尚未登记。` : '示例档案均已登记。',
      },
      { label: 'Desktop', status: 'unknown', detail: '浏览器不会检测或控制本机 Desktop。' },
    ]
  }
  function result(action: MaintenanceAction, summary: string, changed = 0): MaintenanceResult {
    return {
      action,
      completedAt: Date.now(),
      summary,
      changed,
      warnings: ['演示结果：只更新当前页面的示例状态。'],
      checks: checks(),
      backupPath:
        action === 'clean' || action === 'rebuild' ? `/demo/maintenance/backup-${revision}` : null,
    }
  }
  return {
    async runDiagnostics() {
      fail('runDiagnostics')
      return report()
    },
    async previewRepair(itemId) {
      fail('previewRepair')
      const items = diagnosticItems.filter(
        (i) => i.fixable && i.status !== 'ok' && (itemId === 'all' || i.id === itemId),
      )
      if (!items.length) throw new Error('没有可修复项目，请重新检测。')
      const plan: DiagnosticRepairPlan = {
        ticket: crypto.randomUUID(),
        itemIds: items.map((i) => i.id),
        title: itemId === 'all' ? '全部修复预览' : '修复预览',
        changes: items.map((i) => `修复：${i.label}`),
        warnings: ['演示操作，不会修改本机配置或会话。'],
        requiresDesktopClosed: items.some((i) => i.group === 'runtime'),
      }
      repairTickets.set(plan.ticket, { plan, revision: diagnosticRevision })
      return structuredClone(plan)
    },
    async applyRepair(ticket) {
      fail('applyRepair')
      const pending = repairTickets.get(ticket)
      repairTickets.delete(ticket)
      if (!pending || pending.revision !== diagnosticRevision)
        throw new Error('预览已过期，请重新预览。')
      diagnosticRevision++
      diagnosticItems = diagnosticItems.map((i) =>
        pending.plan.itemIds.includes(i.id)
          ? { ...i, status: 'ok', fixable: false, detail: '演示修复已完成，复检通过。' }
          : i,
      )
      return {
        fixedIds: pending.plan.itemIds,
        failedIds: [],
        details: pending.plan.itemIds.map((id) => `演示修复完成：${id}`),
        backupPath: '/demo/maintenance/diagnostic-backup',
        report: report(),
      }
    },
    async previewDirectoryProbe() {
      fail('previewDirectoryProbe')
      const ticket = crypto.randomUUID()
      probeTickets.add(ticket)
      return {
        ticket,
        itemIds: ['codex_home_writable'],
        title: '目录写入检查预览',
        changes: ['演示：在已存在的 Codex 目录中独占创建小型临时文件，写入后删除'],
        warnings: ['仅演示，不创建真实文件；不修改配置、凭据或数据库，不创建缺失目录。'],
        requiresDesktopClosed: false,
      }
    },
    async applyDirectoryProbe(ticket) {
      fail('applyDirectoryProbe')
      if (!probeTickets.delete(ticket)) throw new Error('检查预览不存在或已执行，请重新预览。')
      return {
        fixedIds: [],
        failedIds: [],
        details: ['演示目录写入与清理检查完成，未访问本机文件。'],
        backupPath: null,
        report: report(),
      }
    },
    async utilitySettings() {
      fail('utilitySettings')
      return { imageCompat, diagnosticEndpoint }
    },
    async previewImageCompat(enabled) {
      fail('previewImageCompat')
      const ticket = crypto.randomUUID()
      imageTickets.set(ticket, { enabled, previous: imageCompat })
      return {
        ticket,
        enabled,
        changes: [
          enabled
            ? '演示：关闭 Codex 生图声明并过滤 Atelier 网关的 image_generation 工具'
            : '演示：移除生图声明限制并恢复网关透传',
        ],
        warnings: ['仅更新演示状态，不读取或修改本机配置。'],
      }
    },
    async applyImageCompat(ticket) {
      fail('applyImageCompat')
      const pending = imageTickets.get(ticket)
      imageTickets.delete(ticket)
      if (!pending || pending.previous !== imageCompat) throw new Error('配置已变化，请重新预览。')
      imageCompat = pending.enabled
      return { imageCompat, diagnosticEndpoint }
    },
    async exportBundle() {
      fail('exportBundle')
      return {
        fileName: 'atelier-diagnostics-demo.zip',
        bytes: demoZipBytes(),
        summary: '演示诊断包已生成，未读取本机文件。',
        included: ['带演示标识的说明文件'],
        excluded: ['真实配置', '凭据', '会话正文'],
      }
    },
    async sendBundle(endpoint, userNote) {
      fail('sendBundle')
      const url = new URL(endpoint)
      if (
        (url.protocol !== 'https:' &&
          !(
            url.protocol === 'http:' && ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname)
          )) ||
        url.username ||
        url.password ||
        url.hash ||
        url.search
      )
        throw new Error('请输入不含账号密码的 HTTPS 接收地址。')
      if (userNote.length > 2000) throw new Error('说明不能超过 2000 字。')
      diagnosticEndpoint = endpoint
      return { reportId: 'DEMO-' + Date.now(), message: '演示发送完成，没有发出网络请求。' }
    },
    async snapshot() {
      fail('snapshot')
      return {
        mode: 'demo',
        platform: 'macOS',
        arch: 'arm64',
        version: '0.1.0',
        accountCount,
        managedRoot: '/demo/Atelier/codex-accounts',
        desktopPath: null,
        checks: checks(),
      }
    },
    async diagnose() {
      fail('diagnose')
      return result('diagnose', '诊断完成，示例检查结果已更新。')
    },
    async preview(action) {
      fail('preview')
      if (action === 'diagnose') throw new Error('诊断无需修改预览。')
      const count =
        action === 'clean' ? backupCount + staleAccounts : action === 'rebuild' ? orphanProfiles : 1
      const plan: MaintenancePlan = {
        ticket: crypto.randomUUID(),
        action,
        title: {
          clean: '清理数据预览',
          rebuild: '重建账号注册表预览',
          restart: '重启 Desktop 预览',
          forceKill: '强制结束 Desktop 预览',
        }[action],
        count,
        changes:
          action === 'clean'
            ? [`清理 ${backupCount} 份示例旧备份`, `移除 ${staleAccounts} 条缺少档案的示例账号索引`]
            : action === 'rebuild'
              ? [
                  `扫描 ${accountCount - staleAccounts + orphanProfiles} 个示例账号档案`,
                  `登记 ${orphanProfiles} 个未登记档案，保留已有账号信息`,
                ]
              : action === 'forceKill'
                ? ['模拟强制结束已识别的 Desktop 进程，不重新启动']
                : ['模拟请求 Desktop 正常退出', '模拟重新启动并检查运行状态'],
        warnings: ['演示模式：不会修改本机文件或重启客户端。'],
      }
      tickets.set(plan.ticket, { plan, revision })
      return structuredClone(plan)
    },
    async apply(ticket) {
      fail('apply')
      const pending = tickets.get(ticket)
      if (!pending || pending.revision !== revision) throw new Error('预览已失效，请重新预览。')
      tickets.delete(ticket)
      revision++
      const { action, count } = pending.plan
      if (action === 'clean') {
        accountCount -= staleAccounts
        staleAccounts = 0
        backupCount = 0
      }
      if (action === 'rebuild') {
        accountCount += orphanProfiles
        orphanProfiles = 0
      }
      return result(
        action,
        {
          diagnose: '示例诊断完成。',
          clean: '清理完成，示例旧备份与失效索引已移除。',
          rebuild: '账号注册表已重建，已有名称和分组已保留。',
          restart: '模拟重启完成，未操作真实 Desktop。',
          forceKill: '模拟结束完成，未操作真实 Desktop。',
        }[action],
        action === 'rebuild' ? accountCount : count,
      )
    },
  }
}
