import { defineStore } from 'pinia'
import { ref } from 'vue'
import { service } from '../services/mock'
import type { Extension, Check, Backup } from '../types/atelier'
export const useAtelier = defineStore('atelier', () => {
  const extensions = ref<Extension[]>([]),
    checks = ref<Check[]>([]),
    backups = ref<Backup[]>([]),
    loading = ref(false),
    error = ref(''),
    notice = ref(''),
    profile = ref('日常开发'),
    demo = ref(true)
  let timer: ReturnType<typeof setTimeout>
  function notify(message: string) {
    notice.value = message
    clearTimeout(timer)
    timer = setTimeout(() => (notice.value = ''), 4500)
  }
  async function load() {
    loading.value = true
    error.value = ''
    try {
      ;[extensions.value, checks.value, backups.value] = await Promise.all([
        service.listExtensions(),
        service.diagnose(),
        service.listBackups(),
      ])
    } catch (e) {
      error.value = e instanceof Error ? e.message : '加载失败'
    } finally {
      loading.value = false
    }
  }
  async function toggle(id: string) {
    const updated = await service.toggle(id)
    extensions.value = extensions.value.map((e) => (e.id === id ? updated : e))
    notify(`${updated.name} 已${updated.enabled ? '启用' : '停用'} · 仅演示状态`)
  }
  return {
    extensions,
    checks,
    backups,
    loading,
    error,
    notice,
    profile,
    demo,
    notify,
    load,
    toggle,
  }
})
