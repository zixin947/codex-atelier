import { computed, onScopeDispose, ref } from 'vue'
import { defineStore } from 'pinia'
import { isTauri } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { accountsService } from '../services/accounts'
import type { AccountAutoRefreshStatus } from '../types/accounts'

export const useAccountRefresh = defineStore('account-refresh', () => {
  const status = ref<AccountAutoRefreshStatus | null>(null)
  const ready = ref(false),
    saving = ref(false),
    error = ref('')
  let initializing: Promise<void> | undefined
  let unlisten: UnlistenFn | undefined
  function accept(value: AccountAutoRefreshStatus) {
    if (!status.value || value.version >= status.value.version) status.value = value
  }
  async function sync() {
    try {
      accept(await accountsService.autoRefreshStatus())
      error.value = ''
    } catch (e) {
      error.value = String(e)
    }
  }
  function initialize() {
    if (initializing) return initializing
    initializing = (async () => {
      try {
        if (isTauri())
          unlisten = await listen<AccountAutoRefreshStatus>('accounts-auto-refresh', (event) =>
            accept(event.payload),
          )
        accept(await accountsService.autoRefreshStatus())
        // One-time migration of the previous WebView preference. The backend
        // persists the choice and starts independently on later app launches.
        if (!status.value?.configured) {
          const previous = localStorage.getItem('atelier.accounts.refreshInterval')
          if (
            previous !== null &&
            [0, 30_000, 60_000, 180_000, 300_000].includes(Number(previous))
          ) {
            accept(await accountsService.setAutoRefreshInterval(Number(previous)))
            localStorage.removeItem('atelier.accounts.refreshInterval')
          }
        }
      } catch (e) {
        error.value = String(e)
      } finally {
        ready.value = true
      }
    })()
    return initializing
  }
  async function setInterval(intervalMs: number) {
    if (saving.value) return
    saving.value = true
    error.value = ''
    try {
      accept(await accountsService.setAutoRefreshInterval(intervalMs))
    } catch (e) {
      error.value = String(e)
    } finally {
      saving.value = false
    }
  }
  const message = computed(() => {
    if (error.value) return error.value
    if (!status.value) return '正在读取后台刷新设置…'
    if (status.value.error) return status.value.error
    if (status.value.running) return '正在后台刷新额度…'
    const run = status.value.lastRun
    if (!run) return '后台刷新已启用，等待首次查询。'
    return (
      `${new Date(run.finishedAt).toLocaleTimeString('zh-CN', { hour12: false })} · ${run.updated} 个账号额度已更新` +
      (run.failed ? `，${run.failed} 个失败` : '') +
      (run.error ? `：${run.error}` : '')
    )
  })
  onScopeDispose(() => unlisten?.())
  return { status, ready, saving, error, message, initialize, sync, setInterval, accept }
})
