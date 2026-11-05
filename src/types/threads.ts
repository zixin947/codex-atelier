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
}
export interface ThreadCatalog {
  source: 'native' | 'demo'
  database: string
  provider: string
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
  ticket: string
  action: 'provider' | 'migration' | 'archive' | 'restore'
  rollouts?: { path: string }[]
  changes: ThreadChange[]
  warnings: string[]
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
  apply(ticket: string): Promise<void>
  backups(): Promise<ThreadBackup[]>
  restore(id: string): Promise<ThreadPlan>
}
