import { it, expect, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createRouter, createMemoryHistory } from 'vue-router'
import { ref } from 'vue'
import App from '../src/App.vue'
import { setTheme } from '../src/app/theme'
import { finishStartupInitialization } from '../src/app/startup'
const bridge = vi.hoisted(() => ({ visible: () => false }))
vi.mock('../src/shared/hud/native', () => ({ useNativeHud: (_: unknown, visible: () => boolean) => {
  bridge.visible = visible
  return { ready: ref(true), error: ref('') }
} }))
it('suspends the native overlay for settings and the skin menu, then restores it', async () => {
  Object.defineProperty(HTMLDialogElement.prototype, 'showModal', { configurable: true, value: vi.fn() })
  vi.stubGlobal('matchMedia', () => Object.assign(new EventTarget(), { matches: false }))
  const router = createRouter({ history: createMemoryHistory(), routes: [{ path: '/', component: { template: '<p>home</p>' } }] })
  await router.push('/')
  setTheme('endfield')
  const wrapper = mount(App, { attachTo: document.body, global: { plugins: [createPinia(), router] } })
  try {
    finishStartupInitialization()
    await flushPromises()
    await wrapper.get('[aria-label="应用设置"]').trigger('click')
    expect(bridge.visible()).toBe(false)
    const done = wrapper.findAll('button').find(b => b.text() === '完成')!
    await done.trigger('click')
    const menu = wrapper.get('details.theme-switcher')
    ;(menu.element as HTMLDetailsElement).open = true
    await menu.trigger('toggle')
    expect(bridge.visible()).toBe(false)
    ;(menu.element as HTMLDetailsElement).open = false
    await menu.trigger('toggle')
    // Startup animation may still be finishing; disable it explicitly for this assertion.
    const { startupAnimationActive } = await import('../src/app/startup')
    startupAnimationActive.value = false
    expect(bridge.visible()).toBe(true)
  } finally {
    wrapper.unmount()
    setTheme('endfield')
    Reflect.deleteProperty(HTMLDialogElement.prototype, 'showModal')
    vi.unstubAllGlobals()
  }
})
