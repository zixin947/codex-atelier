export type MaintenanceAction = 'diagnose' | 'clean' | 'rebuild' | 'restart' | 'forceKill'
export interface MaintenanceCheck {
  label: string
  status: 'pass' | 'warning' | 'error' | 'unknown'
  detail: string
}
export interface MaintenanceSnapshot {
  mode: 'native' | 'demo'
  platform: string
  arch: string
  version: string
  accountCount: number
  managedRoot: string
  desktopPath: string | null
  checks: MaintenanceCheck[]
}
export interface MaintenancePlan {
  ticket: string
  action: MaintenanceAction
  title: string
  changes: string[]
  warnings: string[]
  count: number
}
export interface MaintenanceResult {
  action: MaintenanceAction
  completedAt: number
  summary: string
  changed: number
  warnings: string[]
  checks: MaintenanceCheck[]
  backupPath: string | null
}
export interface MaintenanceService {
  snapshot(): Promise<MaintenanceSnapshot>
  diagnose(): Promise<MaintenanceResult>
  preview(action: MaintenanceAction): Promise<MaintenancePlan>
  apply(ticket: string): Promise<MaintenanceResult>
  runDiagnostics(): Promise<DiagnosticReport>
  previewRepair(itemId: string): Promise<DiagnosticRepairPlan>
  applyRepair(ticket: string): Promise<DiagnosticFixResult>
  utilitySettings(): Promise<MaintenanceUtilitySettings>
  previewImageCompat(enabled: boolean): Promise<ImageCompatPlan>
  applyImageCompat(ticket: string): Promise<MaintenanceUtilitySettings>
  exportBundle(): Promise<DiagnosticBundle>
  saveBundle?(): Promise<{ saved: boolean; path: string | null; summary: string }>
  sendBundle(endpoint: string, userNote: string): Promise<DiagnosticSendResult>
}

export type DiagnosticStatus = 'ok' | 'warning' | 'error' | 'unknown' | 'skipped'
export type DiagnosticGroup = 'config' | 'auth' | 'runtime' | 'env'
export interface DiagnosticItem {
  id: string
  group: DiagnosticGroup
  status: DiagnosticStatus
  label: string
  detail: string
  fixable: boolean
}
export interface DiagnosticReport {
  mode: 'native' | 'demo'
  items: DiagnosticItem[]
  hasIssues: boolean
  transitionInFlight: boolean
  checkedAt: number
  root: string
}
export interface DiagnosticRepairPlan {
  ticket: string
  itemIds: string[]
  title: string
  changes: string[]
  warnings: string[]
  requiresDesktopClosed: boolean
}
export interface DiagnosticFixResult {
  fixedIds: string[]
  failedIds: string[]
  details: string[]
  backupPath: string | null
  report: DiagnosticReport
}
export interface MaintenanceUtilitySettings {
  imageCompat: boolean
  diagnosticEndpoint: string | null
}
export interface DiagnosticBundle {
  fileName: string
  bytes: number[]
  summary: string
  included: string[]
  excluded: string[]
}
export interface DiagnosticSendResult {
  reportId: string
  message: string
}
export interface ImageCompatPlan {
  ticket: string
  enabled: boolean
  changes: string[]
  warnings: string[]
}
