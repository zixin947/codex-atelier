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
    hostVersion: 'Desktop engine fixture',
    desktopBridge: {
      state: 'disconnected',
      hosts: 0,
      loadedThreads: 0,
      message: '离线配置读写夹具',
    },
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
  it('allows rollback without current-session control and keeps Hook trust independent', async () => {
    const { data, service } = fixture()
    data.desktopBridge.state = 'error'
    data.plugins[0]!.canRollback = true
    data.plugins[0]!.enabled = true
    data.plugins[0]!.hookTrust.state = 'trusted'
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    const buttons = wrapper.findAll('.behavior-actions button')
    expect(buttons[0]!.attributes('disabled')).toBeUndefined()
    await buttons[0]!.trigger('click')
    await flushPromises()
    expect(service.rollback).toHaveBeenCalledWith('ponytail', 'v1')
    expect(data.plugins[0]!.hookTrust.state).toBe('trusted')
    expect(wrapper.text()).not.toContain('辅助功能')
    wrapper.unmount()
  })
  it('saves for new conversations without a Desktop control connection or UI permission', async () => {
    const { data, service } = fixture()
    data.desktopBridge = {
      state: 'disconnected',
      hosts: 0,
      loadedThreads: 0,
      message: '私有 stdio 宿主不可外接',
    }
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeUndefined()
    expect(wrapper.find('.plugin-accessibility').exists()).toBe(false)
    expect(wrapper.text()).toContain('新对话生效 · 无需重启 Desktop')
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(service.setEnabled).toHaveBeenCalledWith('ponytail', true, 'v1')
    expect(wrapper.get('.plugin-operation-result').text()).toContain('请新建 Desktop 对话')
    expect(wrapper.get('.plugin-operation-result').text()).not.toContain('热重载')
    wrapper.unmount()
  })
  it('reports current-session reload only after the native owner acknowledges it', async () => {
    const { data, service } = fixture()
    data.desktopBridge = {
      state: 'connected',
      hosts: 1,
      loadedThreads: 2,
      message: '已连接 Desktop 桥接宿主',
    }
    data.plugins[0]!.enabled = true
    service.setEnabled = vi.fn(async () => {
      data.plugins[0]!.enabled = false
      data.runtimeReceipt = 'Desktop 宿主已确认热重载 · 覆盖 2 个已加载会话'
      return structuredClone(data)
    })
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.text()).toContain('2 个已加载会话')
    expect(wrapper.text()).not.toContain('已确认热重载')
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(wrapper.get('.plugin-operation-result').text()).toContain('已确认热重载')
    expect(wrapper.get('.plugin-operation-result').text()).toContain('覆盖 2 个已加载会话')
    wrapper.unmount()
  })
  it('recognizes Desktop automatically with no connection card, button or dialog', async () => {
    const { service } = fixture()
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(service.status).toHaveBeenCalledOnce()
    expect(wrapper.find('.plugin-desktop-bridge').exists()).toBe(false)
    expect(wrapper.find('dialog').exists()).toBe(false)
    expect(wrapper.text()).not.toContain('连接 Desktop')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeUndefined()
    expect(service.setEnabled).not.toHaveBeenCalled()
    wrapper.unmount()
  })
  it('does not block configuration switches when current-session control is unavailable', async () => {
    const { data, service } = fixture()
    data.desktopBridge = {
      state: 'disconnected',
      hosts: 0,
      loadedThreads: 0,
      message: '当前 Desktop 未开放会话控制入口',
    }
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeUndefined()
    expect(wrapper.find('.plugin-runtime-unavailable').exists()).toBe(false)
    expect(wrapper.text()).toContain('新对话生效')
    expect(wrapper.text()).not.toContain('连接 Desktop')
    expect(wrapper.find('dialog').exists()).toBe(false)
    expect(service.setEnabled).not.toHaveBeenCalled()
    wrapper.unmount()
  })
  it('rechecks Desktop on window focus and removes listeners on leaving the page', async () => {
    const { data, service } = fixture()
    data.desktopBridge = {
      state: 'disconnected',
      hosts: 0,
      loadedThreads: 0,
      message: '当前会话不可控制',
    }
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeUndefined()
    data.desktopBridge = { state: 'connected', hosts: 1, loadedThreads: 1, message: '已自动识别' }
    window.dispatchEvent(new Event('focus'))
    await flushPromises()
    expect(service.status).toHaveBeenCalledTimes(2)
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeUndefined()
    expect(service.setEnabled).not.toHaveBeenCalled()
    wrapper.unmount()
    window.dispatchEvent(new Event('focus'))
    await flushPromises()
    expect(service.status).toHaveBeenCalledTimes(2)
  })
  it('does not claim hot reload when the host disconnects after a saved write', async () => {
    const { data, service } = fixture()
    data.desktopBridge = { state: 'connected', hosts: 1, loadedThreads: 1, message: '已连接' }
    data.plugins[0]!.enabled = true
    service.setEnabled = vi.fn(async () => {
      data.plugins[0]!.enabled = false
      data.desktopBridge.state = 'error'
      throw new Error('配置已保存，但 Desktop 宿主未确认热重载')
    })
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(wrapper.get('.plugin-operation-result.failed').text()).toContain('未确认热重载')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('false')
    expect(wrapper.get('.plugin-operation-result.failed').text()).toContain('未确认热重载')
    wrapper.unmount()
  })
  it('disables a running plugin only after readback and keeps the saved state after remount', async () => {
    const { data, service } = fixture()
    data.plugins.forEach((p) => {
      p.enabled = true
    })
    let complete!: (value: PluginSnapshot) => void
    service.setEnabled = vi.fn(
      () =>
        new Promise<PluginSnapshot>((resolve) => {
          complete = resolve
        }),
    )
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    expect(service.setEnabled).toHaveBeenCalledWith('ponytail', false, 'v1')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('true')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeDefined()
    expect(wrapper.text()).not.toContain('Ponytail 已停用 · 已保存')
    data.plugins[0]!.enabled = false
    data.revision = 'v2'
    complete(structuredClone(data))
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('false')
    expect(wrapper.get('.plugin-operation-result').text()).toContain('已停用 · 已保存并读回')
    const hooks = wrapper.get('[aria-label="Ponytail Hook 信任状态"]')
    expect(hooks.text()).toContain('随插件停用')
    expect(hooks.classes()).not.toContain('trust-ok')
    expect(wrapper.get('[aria-label="Stop That Shit 插件加载"]').attributes('aria-checked')).toBe(
      'true',
    )
    wrapper.unmount()
    const reopened = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(reopened.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe(
      'false',
    )
    reopened.unmount()
  })
  it('identifies each hook by plugin, event and exact review details without rendering commands as HTML', async () => {
    const { data, service } = fixture()
    data.plugins[0]!.hookTrust.hooks = [
      {
        key: 'fixture-key',
        eventName: 'sessionStart',
        trustStatus: 'trusted',
        enabled: true,
        sourcePath: '/fixture/ponytail/hooks/hooks.json',
        handlerType: 'command',
        command: 'python3 "${PLUGIN_ROOT}/hooks/session-start.py" <script>alert(1)</script>',
        mcpTarget: null,
        matcher: 'startup',
      },
      {
        key: 'fixture-prompt',
        eventName: 'userPromptSubmit',
        trustStatus: 'modified',
        enabled: false,
        sourcePath: '/fixture/ponytail/hooks/hooks.json',
        handlerType: 'command',
        command: 'python3 "${PLUGIN_ROOT}/hooks/prompt.py"',
        mcpTarget: null,
        matcher: null,
      },
    ]
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    const list = wrapper.get('[aria-label="Ponytail Hook 明细"]')
    expect(list.findAll('.hook-row')).toHaveLength(2)
    expect(list.text()).toContain('会话启动时')
    expect(list.text()).toContain('提交消息时')
    expect(list.text()).toContain('SessionStart')
    expect(list.text()).toContain('ponytail@atelier-companions')
    expect(list.text()).toContain('/fixture/ponytail/hooks/hooks.json')
    expect(list.text()).toContain('定义已变化')
    expect(list.text()).toContain('此 Hook 已单独停用')
    expect(list.get('.hook-command').text()).toContain('<script>alert(1)</script>')
    expect(list.find('script').exists()).toBe(false)
    expect(wrapper.find('[aria-label="Stop That Shit Hook 明细"]').exists()).toBe(false)
    expect(service.setEnabled).not.toHaveBeenCalled()
    expect(list.text()).toContain('随插件停用')
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(list.text()).not.toContain('随插件停用')
    expect(list.text()).toContain('此 Hook 已单独停用')
    expect(data.plugins[0]!.hookTrust.hooks[1]!.enabled).toBe(false)
    wrapper.unmount()
  })
  it('reads trust again on refresh and distinguishes changed definitions', async () => {
    const { data, service } = fixture()
    data.plugins[0]!.enabled = true
    data.plugins[0]!.hookTrust = {
      ...data.plugins[0]!.hookTrust,
      state: 'trusted',
      total: 3,
      trusted: 3,
      reason: null,
    }
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    const trust = () => wrapper.get('[aria-label="Ponytail Hook 信任状态"]')
    expect(trust().text()).toContain('3/3 已批准')
    expect(trust().classes()).toContain('trust-ok')
    data.plugins[0]!.hookTrust = {
      ...data.plugins[0]!.hookTrust,
      state: 'reviewRequired',
      trusted: 2,
      modified: 1,
    }
    await wrapper.get('.page-heading .button').trigger('click')
    await flushPromises()
    expect(service.status).toHaveBeenCalledTimes(2)
    expect(trust().text()).toContain('定义已变化')
    expect(trust().text()).toContain('2/3 已批准')
    expect(trust().classes()).not.toContain('trust-ok')
    expect(service.setEnabled).not.toHaveBeenCalled()
    wrapper.unmount()
  })
  it('shows unavailable or unloaded trust without claiming it is untrusted', async () => {
    const { data, service } = fixture()
    data.plugins[0]!.enabled = true
    data.plugins[0]!.hookTrust.reason = 'Hook 读取失败，稍后刷新'
    data.plugins[1]!.hookTrust = {
      ...data.plugins[1]!.hookTrust,
      state: 'notLoaded',
      reason: '插件已停用，CLI 未枚举其 Hook',
    }
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="Ponytail Hook 信任状态"]').text()).toContain('无法读取')
    expect(wrapper.get('[aria-label="Stop That Shit Hook 信任状态"]').text()).toContain(
      '随插件停用',
    )
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('disabled')).toBeUndefined()
    expect(wrapper.text()).not.toContain('首次启用前')
    wrapper.unmount()
  })
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
    expect(wrapper.text()).toContain('已保存并读回')
    wrapper.unmount()
  })
  it('keeps the last real state and shows errors when a write fails', async () => {
    const { data, service } = fixture()
    data.plugins[0]!.enabled = true
    service.setEnabled = vi.fn().mockRejectedValue('配置已被外部修改')
    const wrapper = mount(BehaviorPlugins, { props: { service } })
    await flushPromises()
    await wrapper.get('[aria-label="Ponytail 插件加载"]').trigger('click')
    await flushPromises()
    expect(wrapper.get('[role=alert]').text()).toContain('外部修改')
    expect(wrapper.get('[aria-label="Ponytail 插件加载"]').attributes('aria-checked')).toBe('true')
    expect(wrapper.get('.plugin-operation-result.failed').text()).toContain('操作未确认成功')
    expect(service.status).toHaveBeenCalledTimes(2)
    await wrapper.get('.plugin-operation-result.failed .button').trigger('click')
    await flushPromises()
    expect(service.status).toHaveBeenCalledTimes(3)
    expect(wrapper.find('.plugin-operation-result.failed').exists()).toBe(false)
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
