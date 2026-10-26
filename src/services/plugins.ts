import { invoke, isTauri } from '@tauri-apps/api/core'
import catalog from '../../integrations/behavior-plugins.json'
import type { BehaviorPluginService, PluginSnapshot, BehaviorPluginId } from '../types/plugins'
export function browserSnapshot(): PluginSnapshot {
  return {
    available: false,
    reason: '浏览器仅展示界面。请在 Tauri 桌面应用中检测、安装和切换本机插件。',
    hostVersion: null,
    revision: '',
    configTarget: 'Codex 用户级插件配置',
    plugins: catalog.map((p) => ({
      ...p,
      id: p.id as BehaviorPluginId,
      installed: null,
      enabled: null,
      installedVersion: null,
      pluginId: `${p.id}@atelier-companions`,
      duplicates: false,
      canRollback: false,
      hookTrust: {
        state: 'unavailable',
        total: 0,
        trusted: 0,
        managed: 0,
        untrusted: 0,
        modified: 0,
        disabled: 0,
        reason: '浏览器无法读取本机 Hook 信任状态。',
      },
    })),
  }
}
function native<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) return Promise.reject(new Error('本机插件管理仅在桌面应用中可用。'))
  return invoke<T>(command, args)
}
export const behaviorPluginService: BehaviorPluginService = {
  status: () =>
    isTauri() ? native('behavior_plugins_status') : Promise.resolve(browserSnapshot()),
  install: (id, expectedRevision) => native('behavior_plugin_install', { id, expectedRevision }),
  setEnabled: (id, enabled, expectedRevision) =>
    native('behavior_plugin_set', { id, enabled, expectedRevision }),
  rollback: (id, expectedRevision) => native('behavior_plugin_rollback', { id, expectedRevision }),
}
