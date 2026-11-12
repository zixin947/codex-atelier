// EndfieldHUD HUDDesignMotion / HUDSubsectionTransition, adapted to WAAPI.
// Copyright (c) 2026 DDDuoDuo, MIT. See public/licenses/EndfieldHUD.txt.
import { nextTick, onMounted, onUnmounted, watch, type Ref } from 'vue'

// Only HUD presentation changes here; routes and business results are never delayed.
export function useHudMotion(
  host: Ref<HTMLElement | null>,
  state: () => { active: boolean; expanded: boolean; route: string },
) {
  const animations = new Set<Animation>()
  let media: MediaQueryList | undefined
  let keyboard = false
  let frame = 0
  let plane: HTMLElement | null = null
  function settle() {
    for (const animation of animations) animation.cancel()
    animations.clear()
    if (host.value) delete host.value.dataset.hudTransitioning
    cancelAnimationFrame(frame)
    if (plane) plane.style.transform = ''
  }
  function allowed() {
    if (!media) {
      media = matchMedia('(prefers-reduced-motion: reduce)')
      media.addEventListener('change', settle)
    }
    return !media.matches && !keyboard && !document.hidden
  }
  function play(element: Element, keyframes: Keyframe[], delay = 0) {
    const style = getComputedStyle(host.value!)
    const animation = element.animate(keyframes, {
      duration: parseFloat(style.getPropertyValue('--nav-select-duration')),
      easing: style.getPropertyValue('--ease').trim(),
      delay,
      fill: 'backwards',
    })
    animation.id = 'hud.transition'
    animations.add(animation)
    host.value!.dataset.hudTransitioning = 'true'
    animation.onfinish = animation.oncancel = () => {
      animations.delete(animation)
      if (!animations.size && host.value) delete host.value.dataset.hudTransitioning
    }
  }
  function enter() {
    const root = host.value!
    if (!state().active || !allowed()) return
    root.querySelectorAll('[data-hud-tile]').forEach((tile, index) => {
      const base = getComputedStyle(tile).transform
      play(
        tile,
        [
          { transform: `translateY(18px) ${base === 'none' ? '' : base}`, opacity: 0 },
          { transform: base, opacity: 1, offset: 0.6 },
          { transform: base, opacity: 1 },
        ],
        (index % 7) * 36,
      )
    })
    root.querySelectorAll('.hud-ring-detail').forEach((ring, index) => {
      play(ring, [
        { transform: `rotate(${index ? 25 : -15}deg)`, opacity: 0 },
        { transform: 'rotate(0deg)', opacity: 1 },
      ])
    })
    if (!state().expanded) return
    const panel = root.querySelector('[data-hud-panel]')!
    play(
      panel,
      [
        { clipPath: 'inset(0 48% round 4px)', opacity: 0.4 },
        { clipPath: 'inset(0 round 4px)', opacity: 1 },
      ],
      72,
    )
  }
  watch(state, async (to, from, cleanup) => {
    const root = host.value
    if (!root) return
    const before = new Map(
      [...root.querySelectorAll('[data-hud-tile]')].map((tile) => [
        tile,
        tile.getBoundingClientRect(),
      ]),
    )
    const panel = root.querySelector('[data-hud-panel]')!
    const oldPanel = panel.getBoundingClientRect()
    settle()
    let cancelled = false
    cleanup(() => {
      cancelled = true
    })
    await nextTick()
    if (cancelled || !to.active || !allowed()) return
    if (!from.active) {
      enter()
      return
    }
    if (to.expanded !== from.expanded) {
      root.querySelectorAll('[data-hud-tile]').forEach((tile) => {
        const old = before.get(tile)!
        const end = tile.getBoundingClientRect()
        const base = getComputedStyle(tile).transform
        play(tile, [
          {
            transform: `translate(${old.left - end.left}px, ${old.top - end.top}px) ${base === 'none' ? '' : base}`,
          },
          { transform: base },
        ])
      })
      if (!to.expanded) return
      const end = panel.getBoundingClientRect()
      const left = Math.max(0, oldPanel.left - end.left)
      const right = Math.max(0, end.right - oldPanel.right)
      play(panel, [
        { clipPath: `inset(0 ${right}px 0 ${left}px round 4px)`, opacity: 0.65 },
        { clipPath: 'inset(0 round 4px)', opacity: 1 },
      ])
    } else if (to.route !== from.route) {
      play(panel, [
        { clipPath: 'inset(0 8% 0 0 round 4px)', opacity: 0.65 },
        { clipPath: 'inset(0 round 4px)', opacity: 1 },
      ])
    }
  })
  function pointer() {
    keyboard = false
  }
  function key() {
    keyboard = true
    settle()
  }
  function visibility() {
    if (host.value) host.value.dataset.hudPaused = String(document.hidden)
    if (document.hidden) settle()
  }
  function follow(event: PointerEvent) {
    if (event.pointerType !== 'mouse') return
    keyboard = false
    if (!state().active || state().expanded || !allowed()) return
    cancelAnimationFrame(frame)
    frame = requestAnimationFrame(() => {
      plane = host.value!.querySelector<HTMLElement>('.hud-orbit-plane')!
      const rect = host.value!.getBoundingClientRect()
      const x = ((event.clientX - rect.left) / rect.width) * 2 - 1
      const y = ((event.clientY - rect.top) / rect.height) * 2 - 1
      plane.style.transform = `perspective(1100px) rotateX(${-y * 2}deg) rotateY(${x * 3}deg) translate3d(${x * 4}px,${y * 3}px,0)`
    })
  }
  onMounted(() => {
    document.addEventListener('pointerdown', pointer, true)
    document.addEventListener('keydown', key, true)
    document.addEventListener('visibilitychange', visibility)
    host.value!.addEventListener('pointermove', follow)
    host.value!.addEventListener('pointerleave', settle)
    visibility()
    enter()
  })
  onUnmounted(() => {
    settle()
    media?.removeEventListener('change', settle)
    document.removeEventListener('pointerdown', pointer, true)
    document.removeEventListener('keydown', key, true)
    document.removeEventListener('visibilitychange', visibility)
    host.value?.removeEventListener('pointermove', follow)
    host.value?.removeEventListener('pointerleave', settle)
  })
}
