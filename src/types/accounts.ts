export interface QuotaWindow {
  usedPercent: number | null
  durationMins: number | null
  resetsAt: number | null
}
export interface AccountMetadata {
  accountName: string | null
  workspaceName: string | null
  profileName: string | null
  planSnapshot: string | null
  hasActiveSubscription: boolean | null
  subscriptionExpiresAt: number | null
  subscriptionWillRenew: boolean | null
  isDelinquent: boolean | null
  source: 'none' | 'tokenSnapshot' | 'subscriptionApi'
  sourceAt: number | null
  error: string | null
}
export interface CurrentAccount {
  accountId: string | null
  state: 'unknown' | 'matched' | 'unmanaged' | 'changed' | 'unavailable'
  path: string | null
  checkedAt: number | null
  message: string
}
export interface ResetCredit {
  id: string
  resetType: string
  status: string
  grantedAt: number | null
  expiresAt: number | null
  noExpiration: boolean
  title: string | null
  description: string | null
}
export interface ResetCredits {
  availableCount: number | null
  credits: ResetCredit[] | null
  detailsCheckedAt: number | null
}
export interface CodexAccount {
  id: string
  name: string
  email: string | null
  plan: string | null
  group: string
  note: string
  status: 'imported' | 'ready' | 'error'
  quota: {
    primary: QuotaWindow | null
    secondary: QuotaWindow | null
    checkedAt: number
    resetCredits: ResetCredits | null
  } | null
  error: string | null
  createdAt: number
  metadata: AccountMetadata
}
export interface AccountsSnapshot {
  available: boolean
  reason: string | null
  revision: string
  accounts: CodexAccount[]
  lastSelectedId: string | null
  rollbackAvailable: boolean
  pendingLogin: AccountLogin | null
  currentAccount: CurrentAccount
}
export interface AccountLogin {
  id: string
  state: 'pending' | 'completed' | 'error' | 'cancelled'
  url: string | null
  message: string | null
  expiresAt: number
}
export interface AccountEdit {
  id: string
  name: string
  group: string
  note: string
}
export interface AccountResult {
  id: string
  ok: boolean
  message: string
}
export interface AccountBatch {
  snapshot: AccountsSnapshot
  results: AccountResult[]
}
export interface AccountAutoRefreshStatus {
  version: number
  configured: boolean
  intervalMs: number
  running: boolean
  startedAt: number | null
  nextRefreshAt: number | null
  lastRun: { finishedAt: number; updated: number; failed: number; error: string | null } | null
  error: string | null
}
export interface AccountSwitch {
  ticket: string
  target: string
  before: string
  after: string
  expiresAt: number
  rollback: boolean
  warning: string
}
