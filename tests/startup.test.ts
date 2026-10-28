import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { mountEndfieldLoader } from '../src/shared/startup/endfield-loader'

let dispose: (() => void) | undefined
let media: MediaQueryList
beforeEach(() => {
  vi.useFakeTimers({
    toFake: [
      'setTimeout',
      'clearTimeout',
      'setInterval',
      'clearInterval',
      'requestAnimationFrame',
      'cancelAnimationFrame',
      'performance',
    ],
  })
  vi.spyOn(document, 'hidden', 'get').mockReturnValue(false)
  media = Object.assign(new EventTarget(), {
    matches: false,
    media: '(prefers-reduced-motion: reduce)',
  }) as MediaQueryList
  vi.stubGlobal('matchMedia', () => media)
  document.body.innerHTML = '<div id="app"><main id="main-content" tabindex="-1"></main></div>'
  localStorage.removeItem('atelier.appearance.startupAnimation')
  localStorage.removeItem('atelier.appearance.theme')
})
afterEach(() => {
  dispose?.()
  dispose = undefined
  vi.useRealTimers()
  vi.restoreAllMocks()
  vi.unstubAllGlobals()
  document.body.innerHTML = ''
})

const plate = () => document.querySelector<HTMLElement>('[data-endfield-loader]')
describe('Endfield startup lifecycle (isolated DOM, no native commands)', () => {
  it('reaches 100%, sweeps, fades, and releases the covered interface and timers', () => {
    const onDispose = vi.fn()
    dispose = mountEndfieldLoader({ ready: () => true, onDispose })
    expect(document.getElementById('app')?.hasAttribute('inert')).toBe(true)
    vi.advanceTimersByTime(1800)
    expect(document.querySelector('[data-endfield-loader-pct]')?.textContent).toBe('100%')
    expect(
      document.querySelector<HTMLElement>('[data-endfield-loader-progress]')?.style.transform,
    ).toBe('scaleX(1)')
    vi.advanceTimersByTime(300)
    expect(plate()?.hasAttribute('data-endfield-loader-wiping')).toBe(true)
    vi.advanceTimersByTime(500)
    expect(Number(plate()?.style.opacity)).toBeLessThan(1)
    vi.advanceTimersByTime(1000)
    expect(plate()).toBeNull()
    expect(document.getElementById('app')?.hasAttribute('inert')).toBe(false)
    expect(onDispose).toHaveBeenCalledOnce()
    expect(vi.getTimerCount()).toBe(0)
  })
  it('waits below 100% for local initialization to settle', () => {
    let ready = false
    dispose = mountEndfieldLoader({ ready: () => ready, onDispose: vi.fn() })
    vi.advanceTimersByTime(2400)
    expect(document.querySelector('[data-endfield-loader-pct]')?.textContent).toBe('095%')
    expect(
      document.querySelector<HTMLElement>('[data-endfield-loader-progress]')?.style.transform,
    ).toBe('scaleX(0.95)')
    ready = true
    vi.advanceTimersByTime(100)
    expect(document.querySelector('[data-endfield-loader-pct]')?.textContent).toBe('100%')
  })
  it('finishes with the fallback clock when requestAnimationFrame stops painting', () => {
    vi.stubGlobal(
      'requestAnimationFrame',
      vi.fn(() => 0),
    )
    dispose = mountEndfieldLoader({ ready: () => true, onDispose: vi.fn() })
    vi.advanceTimersByTime(4000)
    expect(plate()).toBeNull()
    expect(vi.getTimerCount()).toBe(0)
  })
  it('hard-removes a stalled startup and preserves an already inert host', () => {
    document.getElementById('app')!.setAttribute('inert', '')
    dispose = mountEndfieldLoader({ ready: () => false, onDispose: vi.fn() })
    vi.advanceTimersByTime(6600)
    expect(plate()).toBeNull()
    expect(document.getElementById('app')?.hasAttribute('inert')).toBe(true)
    expect(vi.getTimerCount()).toBe(0)
  })
  it('supports Esc and restores keyboard focus after skipping', () => {
    const onDispose = vi.fn()
    dispose = mountEndfieldLoader({ ready: () => true, onDispose })
    plate()!.focus()
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))
    dispose()
    expect(plate()).toBeNull()
    expect(document.activeElement?.id).toBe('main-content')
    expect(onDispose).toHaveBeenCalledOnce()
  })
  it('stops immediately when reduced motion is enabled during playback', () => {
    dispose = mountEndfieldLoader({ ready: () => true, onDispose: vi.fn() })
    Object.defineProperty(media, 'matches', { value: true })
    media.dispatchEvent(new Event('change'))
    expect(plate()).toBeNull()
    expect(vi.getTimerCount()).toBe(0)
  })
  it('cleans up playback on pagehide', () => {
    dispose = mountEndfieldLoader({ ready: () => true, onDispose: vi.fn() })
    window.dispatchEvent(new Event('pagehide'))
    expect(plate()).toBeNull()
    expect(vi.getTimerCount()).toBe(0)
  })
})

describe('startup preference and once-per-page policy', () => {
  it('defaults on and only automatically plays once, including after skip', async () => {
    vi.resetModules()
    const startup = await import('../src/app/startup')
    startup.startStartupAnimation()
    dispose = startup.stopStartupAnimation
    expect(plate()).not.toBeNull()
    startup.stopStartupAnimation()
    startup.startStartupAnimation()
    expect(plate()).toBeNull()
  })
  it('honors a saved off preference; preview remains available without changing it', async () => {
    vi.resetModules()
    localStorage.setItem('atelier.appearance.startupAnimation', '0')
    const startup = await import('../src/app/startup')
    startup.startStartupAnimation()
    dispose = startup.stopStartupAnimation
    expect(plate()).toBeNull()
    startup.previewStartupAnimation()
    expect(plate()).not.toBeNull()
    startup.setStartupAnimationEnabled(false)
    expect(plate()).toBeNull()
    expect(localStorage.getItem('atelier.appearance.startupAnimation')).toBe('0')
  })
  it('never shows for the millennium theme or reduced motion', async () => {
    vi.resetModules()
    localStorage.setItem('atelier.appearance.theme', 'millennium')
    const startup = await import('../src/app/startup')
    startup.startStartupAnimation()
    dispose = startup.stopStartupAnimation
    expect(plate()).toBeNull()
    const { setTheme } = await import('../src/app/theme')
    setTheme('endfield')
    Object.defineProperty(media, 'matches', { value: true })
    startup.previewStartupAnimation()
    expect(plate()).toBeNull()
  })
})
