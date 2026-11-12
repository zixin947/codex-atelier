import { expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import Workbench from '../src/features/workbench/Workbench.vue'
import bootstrap from '../src/features/workbench/native-usage.js?raw'

vi.mock('@tauri-apps/api/core', () => ({
  isTauri: () => true,
  invoke: vi.fn(async () => 'magpie://localhost/'),
}))
vi.mock('../src/services/gateway', () => ({
  gatewayService: { snapshot: async () => ({ runtime: { state: 'running' } }) },
}))

it('opens the original Usage page at the selected workbench directory', async () => {
  const wrapper = mount(Workbench, { props: { tab: 'context' } })
  await flushPromises()
  const first = new URL(wrapper.get('iframe').attributes('src'))
  expect(first.searchParams.get('view')).toBe('usage')
  expect(first.searchParams.get('tab')).toBe('context')
  expect(first.searchParams.get('atelierUsage')).toBe('chatgpt')
  expect(first.searchParams.get('atelierContent')).toBe('1')
  expect(wrapper.text()).not.toContain('SIMULATED')
  await wrapper.setProps({ tab: 'requests' })
  await flushPromises()
  expect(new URL(wrapper.get('iframe').attributes('src')).searchParams.get('tab')).toBe('requests')
  wrapper.unmount()
})

it('scopes original API requests even when WebView storage is unavailable', async () => {
  const fetch = vi.fn(async (_input: string | Request) => ({}))
  const window = { fetch }
  const clicked = vi.fn()
  class Anchor {
    href = '/api/usage/requests.csv'
    click() {
      clicked(this.href)
    }
  }
  const run = new Function(
    'window',
    'document',
    'location',
    'localStorage',
    'Request',
    'HTMLAnchorElement',
    bootstrap,
  )
  run(
    window,
    { addEventListener: vi.fn() },
    { href: 'magpie://localhost/?tab=context', protocol: 'magpie:', host: 'localhost' },
    {
      setItem() {
        throw new Error('SecurityError')
      },
    },
    Request,
    Anchor,
  )
  new Anchor().click()
  expect(clicked).toHaveBeenCalledWith('magpie://localhost/api/atelier-chatgpt/usage/requests.csv')
  await window.fetch('/api/usage?period=week')
  await window.fetch('https://example.invalid/api/usage')
  expect(fetch.mock.calls[0]?.[0]).toBe('magpie://localhost/api/atelier-chatgpt/usage?period=week')
  expect(fetch.mock.calls[1]?.[0]).toBe('https://example.invalid/api/usage')
})
