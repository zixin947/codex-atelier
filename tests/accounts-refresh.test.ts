import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount, type VueWrapper } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import Accounts from '../src/features/accounts/Accounts.vue'
import { accountsService } from '../src/services/accounts'
import { useAccountRefresh } from '../src/app/accountRefresh'
import type { AccountAutoRefreshStatus, AccountsSnapshot } from '../src/types/accounts'

vi.mock('../src/services/accounts', () => ({
  accountsService: {
    snapshot: vi.fn(),
    detectCurrent: vi.fn(),
    refresh: vi.fn(),
    autoRefreshStatus: vi.fn(),
    setAutoRefreshInterval: vi.fn(),
  },
}))
const now = 1_800_000_000_000
let current: AccountsSnapshot
let nativeStatus: AccountAutoRefreshStatus
let wrapper: VueWrapper | undefined
beforeEach(() => {
  vi.useFakeTimers()
  vi.setSystemTime(now)
  vi.resetAllMocks()
  localStorage.removeItem('atelier.accounts.refreshInterval')
  vi.spyOn(document, 'visibilityState', 'get').mockReturnValue('visible')
  HTMLDialogElement.prototype.showModal = function () {
    this.setAttribute('open', '')
  }
  setActivePinia(createPinia())
  current = {
    available: true,
    reason: null,
    revision: 'fixture',
    lastSelectedId: null,
    rollbackAvailable: false,
    pendingLogin: null,
    currentAccount: {
      accountId: null,
      state: 'unknown',
      path: null,
      checkedAt: now,
      message: '离线测试',
    },
    accounts: [
      {
        id: 'fixture-account',
        name: '离线账号',
        email: 'fixture@example.invalid',
        plan: 'plus',
        group: '',
        note: '',
        status: 'ready',
        error: null,
        createdAt: now,
        metadata: {
          accountName: null,
          workspaceName: null,
          profileName: null,
          planSnapshot: 'plus',
          hasActiveSubscription: null,
          subscriptionExpiresAt: null,
          subscriptionWillRenew: null,
          isDelinquent: null,
          source: 'none',
          sourceAt: null,
          error: null,
        },
        quota: {
          primary: null,
          secondary: { usedPercent: 15, durationMins: 10080, resetsAt: now + 86_400_000 },
          checkedAt: now - 600_000,
          resetCredits: null,
        },
      },
    ],
  }
  vi.mocked(accountsService.snapshot).mockImplementation(async () => structuredClone(current))
  vi.mocked(accountsService.detectCurrent).mockImplementation(async () => structuredClone(current))
  nativeStatus = {
    version: 1,
    configured: true,
    intervalMs: 0,
    running: false,
    startedAt: null,
    nextRefreshAt: null,
    lastRun: null,
    error: null,
  }
  vi.mocked(accountsService.autoRefreshStatus).mockImplementation(async () =>
    structuredClone(nativeStatus),
  )
  vi.mocked(accountsService.setAutoRefreshInterval).mockImplementation(async (intervalMs) => {
    Object.assign(nativeStatus, { version: nativeStatus.version + 1, configured: true, intervalMs })
    return structuredClone(nativeStatus)
  })
})
afterEach(() => {
  wrapper?.unmount()
  wrapper = undefined
  vi.useRealTimers()
  vi.restoreAllMocks()
})
async function open() {
  wrapper = mount(Accounts)
  await flushPromises()
  await wrapper.get('[aria-label="账号自动刷新频率"]').setValue('60000')
  return wrapper
}
function finished(updated = 1, failed = 0, error: string | null = null) {
  nativeStatus = {
    ...nativeStatus,
    version: nativeStatus.version + 1,
    running: false,
    lastRun: { finishedAt: Date.now(), updated, failed, error },
  }
  useAccountRefresh().accept(structuredClone(nativeStatus))
}
describe('native account background refresh integration', () => {
  it('saves the frequency in the native controller and never starts a page quota timer', async () => {
    const page = await open()
    await flushPromises()
    expect(accountsService.setAutoRefreshInterval).toHaveBeenCalledWith(60000)
    expect(page.get('[aria-label="账号自动刷新频率"]').element.value).toBe('60000')
    await vi.advanceTimersByTimeAsync(181_000)
    expect(accountsService.refresh).not.toHaveBeenCalled()
    expect(page.get('.account-auto-refresh-note').text()).toContain(
      '切换页面、最小化或隐藏窗口仍会刷新',
    )
    vi.mocked(accountsService.setAutoRefreshInterval).mockRejectedValueOnce('保存失败')
    await page.get('[aria-label="账号自动刷新频率"]').setValue('30000')
    await flushPromises()
    expect(page.get('[aria-label="账号自动刷新频率"]').element.value).toBe('60000')
    expect(page.get('.account-auto-refresh-note').text()).toContain('保存失败')
  })
  it('keeps native settings across route unmounts and shows the result completed while hidden', async () => {
    const page = await open()
    await flushPromises()
    page.unmount()
    wrapper = undefined
    vi.spyOn(document, 'visibilityState', 'get').mockReturnValue('hidden')
    await vi.advanceTimersByTimeAsync(60_000)
    current.accounts[0]!.quota!.checkedAt = Date.now()
    current.accounts[0]!.quota!.secondary!.usedPercent = 42
    finished()
    wrapper = mount(Accounts)
    await flushPromises()
    expect(wrapper.get('[aria-label="账号自动刷新频率"]').element.value).toBe('60000')
    expect(accountsService.setAutoRefreshInterval).toHaveBeenCalledTimes(1)
    expect(wrapper.get('[role="progressbar"]').attributes('aria-valuenow')).toBe('58')
    expect(wrapper.find('[data-quota-state="stale"]').exists()).toBe(false)
    expect(wrapper.get('.account-auto-refresh-note').text()).toContain('1 个账号额度已更新')
  })
  it('migrates the legacy frequency once and keeps native settings authoritative', async () => {
    nativeStatus.configured = false
    localStorage.setItem('atelier.accounts.refreshInterval', '60000')
    const refresh = useAccountRefresh()
    await refresh.initialize()
    await refresh.initialize()
    expect(accountsService.setAutoRefreshInterval).toHaveBeenCalledExactlyOnceWith(60000)
    expect(localStorage.getItem('atelier.accounts.refreshInterval')).toBeNull()
    refresh.accept({ ...nativeStatus, version: nativeStatus.version - 1, intervalMs: 0 })
    expect(refresh.status?.intervalMs).toBe(60000)
  })
  it('shows the native running state and reads quota after failure without making up a new timestamp', async () => {
    const page = await open()
    await flushPromises()
    const refresh = useAccountRefresh()
    refresh.accept({ ...nativeStatus, version: ++nativeStatus.version, running: true })
    await flushPromises()
    expect(page.find('[data-quota-state="loading"]').exists()).toBe(true)
    current.accounts[0]!.status = 'error'
    current.accounts[0]!.error = '离线接口失败'
    finished(0, 1, '离线接口失败')
    await flushPromises()
    expect(page.get('.account-auto-refresh-note').text()).toContain(
      '0 个账号额度已更新，1 个失败：离线接口失败',
    )
    expect(current.accounts[0]!.quota!.checkedAt).toBe(now - 600_000)
    expect(page.find('[data-quota-state="failed"]').exists()).toBe(true)
  })
  it('marks reset expiry per window and clears the old label after native readback', async () => {
    const quota = current.accounts[0]!.quota!
    quota.checkedAt = now - 120_000
    quota.primary = { usedPercent: 50, durationMins: 300, resetsAt: now - 60_000 }
    const page = await open()
    expect(page.findAll('[data-quota-state="stale"]')).toHaveLength(1)
    expect(page.findAll('[data-quota-state="healthy"]')).toHaveLength(1)
    await vi.advanceTimersByTimeAsync(60_000)
    quota.checkedAt = Date.now()
    finished()
    await flushPromises()
    expect(page.findAll('[data-quota-state="stale"]')).toHaveLength(0)
  })
})
