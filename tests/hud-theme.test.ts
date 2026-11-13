import { beforeEach, afterEach, expect, it, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import App from '../src/App.vue'
import { setTheme } from '../src/app/theme'

beforeEach(() => {
  vi.stubGlobal('matchMedia', () => Object.assign(new EventTarget(), { matches: false }))
  Object.defineProperty(Element.prototype, 'animate', {
    configurable: true,
    value: vi.fn(() => ({ cancel: vi.fn(), id: '' })),
  })
})

afterEach(() => {
  setTheme('endfield')
  localStorage.clear()
  vi.unstubAllGlobals()
  Reflect.deleteProperty(Element.prototype, 'animate')
})

it('switches all three skins without losing the current page, form or navigation', async () => {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', component: { template: '<p>概览内容</p>' } },
      { path: '/accounts', component: { template: '<input aria-label="账号搜索" />' } },
    ],
  })
  await router.push('/accounts')
  const wrapper = mount(App, {
    attachTo: document.body,
    global: { plugins: [createPinia(), router] },
  })
  const input = wrapper.get('[aria-label="账号搜索"]')
  await input.setValue('保留搜索内容')
  for (const name of ['千禧机壳', '终末地']) {
    await wrapper.get(`[aria-label="使用${name}皮肤"]`).trigger('click')
    expect(wrapper.get('[aria-label="账号搜索"]').element).toBe(input.element)
    expect((input.element as HTMLInputElement).value).toBe('保留搜索内容')
    expect(router.currentRoute.value.path).toBe('/accounts')
    expect(wrapper.findAll('nav[aria-label="主导航"] a')).toHaveLength(11)
  }
  setTheme('endfield')
  await flushPromises()
  expect(wrapper.find('.sidebar').exists()).toBe(true)
  await wrapper.get('[aria-label="返回环形首页"]').trigger('click')
  await flushPromises()
  expect(router.currentRoute.value.path).toBe('/')
  expect(wrapper.find('.hud-home').exists()).toBe(true)
  expect(wrapper.get('main').isVisible()).toBe(false)
  expect(wrapper.find('.hud-map').exists()).toBe(true)
  expect(wrapper.find('.hud-module.active').exists()).toBe(false)
  await wrapper.get('.hud-module[href="/"]').trigger('click')
  await flushPromises()
  expect(wrapper.find('.sidebar').exists()).toBe(true)
  expect(router.currentRoute.value.path).toBe('/')
  expect(wrapper.get('main').isVisible()).toBe(true)
  expect(wrapper.find('.hud-map').exists()).toBe(false)
  expect(wrapper.text()).toContain('概览内容')
  expect(wrapper.findAll('.workbench-subnav a')).toHaveLength(4)
  wrapper.unmount()
})

it('restores the saved HUD skin when the application loads again', async () => {
  localStorage.setItem('atelier.appearance.theme', 'hud')
  vi.resetModules()
  const { theme, initializeTheme } = await import('../src/app/theme')
  initializeTheme()
  expect(theme.value).toBe('endfield')
  expect(document.documentElement.dataset.theme).toBe('endfield')
})
