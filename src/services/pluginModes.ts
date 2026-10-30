import { invoke, isTauri } from '@tauri-apps/api/core'
import type { PluginModeService } from '../types/pluginModes'
function native<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri())
    return Promise.reject(new Error('浏览器仅预览。真实模式读取和保存请使用 Atelier 桌面应用。'))
  return invoke<T>(command, args)
}
export const pluginModeService: PluginModeService = {
  ponytail: () => native('plugin_pony_mode'),
  setPonytail: (mode, expectedRevision) =>
    native('plugin_pony_mode_set', { mode, expectedRevision }),
  rollbackPonytail: (expectedRevision) => native('plugin_pony_mode_rollback', { expectedRevision }),
  stsSessions: () => native('plugin_sts_sessions'),
  stsSession: (sessionId) => native('plugin_sts_mode', { sessionId }),
  setSts: (change) => native('plugin_sts_mode_set', { change }),
  rollbackSts: (sessionId, expectedRevision) =>
    native('plugin_sts_mode_rollback', { sessionId, expectedRevision }),
}
