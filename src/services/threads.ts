import { invoke, isTauri } from '@tauri-apps/api/core'
import type { ThreadsService } from '../types/threads'
import { createThreadDemo } from './threadsMock'
export const threadsService: ThreadsService = isTauri()
  ? {
      catalog: () => invoke('threads_catalog'),
      detail: (id) => invoke('threads_detail', { id }),
      preview: (ids, action, value) => invoke('threads_preview', { ids, action, value }),
      apply: (ticket) => invoke('threads_apply', { ticket, confirmed: true }),
      backups: () => invoke('threads_backups'),
      restore: (id) => invoke('threads_restore', { id }),
    }
  : createThreadDemo()
