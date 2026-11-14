import { afterEach, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import Threads from '../src/features/threads/Threads.vue'
import { createThreadDemo } from '../src/services/threadsMock'
import tauriConfig from '../src-tauri/tauri.conf.json'
const api = vi.hoisted(() => ({ catalog: vi.fn(), detail: vi.fn(), preview: vi.fn(), apply: vi.fn() }))
vi.mock('../src/services/threads', () => ({ threadsService: api }))
afterEach(() => { document.body.innerHTML = ''; vi.clearAllMocks() })
it('allows the same-origin thread sync frame in the native content security policy', () => {
  const directives = tauriConfig.app.security.csp.split(';').map(value => value.trim().split(/\s+/))
  expect(directives.find(([name]) => name === 'frame-src')).toContain("'self'")
})
it('places the sync button after usage parsing and keeps the alignment confirmation flow', async () => {
  const demo = createThreadDemo()
  api.catalog.mockResolvedValue(await demo.catalog())
  api.detail.mockImplementation((id: string) => demo.detail(id))
  const wrapper = mount(Threads, { attachTo: document.body })
  await flushPromises()
  const sync = wrapper.find('button.thread-sync-button')
  expect(sync.text()).toBe('同步线程')
  expect(sync.classes()).toContain('primary')
  expect(sync.element.previousElementSibling?.textContent?.trim()).toBe('解析本页用量')
  expect(sync.attributes('disabled')).toBeUndefined()
  await sync.trigger('click')
  expect(document.querySelector('iframe')?.getAttribute('src')).toContain('/cockpit.html?total=')
  expect(api.apply).not.toHaveBeenCalled()
  wrapper.unmount()
})
it('opens the original Cockpit surface without passing a filtered list to the old repair service', async () => {
  const demo = createThreadDemo()
  api.catalog.mockResolvedValue(await demo.catalog())
  api.detail.mockImplementation((id: string) => demo.detail(id))
  const wrapper = mount(Threads, { attachTo: document.body })
  await flushPromises()
  await wrapper.findAll('button').find(b => b.text().includes('线程对齐检查'))!.trigger('click')
  expect(document.querySelector('iframe')?.getAttribute('src')).toContain('/cockpit.html?total=')
  expect(api.preview).not.toHaveBeenCalled()
  expect(api.apply).not.toHaveBeenCalled()
  window.dispatchEvent(new MessageEvent('message', { origin: location.origin, data: { type: 'cockpit-repair-close' } }))
  await flushPromises()
  expect(document.querySelector('iframe')).toBeNull()
  wrapper.unmount()
})
