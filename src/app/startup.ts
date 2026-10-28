import { readonly, ref, watch } from 'vue'
import { theme } from './theme'
import { mountEndfieldLoader } from '../shared/startup/endfield-loader'

const key = 'atelier.appearance.startupAnimation'
function readPreference() {
  try {
    return localStorage.getItem(key) !== '0'
  } catch {
    return true
  }
}
const enabled = ref(readPreference()),
  active = ref(false)
export const startupAnimationEnabled = readonly(enabled)
export const startupAnimationActive = readonly(active)
let attempted = false,
  ready = false
let dispose: (() => void) | undefined
export function finishStartupInitialization() {
  ready = true
}
export function stopStartupAnimation() {
  dispose?.()
}
export function setStartupAnimationEnabled(value: boolean): boolean {
  enabled.value = value
  if (!value) stopStartupAnimation()
  try {
    localStorage.setItem(key, value ? '1' : '0')
    return true
  } catch {
    return false
  }
}
function play() {
  if (
    active.value ||
    theme.value !== 'endfield' ||
    document.hidden ||
    window.matchMedia('(prefers-reduced-motion: reduce)').matches
  )
    return
  active.value = true
  dispose = mountEndfieldLoader({
    ready: () => ready,
    onDispose: () => {
      active.value = false
      dispose = undefined
    },
  })
}
export function startStartupAnimation() {
  if (attempted) return
  attempted = true
  if (enabled.value) play()
}
export function previewStartupAnimation() {
  play()
}
const stopWatching = watch(theme, (value) => {
  if (value !== 'endfield') stopStartupAnimation()
})
if (import.meta.hot)
  import.meta.hot.dispose(() => {
    stopStartupAnimation()
    stopWatching()
  })
