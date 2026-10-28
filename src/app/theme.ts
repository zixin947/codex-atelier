import { readonly, ref } from 'vue'

export type ThemeId = 'endfield' | 'millennium'
export interface BackgroundOptions {
  enabled: boolean
  animated: boolean
  trail: boolean
  watermark: boolean
}
const key = 'atelier.appearance.theme'
const backgroundKey = 'atelier.appearance.endfieldBackground'
const defaults: BackgroundOptions = { enabled: true, animated: true, trail: true, watermark: true }
function savedBackground(): BackgroundOptions {
  try {
    const value: unknown = JSON.parse(localStorage.getItem(backgroundKey) ?? 'null')
    if (value === null || typeof value !== 'object') return { ...defaults }
    const candidate = value as Record<string, unknown>
    const boolean = (key: keyof BackgroundOptions): boolean =>
      typeof candidate[key] === 'boolean' ? candidate[key] : defaults[key]
    return {
      enabled: boolean('enabled'),
      animated: boolean('animated'),
      trail: boolean('trail'),
      watermark: boolean('watermark'),
    }
  } catch {
    return { ...defaults }
  }
}
const background = ref<BackgroundOptions>(savedBackground())
export const backgroundOptions = readonly(background)
export function updateBackground(patch: Partial<BackgroundOptions>): boolean {
  background.value = { ...background.value, ...patch }
  try {
    localStorage.setItem(backgroundKey, JSON.stringify(background.value))
    return true
  } catch {
    return false
  }
}
function savedTheme(): ThemeId {
  try {
    return localStorage.getItem(key) === 'millennium' ? 'millennium' : 'endfield'
  } catch {
    return 'endfield'
  }
}
const activeTheme = ref<ThemeId>(savedTheme())
export const theme = readonly(activeTheme)
export function initializeTheme() {
  document.documentElement.dataset.theme = activeTheme.value
}
export function setTheme(value: ThemeId): boolean {
  activeTheme.value = value
  initializeTheme()
  try {
    localStorage.setItem(key, value)
    return true
  } catch {
    return false
  }
}
