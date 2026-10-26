export type BehaviorPluginId = 'ponytail' | 'stop-that-shit'
export interface BehaviorPlugin {
  id: BehaviorPluginId
  name: string
  repository: string
  commit: string
  tag: string
  version: string
  summary: string
  description: string
  license: string
  installed: boolean | null
  enabled: boolean | null
  installedVersion: string | null
  pluginId: string
  duplicates: boolean
  canRollback: boolean
}
export interface PluginSnapshot {
  available: boolean
  reason: string | null
  hostVersion: string | null
  revision: string
  configTarget: string
  plugins: BehaviorPlugin[]
}
export interface BehaviorPluginService {
  status(): Promise<PluginSnapshot>
  install(id: BehaviorPluginId, revision: string): Promise<PluginSnapshot>
  setEnabled(id: BehaviorPluginId, enabled: boolean, revision: string): Promise<PluginSnapshot>
  rollback(id: BehaviorPluginId, revision: string): Promise<PluginSnapshot>
}
