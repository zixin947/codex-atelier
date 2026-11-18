import { readFileSync } from 'node:fs'
import { afterEach, expect, it, vi } from 'vitest'
const script = readFileSync('src/features/connections/native-routing-detail.js', 'utf8')
afterEach(() => {
  window.dispatchEvent(new Event('pagehide'))
  document.body.replaceChildren()
})
it('selects actual equipment and reads the existing counters without inventing telemetry', () => {
  document.body.innerHTML = `<main id="view-routing"><div class="rt-stage"><svg class="rt-wires" viewBox="0 0 500 400"></svg><div class="rt-src"><b>测试 Agent</b><small>demo/model</small></div><div class="rt-hub"><b>Atelier</b><small>127.0.0.1</small></div></div><div class="rt-stats"><span><b>7</b><span>请求</span></span></div><div class="rt-log"></div></main>`
  new Function(script)()
  document.dispatchEvent(new Event('DOMContentLoaded'))
  expect(document.querySelector('.bp-inspector h3')?.textContent).toBe('Atelier')
  expect(document.querySelector('.bp-readouts strong')?.textContent).toBe('7')
  ;(document.querySelector('.bp-devices button') as HTMLButtonElement).click()
  expect(document.querySelector('.bp-inspector h3')?.textContent).toBe('测试 Agent')
  expect(document.querySelector('.bp-device-state')?.textContent).toBe('demo/model')
  expect(document.querySelector('.bp-devices button')?.getAttribute('aria-pressed')).toBe('true')
})

it('moves cargo with packets and removes it when the request packet leaves', async () => {
  document.body.innerHTML =
    '<main id="view-routing"><div class="rt-stage"><svg class="rt-wires" viewBox="0 0 500 400"></svg><svg class="rt-sky"></svg><div class="rt-hub"><b>Atelier</b></div></div><div class="rt-log"></div></main>'
  new Function(script)()
  document.dispatchEvent(new Event('DOMContentLoaded'))
  const packet = document.createElementNS('http://www.w3.org/2000/svg', 'circle')
  packet.setAttribute('class', 'pkt')
  packet.setAttribute('cx', '20')
  packet.setAttribute('cy', '40')
  document.querySelector('.rt-sky')!.append(packet)
  await vi.waitFor(() =>
    expect(document.querySelector('.bp-cargo')?.getAttribute('transform')).toBe('translate(20 40)'),
  )
  packet.setAttribute('cx', '80')
  packet.classList.add('back', 'err')
  await vi.waitFor(() => {
    expect(document.querySelector('.bp-cargo')?.getAttribute('transform')).toBe('translate(80 40)')
    expect(document.querySelector('.bp-cargo.back.err')).not.toBeNull()
  })
  packet.remove()
  await vi.waitFor(() => expect(document.querySelector('.bp-cargo')).toBeNull())
})
