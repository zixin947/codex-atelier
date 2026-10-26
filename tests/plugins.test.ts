import { beforeEach, describe, it, expect, vi } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import BehaviorPlugins from '../src/features/plugins/BehaviorPlugins.vue'
import { browserSnapshot, behaviorPluginService } from '../src/services/plugins'
import type { BehaviorPluginService, PluginSnapshot } from '../src/types/plugins'
beforeEach(() => {
  setActivePinia(createPinia())
  HTMLDialogElement.prototype.showModal = function () {
    this.setAttribute('open', '')
  }
})
function fixture() {
  const data: PluginSnapshot = {
    ...browserSnapshot(),
    available: true,
    reason: null,
    hostVersion: 'codex-cli test',
    revision: 'v1',
    plugins: browserSnapshot().plugins.map((p) => ({
      ...p,
      installed: true,
      enabled: false,
      installedVersion: p.version,
    })),
  }
  const service: BehaviorPluginService = {
    status: vi.fn(async () => structuredClone(data)),
    install: vi.fn(async () => structuredClone(data)),
    setEnabled: vi.fn(async (id, enabled, revision) => {
      expect(revision).toBe(data.revision)
      data.plugins.find((p) => p.id === id)!.enabled = enabled
      data.revision = 'v2'
      return structuredClone(data)
    }),
    rollback: vi.fn(async () => structuredClone(data)),
  }
  return { data, service }
}
describe('real plugin controls', () => {
  it('never offers real writes in the browser', async () => {
    const snapshot = await behaviorPluginService.status()
    expect(snapshot.available).toBe(false)
    expect(snapshot.plugins.every((p) => p.installed === null)).toBe(true)
    await expect(behaviorPluginService.setEnabled('ponytail', true, 'v1')).rejects.toThrow(
      '仅在桌面',
    )
    const wrapper = mount(BehaviorPlugins)
    await flushPromises()
    expect(
      wrapper.findAll('[role=switch]').every((b) => b.attributes('disabled') !== undefined),
    ).toBe(true)
    expect(wrapper.text()).toContain('浏览器仅展示界面')
    wrapper.unmount()
  })
  it('changes only one plugin, using the current official revision', async () => {
    const { service } = fixture()
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('false')
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(service.setEnabled).toHaveBeenCalledWith('ponytail', true, 'v1')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('true')
    expect(wrapper.get('[aria-label="Stop That Shit 插件加载"]').attributes('aria-checked')).toBe(
      'false',
    )
    expect(wrapper.text()).toContain('未观测')
    wrapper.unmount()
  })
  it('keeps the last real state and shows errors when a write fails', async () => {
    const { service } = fixture()
    service.setEnabled = vi.fn().mockRejectedValue('配置已被外部修改')
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(wrapper.get('[role=alert]').text()).toContain('外部修改')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('false')
    expect(service.status).toHaveBeenCalledTimes(2)
    wrapper.unmount()
  })
  it('requires the install preview before installing a fixed version', async () => {
    const { data, service } = fixture()
    data.plugins[0]!.installed = false
    const wrapper = mount(BehaviorPlugins, { props: { service }, attachTo: document.body })
    await flushPromises()
    await wrapper.get('.behavior-actions .primary').trigger('click')
    expect(service.install).not.toHaveBeenCalled()
    expect(wrapper.get('dialog').text()).toContain('v5.3.2')
    expect(wrapper.get('dialog').text()).toContain('默认停用')
    await wrapper.get('.modal-actions .primary').trigger('click')
    await flushPromises()
    expect(service.install).toHaveBeenCalledWith('ponytail', 'v1')
    wrapper.unmount()
  })
  it('blocks duplicate sources and allows a saved-state rollback', async () => {
    const { data, service } = fixture()
    data.plugins[0]!.duplicates = true
    data.plugins[1]!.canRollback = true
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeDefined()
    await wrapper.findAll('.behavior-actions .button')[1]!.trigger('click')
    await flushPromises()
    expect(service.rollback).toHaveBeenCalledWith('stop-that-shit', 'v1')
    wrapper.unmount()
  })
})
