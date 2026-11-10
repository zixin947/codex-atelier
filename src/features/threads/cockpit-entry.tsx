import React from 'react'
import { createRoot } from 'react-dom/client'
import i18next from 'i18next'
import { initReactI18next } from 'react-i18next'
import zh from '../../../vendor/cockpit-original/src/locales/zh-CN.json'
import { CodexSessionVisibilityRepairModal } from '../../../vendor/cockpit-original/src/components/codex/CodexSessionVisibilityRepairModal'
import '../../../vendor/cockpit-original/src/styles/base.css'
import '../../../vendor/cockpit-original/src/styles/pages/codex-accounts-overview.css'
import '../../../vendor/cockpit-original/src/styles/components.css'
import '../../../vendor/cockpit-original/src/styles/pages/codex-session-manager.css'
import './cockpit-endfield.css'
// Share the host IPC callback registry with the same-origin embedded surface.
for (const key of ['__TAURI_INTERNALS__', '__TAURI_EVENT_PLUGIN_INTERNALS__']) {
  const host = parent as unknown as Record<string, unknown>
  if (host[key]) (window as unknown as Record<string, unknown>)[key] = host[key]
}
const params = new URLSearchParams(location.search)
void i18next.use(initReactI18next).init({ lng: 'zh-CN', resources: { 'zh-CN': { translation: zh } }, interpolation: { escapeValue: false } }).then(() => {
  createRoot(document.getElementById('root')!).render(React.createElement(CodexSessionVisibilityRepairModal, {
    open: true,
    totalSessionCount: Number(params.get("total") || 0),
    selectedSessionIds: JSON.parse(params.get("selected") || "[]") as string[],
    onClose: () => parent.postMessage({ type: 'cockpit-repair-close' }, location.origin),
    onRepaired: () => parent.postMessage({ type: 'cockpit-repair-done' }, location.origin),
  }))
})
