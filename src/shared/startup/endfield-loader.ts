import pixelNameplate from '../../assets/branding/codex-pixel-nameplate.png'

/* Timing adapted from dsh-theme-endfield (MIT), 30-boot-loader.js.
 * Source: 359bfad889b4f3d2e79515c4145f624f34ba84f1. See THIRD_PARTY_NOTICES.md.
 * Horizontal cockpit composition follows the user's screenshot; all insignia
 * are drawn for Atelier. The pixel nameplate is generated from the user's
 * requested Blossom reference; provenance: docs/codex-pixel-nameplate.md.
 * Only a local interface animation; it never represents account/network progress.
 */
export interface BootAnimationOptions {
  ready: () => boolean
  onDispose: () => void
}

export function mountEndfieldLoader(options: BootAnimationOptions): () => void {
  const plate = document.createElement('div')
  plate.dataset.endfieldLoader = ''
  plate.tabIndex = -1
  plate.setAttribute('role', 'region')
  plate.setAttribute('aria-label', '终末地启动动画')
  plate.innerHTML = `
    <div aria-hidden="true" translate="no" lang="en" class="notranslate">
      <div class="field-boot-grid"></div>
      <div class="field-boot-orbit">
        <svg viewBox="0 0 1000 1000" fill="none">
          <circle class="field-boot-orbit-ticks" cx="500" cy="500" r="465" pathLength="360" />
          <circle class="field-boot-orbit-arcs" cx="500" cy="500" r="443" pathLength="360" />
          <circle class="field-boot-orbit-outline" cx="500" cy="500" r="402" />
          <circle class="field-boot-orbit-inner" cx="500" cy="500" r="300" />
          <circle class="field-boot-orbit-core" cx="500" cy="500" r="145" />
          <path class="field-boot-orbit-axis" d="M500 48V952M48 500H952" />
        </svg>
      </div>
      <div class="field-boot-spine"></div>
      <div class="field-boot-underline"></div>
      <div data-endfield-loader-track></div>
      <div data-endfield-loader-fill></div>
      <div data-endfield-loader-meter>
        <span data-endfield-loader-pct><span data-endfield-loader-pct-value>000</span><small>%</small></span>
        <span data-endfield-loader-status></span>
      </div>
      <div class="field-boot-insignia">
        <svg viewBox="0 0 320 300" fill="none">
          <path d="M20 36H300L160 278Z M36 45H284L160 260Z" stroke="currentColor" stroke-width="3" />
          <path d="M111 177L160 78L209 177H184L160 128L136 177Z" fill="currentColor" />
          <path d="M100 190H220M128 204H192" stroke="currentColor" stroke-width="4" />
        </svg>
        <span>ATELIER SYSTEMS</span><small>PERSONAL WORKSTATION / 01</small>
      </div>
      <div class="field-boot-emblem">
        <img src="${pixelNameplate}" alt="Codex 像素铭牌，右侧为结形标志" decoding="sync" fetchpriority="high" />
      </div>
      <div class="field-boot-channel field-boot-channel-left"><b>01</b><span><small>INTERFACE</small><strong data-endfield-loader-interface>INITIALIZING</strong></span></div>
      <div class="field-boot-channel field-boot-channel-right"><b>02</b><span><small>VISUAL SYSTEM</small><strong>ENDFIELD</strong></span></div>
      <div data-endfield-loader-brand><h1>CODEX ATELIER</h1><p>ENDFIELD WORKBENCH / YOUR PERSONAL CONTROL ROOM</p></div>
      <div class="field-boot-progress">
        <div class="field-boot-progress-track"><div data-endfield-loader-progress></div></div>
        <div class="field-boot-progress-caption"><span>WORKSPACE / INITIALIZING</span><b>ATELIER SYSTEMS 01-A</b></div>
      </div>
      <div data-endfield-loader-wipe></div>
    </div>
    <p class="field-boot-accessible" role="status">正在初始化 Atelier 界面，可按 Esc 跳过。</p>`
  const element = (name: string) =>
    plate.querySelector<HTMLElement>(`[data-endfield-loader-${name}]`)!
  const fill = element('fill'),
    progressBar = element('progress'),
    pct = element('pct-value')
  const status = element('status'),
    interfaceStatus = element('interface'),
    wipe = element('wipe')
  const app = document.getElementById('app')
  const wasInert = app?.hasAttribute('inert') ?? false
  const previousFocus =
    document.activeElement instanceof HTMLElement ? document.activeElement : null
  app?.setAttribute('inert', '')
  document.body.append(plate)

  let disposed = false,
    frame = 0,
    interval = 0,
    deadline = 0
  let completeAt: number | null = null
  const start = performance.now()
  const motion = window.matchMedia('(prefers-reduced-motion: reduce)')
  function destroy() {
    if (disposed) return
    disposed = true
    cancelAnimationFrame(frame)
    clearInterval(interval)
    clearTimeout(deadline)
    document.removeEventListener('keydown', key, true)
    document.removeEventListener('visibilitychange', visibility)
    window.removeEventListener('pagehide', destroy)
    motion.removeEventListener('change', motionChanged)
    const restoreFocus = plate.contains(document.activeElement)
    plate.remove()
    if (!wasInert) app?.removeAttribute('inert')
    if (restoreFocus) {
      const target =
        previousFocus?.isConnected && previousFocus !== document.body
          ? previousFocus
          : document.getElementById('main-content')
      target?.focus({ preventScroll: true })
    }
    options.onDispose()
  }
  function key(event: KeyboardEvent) {
    if (event.key !== 'Escape') return
    event.preventDefault()
    event.stopPropagation()
    destroy()
  }
  function visibility() {
    if (document.hidden) destroy()
  }
  function motionChanged() {
    if (motion.matches) destroy()
  }
  function tick() {
    if (disposed) return
    const now = performance.now(),
      elapsed = now - start
    const eased = 1 - Math.pow(1 - Math.min(1, elapsed / 1750), 3)
    // 100% is only reached after Atelier's local initialization settles.
    const progress = options.ready() ? eased : Math.min(0.95, eased)
    const value = Math.floor(progress * 100)
    fill.style.height = `${progress * 100}%`
    progressBar.style.transform = `scaleX(${progress})`
    pct.textContent = String(value).padStart(3, '0')
    interfaceStatus.textContent = options.ready() ? 'READY' : 'INITIALIZING'
    status.textContent =
      value < 45 ? 'Initializing interface…' : value < 100 ? 'Preparing workspace…' : 'Ready'
    if (value < 100) return
    plate.dataset.endfieldLoaderComplete = ''
    completeAt ??= now
    const flourish = now - completeAt - 220
    if (flourish < 0) return
    plate.dataset.endfieldLoaderWiping = ''
    const fraction = Math.min(1, flourish / 520)
    wipe.style.opacity = '1'
    wipe.style.width = `${10 + (1 - Math.pow(1 - fraction, 3)) * Math.max(0, plate.clientWidth - 10)}px`
    if (flourish > 520) {
      plate.dataset.endfieldLoaderExit = ''
      plate.style.opacity = `${Math.max(0, 1 - (flourish - 520) / 620)}`
    }
    if (flourish >= 1140) destroy()
  }
  function paint() {
    tick()
    if (!disposed) frame = requestAnimationFrame(paint)
  }
  document.addEventListener('keydown', key, true)
  document.addEventListener('visibilitychange', visibility)
  window.addEventListener('pagehide', destroy)
  motion.addEventListener('change', motionChanged)
  // The deadline outlives every animation phase; no stalled initialization or
  // throttled frame clock can keep the interface covered indefinitely.
  deadline = window.setTimeout(destroy, 6500)
  interval = window.setInterval(tick, 60)
  paint()
  return destroy
}
