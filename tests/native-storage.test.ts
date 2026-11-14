import { expect, it } from 'vitest'
import storage from '../src/features/connections/native-storage.js?raw'

it('migrates legacy preferences before releasing page startup and keeps existing Atelier values', async () => {
  localStorage.clear()
  sessionStorage.clear()
  localStorage.setItem('atelier.theme', 'endfield')
  const scope = window as Window & { atelierStorageReady?: Promise<void> }
  const run = new Function('location', storage)
  run({ origin: 'atelier://localhost' })
  document.dispatchEvent(new Event('DOMContentLoaded'))
  const frame = document.querySelector('iframe')!
  expect(frame.src).toContain('magpie://localhost/atelier-storage-export')
  window.dispatchEvent(
    new MessageEvent('message', {
      origin: 'magpie://localhost',
      source: frame.contentWindow,
      data: {
        type: 'atelier-storage-export',
        local: [
          ['magpie.theme', 'old'],
          ['magpie.gw', '{"key":"literal-magpie"}'],
        ],
        session: [['magpie.usage', 'context']],
      },
    }),
  )
  await scope.atelierStorageReady
  expect(localStorage.getItem('atelier.theme')).toBe('endfield')
  expect(localStorage.getItem('atelier.gw')).toBe('{"key":"literal-magpie"}')
  expect(sessionStorage.getItem('atelier.usage')).toBe('context')
  expect(document.querySelector('iframe')).toBeNull()
})
