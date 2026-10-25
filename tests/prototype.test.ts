import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import { createRouter, createMemoryHistory } from 'vue-router'
import { useAtelier } from '../src/app/store'
import { createMockAdapter } from '../src/services/mock'
import Extensions from '../src/features/Extensions.vue'
import Diagnostics from '../src/features/Diagnostics.vue'
import App from '../src/App.vue'
import { router as appRouter } from '../src/app/router'
const settle = async () => {
  await new Promise((r) => setTimeout(r, 320))
  await flushPromises()
}
beforeEach(() => {
  setActivePinia(createPinia())
  HTMLDialogElement.prototype.showModal = function () {
    this.setAttribute('open', '')
  }
})
async function loaded() {
  const store = useAtelier()
  await store.load()
  return store
}
async function routePlugin() {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [{ path: '/', component: { template: '<div />' } }],
  })
  await router.push('/')
  return router
}
describe('mock isolation', () => {
  it('returns independent snapshots and rejects unknown extensions', async () => {
    const a = createMockAdapter(),
      b = createMockAdapter()
    const first = await a.listExtensions()
    first[0]!.name = 'changed'
    expect((await a.listExtensions())[0]!.name).toBe('Filesystem')
    await a.toggle('filesystem')
    expect((await b.listExtensions())[0]!.enabled).toBe(true)
    await expect(a.toggle('missing')).rejects.toThrow('示例扩展不存在')
  })
  it('previews actual issue-specific differences', async () => {
    const a = createMockAdapter()
    expect((await a.preview('extension')).after).toContain('/demo/workspace/')
    expect((await a.preview('provider')).after).not.toContain('provider_alias')
    expect((await a.preview('b1')).after).toContain('backup-demo')
  })
})
describe('prototype interactions', () => {
  it('searches, filters, switches categories and toggles demo state', async () => {
    const store = await loaded()
    const wrapper = mount(Extensions, { global: { plugins: [await routePlugin()] } })
    expect(wrapper.findAll('.extension-card')).toHaveLength(4)
    await wrapper.get('input').setValue('no match')
    expect(wrapper.text()).toContain('没有找到匹配扩展')
    await wrapper.get('.empty button').trigger('click')
    await wrapper.get('select').setValue('disabled')
    expect(wrapper.findAll('.extension-card')).toHaveLength(1)
    expect(wrapper.get('.extension-card').text()).toContain('Browser tools')
    await wrapper.get('[role=switch]').trigger('click')
    await settle()
    expect(store.extensions.find((e) => e.id === 'browser')!.enabled).toBe(true)
    await wrapper.get('select').setValue('all')
    await wrapper.findAll('.tabs button')[1]!.trigger('click')
    expect(wrapper.findAll('.extension-card')).toHaveLength(3)
    store.demo = false
    await flushPromises()
    expect(wrapper.get('[role=switch]').attributes('disabled')).toBeDefined()
    wrapper.unmount()
  })
  it('requires explicit confirmation and only simulates repair', async () => {
    await loaded()
    const wrapper = mount(Diagnostics, { attachTo: document.body })
    const triggerButton = wrapper.get('.button.primary').element as HTMLButtonElement
    triggerButton.focus()
    await wrapper.get('.button.primary').trigger('click')
    await settle()
    expect(wrapper.find('dialog[open]').exists()).toBe(true)
    const confirm = wrapper.findAll('.modal-actions button')[1]!
    expect(confirm.attributes('disabled')).toBeDefined()
    await wrapper.get('input[type=checkbox]').setValue(true)
    await confirm.trigger('click')
    await settle()
    expect(wrapper.find('dialog').exists()).toBe(false)
    expect(document.activeElement).toBe(wrapper.get('.button.primary').element)
    expect(useAtelier().notice).toContain('未写入配置')
    wrapper.unmount()
  })
  it('surfaces service failure with retry', async () => {
    const store = await loaded()
    const { service } = await import('../src/services/mock')
    const spy = vi.spyOn(service, 'listExtensions').mockRejectedValueOnce(Error('演示服务失败'))
    await store.load()
    expect(store.error).toBe('演示服务失败')
    await store.load()
    expect(store.error).toBe('')
    spy.mockRestore()
  })
  it('disables demo interactions from application settings', async () => {
    await appRouter.push('/')
    const wrapper = mount(App, { global: { plugins: [appRouter] } })
    await settle()
    await wrapper.get('[aria-label="应用设置"]').trigger('click')
    expect(wrapper.find('dialog').text()).toContain('演示交互')
    await wrapper.get('[role=switch]').trigger('click')
    expect(useAtelier().demo).toBe(false)
    wrapper.unmount()
  })
})
