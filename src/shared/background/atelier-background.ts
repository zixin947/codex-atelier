/* Original Atelier background: a survey grid with slow contour rings.
 *
 * Written for this project. No upstream renderer, palette file or artwork is
 * used. The sheet is decoration only: it never carries state, never reports
 * progress, and freezes when the page is hidden or motion is reduced.
 */

export interface BackgroundOptions {
  enabled: boolean
  animated: boolean
  trail: boolean
  watermark: boolean
}

interface Ring {
  cx: number
  cy: number
  radius: number
  wobble: number
  phase: number
  drift: number
}

export interface BackgroundRenderer {
  refresh(): void
  destroy(): void
}

const CELL = 48
const RING_STEP = 34

export function mountAtelierBackground(
  host: HTMLElement,
  readOptions: () => BackgroundOptions,
): BackgroundRenderer {
  const canvas = document.createElement('canvas')
  canvas.setAttribute('aria-hidden', 'true')
  const context = canvas.getContext('2d')
  const container = document.createElement('div')
  container.className = 'atelier-background'
  container.setAttribute('data-atelier-background', '')
  container.setAttribute('aria-hidden', 'true')
  container.append(canvas)
  host.prepend(container)

  let rings: Ring[] = []
  let width = 0
  let height = 0
  let frame = 0
  let running = false
  let last = 0
  const pointer = { x: -1, y: -1, active: false }

  function resize() {
    const ratio = Math.min(window.devicePixelRatio || 1, 2)
    const box = host.getBoundingClientRect()
    width = Math.max(1, Math.round(box.width))
    height = Math.max(1, Math.round(box.height))
    canvas.width = Math.round(width * ratio)
    canvas.height = Math.round(height * ratio)
    canvas.style.width = `${width}px`
    canvas.style.height = `${height}px`
    context?.setTransform(ratio, 0, 0, ratio, 0, 0)
    rings = buildRings(width, height)
  }

  function buildRings(w: number, h: number): Ring[] {
    // Anchors sit on a loose grid so islands read as a survey, not a pattern.
    const anchors: Array<[number, number, number]> = [
      [0.13, 0.2, 1],
      [0.42, 0.68, 0.8],
      [0.72, 0.28, 0.92],
      [0.9, 0.76, 0.7],
    ]
    return anchors.map(([ax, ay, scale], index) => ({
      cx: ax * w,
      cy: ay * h,
      radius: Math.max(w, h) * 0.06 * scale + 30,
      wobble: 9 + index * 3,
      phase: index * 1.7,
      drift: 0.08 + index * 0.03,
    }))
  }

  function drawGrid() {
    if (!context) return
    const ink = 'rgba(32, 35, 31, 0.055)'
    context.save()
    context.strokeStyle = ink
    context.lineWidth = 1
    context.beginPath()
    for (let x = CELL; x < width; x += CELL) {
      context.moveTo(Math.round(x) + 0.5, 0)
      context.lineTo(Math.round(x) + 0.5, height)
    }
    for (let y = CELL; y < height; y += CELL) {
      context.moveTo(0, Math.round(y) + 0.5)
      context.lineTo(width, Math.round(y) + 0.5)
    }
    context.stroke()

    // Every fourth line carries the survey frame.
    context.strokeStyle = 'rgba(32, 35, 31, 0.1)'
    context.beginPath()
    for (let x = CELL * 4; x < width; x += CELL * 4) {
      context.moveTo(Math.round(x) + 0.5, 0)
      context.lineTo(Math.round(x) + 0.5, height)
    }
    for (let y = CELL * 4; y < height; y += CELL * 4) {
      context.moveTo(0, Math.round(y) + 0.5)
      context.lineTo(width, Math.round(y) + 0.5)
    }
    context.stroke()
    context.restore()
  }

  function drawRings(time: number) {
    if (!context) return
    context.save()
    context.lineWidth = 1
    for (const ring of rings) {
      const rings_here = 5
      for (let i = 0; i < rings_here; i += 1) {
        const radius = ring.radius + i * RING_STEP
        const phase = ring.phase + time * ring.drift
        context.beginPath()
        const steps = 120
        for (let step = 0; step <= steps; step += 1) {
          const angle = (step / steps) * Math.PI * 2
          const wobble =
            Math.sin(angle * 3 + phase) * ring.wobble + Math.cos(angle * 5 - phase * 0.7) * 4
          const r = radius + wobble
          const x = ring.cx + Math.cos(angle) * r
          const y = ring.cy + Math.sin(angle) * r * 0.82
          if (step === 0) context.moveTo(x, y)
          else context.lineTo(x, y)
        }
        const distance = pointer.active
          ? Math.hypot(ring.cx - pointer.x, ring.cy - pointer.y)
          : Number.POSITIVE_INFINITY
        const near = pointer.active && distance < 260
        context.strokeStyle = near ? 'rgba(92, 87, 0, 0.20)' : 'rgba(32, 35, 31, 0.075)'
        context.stroke()
      }
    }
    context.restore()
  }

  function drawPointer() {
    if (!context || !pointer.active) return
    const options = readOptions()
    if (!options.trail) return
    context.save()
    context.strokeStyle = 'rgba(92, 87, 0, 0.5)'
    context.lineWidth = 1
    context.beginPath()
    context.moveTo(pointer.x - 10, pointer.y)
    context.lineTo(pointer.x + 10, pointer.y)
    context.moveTo(pointer.x, pointer.y - 10)
    context.lineTo(pointer.x, pointer.y + 10)
    context.stroke()
    context.restore()
  }

  function draw(time: number) {
    if (!context) return
    context.clearRect(0, 0, width, height)
    drawGrid()
    drawRings(time)
    drawPointer()
  }

  function loop(timestamp: number) {
    if (!running) return
    const options = readOptions()
    const step = options.animated ? 1 : 0
    if (step || frame === 0) {
      // Cap the redraw rate; the field moves slowly by design.
      if (timestamp - last > 40) {
        last = timestamp
        frame += 1
        draw(timestamp / 1000)
      }
    }
    requestAnimationFrame(loop)
  }

  function start() {
    if (running) return
    running = true
    last = 0
    draw(performance.now() / 1000)
    requestAnimationFrame(loop)
  }

  function stop() {
    running = false
  }

  function onPointerMove(event: PointerEvent) {
    const box = host.getBoundingClientRect()
    pointer.x = event.clientX - box.left
    pointer.y = event.clientY - box.top
    pointer.active = true
  }
  function onPointerLeave() {
    pointer.active = false
  }
  function onVisibility() {
    if (document.hidden) stop()
    else start()
  }
  function onResize() {
    resize()
    draw(performance.now() / 1000)
  }

  resize()
  start()
  window.addEventListener('pointermove', onPointerMove, { passive: true })
  window.addEventListener('pointerleave', onPointerLeave)
  document.addEventListener('visibilitychange', onVisibility)
  window.addEventListener('resize', onResize)

  return {
    refresh() {
      const options = readOptions()
      if (!options.enabled) {
        stop()
        container.style.display = 'none'
        return
      }
      container.style.display = ''
      resize()
      draw(performance.now() / 1000)
      if (options.animated && !document.hidden) start()
      else stop()
    },
    destroy() {
      stop()
      window.removeEventListener('pointermove', onPointerMove)
      window.removeEventListener('pointerleave', onPointerLeave)
      document.removeEventListener('visibilitychange', onVisibility)
      window.removeEventListener('resize', onResize)
      container.remove()
    },
  }
}
