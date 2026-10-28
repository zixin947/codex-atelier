import { invoke, isTauri } from '@tauri-apps/api/core'
import type { GatewayService, GatewaySnapshot } from '../types/gateway'
export function unavailableGateway(): GatewaySnapshot {
  return {
    available: false,
    reason: '请在 Atelier 桌面应用中管理账号和本地网关。浏览器预览不连接本机数据。',
    revision: '',
    profiles: [],
    settings: {
      port: 18910,
      autoSwitch: false,
      maxAttempts: 2,
      preferredProfileId: null,
      model: '',
    },
    runtime: {
      state: 'stopped',
      address: null,
      activeRequests: 0,
      completedRequests: 0,
      failedRequests: 0,
      message: null,
    },
    events: [],
  }
}
function native<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) return Promise.reject(new Error('此操作仅在 Atelier 桌面应用中可用。'))
  return invoke<T>(command, args)
}
export const gatewayService: GatewayService = {
  snapshot: () => (isTauri() ? native('gateway_snapshot') : Promise.resolve(unavailableGateway())),
  saveProvider: (input, expectedRevision) =>
    native('gateway_provider_save', { input, expectedRevision }),
  updateProfile: (input, expectedRevision) =>
    native('gateway_profile_update', { input, expectedRevision }),
  removeProfile: (id, expectedRevision) =>
    native('gateway_profile_remove', { id, expectedRevision }),
  saveSettings: (settings, expectedRevision) =>
    native('gateway_settings_save', { settings, expectedRevision }),
  selectProfile: (id, expectedRevision) =>
    native('gateway_profile_select', { id, expectedRevision }),
  start: () => native('gateway_start'),
  stop: () => native('gateway_stop'),
  localKey: () => native('gateway_local_key'),
  refreshModels: (id) => native('gateway_models_refresh', { id }),
  beginLogin: (name, accountId) =>
    native('gateway_oauth_begin', { name, accountId: accountId ?? null }),
  openLogin: (id) => native('gateway_open_login', { id }),
  pollLogin: (id) => native('gateway_oauth_poll', { id }),
  cancelLogin: (id) => native('gateway_oauth_cancel', { id }),
  signOut: (id) => native('gateway_oauth_signout', { id }),
  preview: () => native('gateway_config_preview'),
  launchDesktop: () => native('gateway_launch_desktop'),
}
