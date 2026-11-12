import { ref, watch, onMounted, onUnmounted, type Ref } from 'vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'

// Native presentation boundary: the Swift renderer only returns a navigation index.
export function useNativeHud(
  host: Ref<HTMLElement | null>,
  visible: () => boolean,
  labels: () => string[],
  navigate: (index: number) => void,
) {
  const ready = ref(false)
  const error = ref('')
  let supported = false
  let disposed = false
  let unlisten: UnlistenFn | undefined
  let observer: ResizeObserver | undefined
  async function update() {
    if (!supported || disposed || !host.value) return
    const rect = host.value.getBoundingClientRect()
    try {
      await invoke('hud_native_update', {
        visible: visible(),
        labels: labels(),
        frame: [rect.x, rect.y, rect.width, rect.height],
      })
      if (!disposed) {
        ready.value = true
        error.value = ''
      }
    } catch (failure) {
      if (!disposed) {
        ready.value = false
        error.value = String(failure)
      }
    }
  }
  watch(visible, update, { flush: 'post' })
  onMounted(async () => {
    if (!isTauri()) return
    supported = await invoke<boolean>('hud_native_supported')
    if (disposed || !supported) return
    unlisten = await listen<number>('atelier:hud-navigate', ({ payload }) => {
      if (Number.isInteger(payload) && payload >= 0 && payload < labels().length) navigate(payload)
    })
    if (disposed) {
      unlisten()
      return
    }
    observer = new ResizeObserver(update)
    observer.observe(host.value!)
    await update()
  })
  onUnmounted(() => {
    disposed = true
    observer?.disconnect()
    unlisten?.()
  })
  return { ready, error }
}
