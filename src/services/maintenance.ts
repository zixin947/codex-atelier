import { invoke, isTauri } from '@tauri-apps/api/core'
import type { MaintenanceService } from '../types/maintenance'
import { createMaintenanceMock } from './maintenanceMock'

export const maintenanceService: MaintenanceService = isTauri()
  ? {
      runDiagnostics: () => invoke('maintenance_run_diagnostics'),
      previewRepair: (itemId) => invoke('maintenance_preview_repair', { itemId }),
      applyRepair: (ticket) => invoke('maintenance_apply_repair', { ticket, confirmed: true }),
      previewDirectoryProbe: () => invoke('maintenance_preview_directory_probe'),
      applyDirectoryProbe: (ticket) =>
        invoke('maintenance_apply_directory_probe', { ticket, confirmed: true }),
      utilitySettings: () => invoke('maintenance_utility_settings'),
      previewImageCompat: (enabled) => invoke('maintenance_preview_image_compat', { enabled }),
      applyImageCompat: (ticket) =>
        invoke('maintenance_apply_image_compat', { ticket, confirmed: true }),
      exportBundle: () => invoke('maintenance_export_bundle'),
      saveBundle: () => invoke('maintenance_save_bundle'),
      sendBundle: (endpoint, userNote) =>
        invoke('maintenance_send_bundle', { endpoint, userNote, confirmed: true }),
      snapshot: () => invoke('maintenance_snapshot'),
      diagnose: () => invoke('maintenance_diagnose'),
      preview: (action) => invoke('maintenance_preview', { action }),
      apply: (ticket) => invoke('maintenance_apply', { ticket, confirmed: true }),
    }
  : createMaintenanceMock()
