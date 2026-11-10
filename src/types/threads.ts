export interface ThreadRecord {
  id: string
  title: string
  cwd: string
  provider: string
  model: string
  updatedAt: number
  createdAt: number
  archived: boolean
  tokens: number | null
  rolloutPath: string
  source: string
  kind?: 'conversation' | 'subtask' | 'unverified'
}
export interface ThreadCatalog {
  source: 'native' | 'demo'
  listingSource?: 'official' | 'storage' | 'index'
  database: string
  provider: string
  providerOptions?: string[]
  checkedAt: number
  threads: ThreadRecord[]
  warnings: string[]
}
export interface ThreadUsage {
  input: number
  cached: number
  output: number
  reasoning: number
  total: number
}
export interface ThreadDetail {
  compressed?: boolean
  resolvedPath?: string
  id: string
  bytes: number
  turns: number
  usage: ThreadUsage | null
  tools: { name: string; count: number }[]
  timeline: { date: string; total: number }[]
  modelUsage: { model: string; total: number }[]
  models: string[]
  messages: { role: string; text: string }[]
  warnings: string[]
  encrypted: boolean
  complete: boolean
}
export interface ThreadChange {
  id: string
  title: string
  before: string
  after: string
}
export interface ThreadPlan {
  upstreamReport?: ThreadRepairReport
  ticket: string
  action: 'provider' | 'migration' | 'archive' | 'restore'
  targetProvider?: string | null
  selectedCount?: number
  indexChanges?: { id: string; database: string; table: string; before: string; after: string }[]
  fileTimeChanges?: { path: string; before: number; after: number }[]
  rollouts?: { path: string }[]
  visibilityChanges?: number
  metadataChanges?: {
    id: string
    field: string
    before: string | number | null
    after: string | number | null
  }[]
  changes: ThreadChange[]
  warnings: string[]
}
export interface ThreadRepairReport {
  localProjectRecoveryDeferred?: boolean
  remainingChanges?: number
  verificationError?: string
  message: string
  updatedSqliteRowCount?: number
  changedRolloutFileCount?: number
  insertedCatalogRowCount?: number
  removedCatalogRowCount?: number
  updatedGlobalStateEntryCount?: number
  metadataRebuildFailedCount?: number
  skippedRolloutFileCount?: number
  skippedSqliteFileCount?: number
  backupDirs?: string[]
}
export interface ThreadBackup {
  id: string
  createdAt: number
  action: string
  count: number
}
export interface ThreadsService {
  catalog(): Promise<ThreadCatalog>
  detail(id: string): Promise<ThreadDetail>
  preview(
    ids: string[],
    action: 'provider' | 'migration' | 'archive',
    value: string,
  ): Promise<ThreadPlan>
  apply(ticket: string): Promise<ThreadRepairReport | void>
  backups(): Promise<ThreadBackup[]>
  restore(id: string): Promise<ThreadPlan>
}
