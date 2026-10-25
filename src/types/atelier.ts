export type Kind = 'MCP' | 'Skills'
export interface Extension {
  id: string
  name: string
  subtitle: string
  kind: Kind
  enabled: boolean
  version: string
  description: string
  tools: string[]
}
export interface Check {
  id: string
  title: string
  group: string
  status: 'pass' | 'warning'
  detail: string
  risk: string
}
export interface Backup {
  id: string
  name: string
  date: string
  size: string
}
export interface Preview {
  id: string
  revision: string
  before: string
  after: string
}
export interface AtelierService {
  listBackups(): Promise<Backup[]>
  listExtensions(): Promise<Extension[]>
  toggle(id: string): Promise<Extension>
  diagnose(): Promise<Check[]>
  preview(id: string): Promise<Preview>
  confirm(preview: Preview): Promise<void>
}
// Future production adapters must preserve all transaction boundaries. No implementation in this prototype.
export interface ConfigurationTransaction {
  readSnapshot(): Promise<{ revision: string; content: string }>
  detectExternalChange(revision: string): Promise<boolean>
  preview(revision: string): Promise<Preview>
  backup(revision: string): Promise<string>
  confirmAndWrite(preview: Preview, confirmation: string): Promise<void>
  validate(): Promise<boolean>
  rollback(backupId: string): Promise<void>
}
