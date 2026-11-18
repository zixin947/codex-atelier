// Blueprint presentation only: reads existing routing nodes; never calls management APIs.
;(() => {
  const start = () => {
    const view = document.getElementById('view-routing')
    if (!view) return
    const ns = 'http://www.w3.org/2000/svg'
    const panel = document.createElement('section')
    panel.className = 'bp-inspector'
    panel.setAttribute('aria-label', '蓝图设备详情')
    panel.innerHTML =
      '<div class="bp-heading">设备详情 <span>BLUEPRINT</span></div><div class="bp-devices" role="group" aria-label="选择设备"></div><div class="bp-portrait" aria-hidden="true"></div><h3></h3><p class="bp-device-state"></p><p class="bp-scope">网关累计 · 全部设备</p><div class="bp-readouts"></div>'
    const tracks = document.createElementNS(ns, 'svg')
    tracks.setAttribute('class', 'bp-tracks')
    tracks.setAttribute('aria-hidden', 'true')
    // Cargo mirrors existing packets, including their lifetime, direction and reduced-motion poses.
    const cargo = new Map()
    const cargoObserver = new MutationObserver(() => {
      const packets = new Set(view.querySelectorAll('.rt-sky .pkt'))
      for (const [packet, box] of cargo)
        if (!packets.has(packet)) {
          box.remove()
          cargo.delete(packet)
        }
      for (const packet of packets) {
        let box = cargo.get(packet)
        if (!box) {
          box = document.createElementNS(ns, 'g')
          box.innerHTML =
            '<path class="bp-crate-top" d="M-8 -6 L0 -10 L8 -6 L0 -2Z"/><path class="bp-crate-left" d="M-8 -6 L0 -2 L0 8 L-8 4Z"/><path class="bp-crate-right" d="M0 -2 L8 -6 L8 4 L0 8Z"/><path class="bp-crate-band" d="M-4 -8 L4 -4 L4 6 M-4 -4 L-4 6"/>'
          packet.parentNode.append(box)
          cargo.set(packet, box)
        }
        const cls =
          'bp-cargo' +
          (packet.classList.contains('back') ? ' back' : '') +
          (packet.classList.contains('err') ? ' err' : '')
        if (box.getAttribute('class') !== cls) box.setAttribute('class', cls)
        const transform = `translate(${packet.getAttribute('cx') || 0} ${packet.getAttribute('cy') || 0})`
        if (box.getAttribute('transform') !== transform) box.setAttribute('transform', transform)
        const visibility = packet.getAttribute('visibility') || 'visible'
        if (box.getAttribute('visibility') !== visibility)
          box.setAttribute('visibility', visibility)
      }
    })
    let observedSky = null
    let selected = null,
      nodes = [],
      lastWire = '',
      lastDetail = '',
      frame = 0
    const update = () => {
      frame = 0
      const stage = view.querySelector('.rt-stage'),
        log = view.querySelector('.rt-log')
      if (!stage || !log) return
      const sky = stage.querySelector('.rt-sky')
      if (sky && sky !== observedSky) {
        cargoObserver.disconnect()
        observedSky = sky
        cargoObserver.observe(sky, {
          childList: true,
          subtree: true,
          attributes: true,
          attributeFilter: ['cx', 'cy', 'visibility', 'class'],
        })
      }
      if (!panel.isConnected) log.prepend(panel)
      if (!tracks.isConnected) stage.append(tracks)
      const next = [
        ...stage.querySelectorAll(
          '.rt-src, .rt-hub, .rt-accts > li:not(.idle):not(.rt-gap):not(.rt-fold):not(.rt-sub)',
        ),
      ]
      if (!next.includes(selected)) selected = stage.querySelector('.rt-hub')
      if (next.length !== nodes.length || next.some((n, i) => n !== nodes[i])) {
        nodes = next
        panel.querySelector('.bp-devices').replaceChildren(
          ...nodes.map((node, i) => {
            const button = document.createElement('button')
            button.type = 'button'
            button.textContent = node.matches('.rt-hub')
              ? '中枢'
              : node.matches('.rt-src')
                ? `Agent ${i + 1}`
                : `供应商 ${nodes.slice(0, i + 1).filter((n) => !n.matches('.rt-src, .rt-hub')).length}`
            button.addEventListener('click', () => {
              selected = node
              update()
            })
            return button
          }),
        )
      }
      nodes.forEach((node, i) => {
        const active = node === selected
        node.classList.toggle('bp-selected', active)
        panel.querySelector('.bp-devices').children[i].setAttribute('aria-pressed', String(active))
      })
      if (selected) {
        const name = selected.querySelector('b')?.textContent.trim() || '设备'
        const state = [...selected.querySelectorAll('small, em, i')]
          .map((n) => n.textContent.trim())
          .filter(Boolean)
          .join(' · ')
        const stats = view.querySelector('.rt-stats')?.textContent || ''
        const signature = name + state + stats + selected.className
        if (signature !== lastDetail) {
          lastDetail = signature
          panel.querySelector('h3').textContent = name
          panel.querySelector('.bp-device-state').textContent = state || '状态见下方请求记录'
          panel.querySelector('.bp-portrait').style.backgroundImage =
            getComputedStyle(selected).backgroundImage
          const readouts = panel.querySelector('.bp-readouts')
          readouts.replaceChildren()
          for (const stat of view.querySelectorAll('.rt-stats > span')) {
            const item = document.createElement('div')
            const number = document.createElement('strong'),
              label = document.createElement('span')
            number.textContent = stat.querySelector('b')?.textContent || '—'
            label.textContent = stat.querySelector('span')?.textContent || ''
            item.append(number, label)
            readouts.append(item)
          }
        }
      }
      const wires = stage.querySelector('.rt-wires')
      const paths = [...wires.querySelectorAll('path[d]')]
      const signature =
        wires.getAttribute('viewBox') + paths.map((p) => p.getAttribute('d')).join('|')
      if (signature !== lastWire) {
        lastWire = signature
        tracks.setAttribute('viewBox', wires.getAttribute('viewBox'))
        tracks.replaceChildren()
        for (const path of paths) {
          // Three coincident strokes form the rail, inner lip and flat belt surface.
          for (const layer of ['rail', 'lip', 'surface']) {
            const belt = document.createElementNS(ns, 'path')
            belt.setAttribute('class', `bp-belt-${layer}`)
            belt.setAttribute('d', path.getAttribute('d'))
            tracks.append(belt)
          }
          const length = path.getTotalLength()
          for (let at = 24; at < length - 8; at += Math.max(42, length / 64)) {
            const p = path.getPointAtLength(at),
              q = path.getPointAtLength(at + 1)
            const arrow = document.createElementNS(ns, 'path')
            arrow.setAttribute('class', 'bp-belt-arrow')
            arrow.setAttribute('d', 'M-3.5 -3.5 L4 0 L-3.5 3.5 Z')
            arrow.setAttribute(
              'transform',
              `translate(${p.x} ${p.y}) rotate(${(Math.atan2(q.y - p.y, q.x - p.x) * 180) / Math.PI})`,
            )
            tracks.append(arrow)
          }
        }
      }
    }
    const schedule = (records) => {
      if (records.every((r) => r.target.closest?.('.bp-inspector, .bp-tracks'))) return
      if (!frame) frame = requestAnimationFrame(update)
    }
    const observer = new MutationObserver(schedule)
    observer.observe(view, {
      childList: true,
      subtree: true,
      characterData: true,
      attributes: true,
      attributeFilter: ['d', 'viewBox'],
    })
    view.addEventListener('click', (event) => {
      const node = event.target.closest('.rt-src, .rt-hub, .rt-accts > li')
      if (nodes.includes(node)) {
        selected = node
        update()
      }
    })
    window.addEventListener(
      'pagehide',
      () => {
        observer.disconnect()
        cargoObserver.disconnect()
        cancelAnimationFrame(frame)
      },
      { once: true },
    )
    update()
  }
  if (document.readyState === 'loading')
    document.addEventListener('DOMContentLoaded', start, { once: true })
  else start()
})()
