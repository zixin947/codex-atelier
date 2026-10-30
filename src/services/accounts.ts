import { invoke, isTauri } from '@tauri-apps/api/core'
import type {
  AccountBatch,
  AccountAutoRefreshStatus,
  AccountEdit,
  AccountLogin,
  AccountsSnapshot,
  AccountSwitch,
} from '../types/accounts'
function native<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) return Promise.reject(new Error('请在 Atelier 桌面应用中操作。'))
  return invoke<T>(command, args)
}
export const accountsService = {
  autoRefreshStatus: (): Promise<AccountAutoRefreshStatus> =>
    native('accounts_auto_refresh_status'),
  setAutoRefreshInterval: (intervalMs: number): Promise<AccountAutoRefreshStatus> =>
    native('accounts_auto_refresh_set', { intervalMs }),
  snapshot: (): Promise<AccountsSnapshot> =>
    isTauri()
      ? native('accounts_snapshot')
      : Promise.resolve({
          available: false,
          reason: '浏览器预览不会连接本机账号。请在 Atelier 桌面应用中添加账号。',
          revision: '',
          accounts: [],
          lastSelectedId: null,
          rollbackAvailable: false,
          pendingLogin: null,
          currentAccount: {
            accountId: null,
            state: 'unknown',
            path: null,
            checkedAt: null,
            message: '浏览器不核对本机账号。',
          },
        }),
  login: (name: string, accountId?: string): Promise<AccountLogin> =>
    native('accounts_login_start', { name, accountId: accountId ?? null }),
  poll: (id: string): Promise<AccountLogin> => native('accounts_login_poll', { id }),
  cancel: (id: string): Promise<void> => native('accounts_login_cancel', { id }),
  open: (id: string): Promise<void> => native('accounts_login_open', { id }),
  detectCurrent: (): Promise<AccountsSnapshot> => native('accounts_detect_current'),
  import: (content: string): Promise<AccountBatch> => native('accounts_import', { content }),
  refresh: (ids: string[], refreshMetadata = true): Promise<AccountBatch> =>
    native('accounts_refresh', { ids, refreshMetadata, includeResetDetails: refreshMetadata }),
  resetCreditPreview: (id: string): Promise<AccountsSnapshot> =>
    native('accounts_reset_credit_preview', { id }),
  edit: (input: AccountEdit, revision: string): Promise<AccountsSnapshot> =>
    native('accounts_edit', { input, revision }),
  remove: (ids: string[], revision: string): Promise<AccountBatch> =>
    native('accounts_remove', { ids, revision }),
  preview: (id: string | null): Promise<AccountSwitch> => native('accounts_switch_preview', { id }),
  apply: (ticket: string): Promise<AccountsSnapshot> =>
    native('accounts_switch_apply', { ticket, confirmed: true }),
  group: (ids: string[], group: string, revision: string): Promise<AccountsSnapshot> =>
    native('accounts_group', { ids, group, revision }),
  export: (ids: string[]): Promise<string> => native('accounts_export', { ids, confirmed: true }),
}
