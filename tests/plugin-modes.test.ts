import { beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import PluginModes from '../src/features/plugins/PluginModes.vue'
import type { PluginModeService, PonytailModeState, StsSessionMode } from '../src/types/pluginModes'
beforeEach(() => {
  HTMLDialogElement.prototype.showModal = function () {
    this.setAttribute('open', '')
  }
})
function fixture() {
  const pony: PonytailModeState = {
    mode: 'full',
    configMode: 'full',
    environmentMode: null,
    revision: 'p1',
    path: '/fixture/ponytail/config.json',
    canRollback: false,
    receipt: null,
  }
  const sessions: StsSessionMode[] = [
    '00000000-0000-4000-8000-000000000001',
    '00000000-0000-4000-8000-000000000002',
  ].map((id, i) => ({
    id,
    title: `离线对话 ${i + 1}`,
    cwd: '/fixture/workspace',
    updatedAt: 1,
    mode: 'unconfirmed',
    level: 'watch',
    allowedPaths: null,
    revision: `s${i}`,
    canRollback: false,
    source: 'default',
    receipt: null,
  }))
  const service: PluginModeService = {
    ponytail: vi.fn(async () => structuredClone(pony)),
    setPonytail: vi.fn(async (mode, expected) => {
      expect(expected).toBe(pony.revision)
      pony.mode = mode
      pony.configMode = mode
      pony.revision = 'p2'
      pony.canRollback = true
      pony.receipt = '默认档位已保存并读回'
      return structuredClone(pony)
    }),
    rollbackPonytail: vi.fn(async () => structuredClone(pony)),
    stsSessions: vi.fn(async () => ({ sessions: structuredClone(sessions), skipped: 0 })),
    stsSession: vi.fn(async () => structuredClone(sessions[0]!)),
    setSts: vi.fn(async (change) => {
      const target = sessions.find((s) => s.id === change.sessionId)!
      expect(change.expectedRevision).toBe(target.revision)
      target.mode = change.mode
      target.level = change.level
      target.allowedPaths = change.allowedPaths
      target.revision = 'changed'
      target.canRollback = true
      target.receipt = '该对话契约已保存并读回'
      return structuredClone(target)
    }),
    rollbackSts: vi.fn(async () => structuredClone(sessions[0]!)),
  }
  return { pony, sessions, service }
}
const props = { installed: true, enabled: true, duplicates: false }
describe('plugin mode controls', () => {
  it('supports keyboard selection without saving until confirmation', async () => {
    const { service } = fixture()
    const wrapper = mount(PluginModes, {
      props: { ...props, pluginId: 'ponytail', service },
      attachTo: document.body,
    })
    await flushPromises()
    await wrapper.get('[role=radiogroup]').trigger('keydown', { key: 'ArrowRight' })
    await flushPromises()
    const selected = wrapper.get('[aria-checked="true"]')
    expect(selected.text()).toContain('Ultra')
    expect(document.activeElement).toBe(selected.element)
    expect(service.setPonytail).not.toHaveBeenCalled()
    wrapper.unmount()
  })
  it('rolls back only the selected conversation with its current revision', async () => {
    const { sessions, service } = fixture()
    sessions[1]!.canRollback = true
    service.rollbackSts = vi.fn(async () => ({
      ...structuredClone(sessions[1]!),
      receipt: '回退已读回',
    }))
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'stop-that-shit', service } })
    await flushPromises()
    await wrapper.get('[aria-label="STS 目标对话"]').setValue(sessions[1]!.id)
    await wrapper.findAll('.mode-actions button')[1]!.trigger('click')
    await wrapper.get('dialog .primary').trigger('click')
    await flushPromises()
    expect(service.rollbackSts).toHaveBeenCalledWith(sessions[1]!.id, 's1')
    expect(wrapper.get('.mode-receipt').text()).toContain('回退已读回')
    wrapper.unmount()
  })
  it('previews Ponytail scope and waits for actual readback before reporting a saved mode', async () => {
    const { service } = fixture()
    let finish!: (state: PonytailModeState) => void
    service.setPonytail = vi.fn(
      () =>
        new Promise((resolve) => {
          finish = resolve
        }),
    )
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'ponytail', service } })
    await flushPromises()
    await wrapper.findAll('[role=radio]')[2]!.trigger('click')
    expect(wrapper.text()).toContain('已保存：full')
    expect(service.setPonytail).not.toHaveBeenCalled()
    await wrapper.get('.mode-actions .primary').trigger('click')
    expect(wrapper.get('dialog').text()).toContain('新对话启动时读取')
    await wrapper.get('dialog .primary').trigger('click')
    expect(service.setPonytail).toHaveBeenCalledWith('ultra', 'p1')
    expect(wrapper.find('.mode-receipt').exists()).toBe(false)
    expect(wrapper.emitted('busy')?.[0]).toEqual([true])
    finish({
      mode: 'ultra',
      configMode: 'ultra',
      environmentMode: null,
      revision: 'p2',
      path: '/fixture',
      canRollback: true,
      receipt: '真正读回 ultra',
    })
    await flushPromises()
    expect(wrapper.get('.mode-receipt').text()).toBe('真正读回 ultra')
    expect(wrapper.text()).toContain('已保存：ultra')
    expect(wrapper.emitted('busy')?.at(-1)).toEqual([false])
    wrapper.unmount()
  })
  it('requires choosing an exact STS conversation and does not touch the other one', async () => {
    const { sessions, service } = fixture()
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'stop-that-shit', service } })
    await flushPromises()
    expect(wrapper.get('[aria-label="STS 任务模式"]').attributes('disabled')).toBeDefined()
    expect(service.setSts).not.toHaveBeenCalled()
    await wrapper.get('[aria-label="STS 目标对话"]').setValue(sessions[1]!.id)
    await wrapper.get('[aria-label="STS 任务模式"]').setValue('review')
    await wrapper.get('[aria-label="STS 控制等级"]').setValue('guard')
    await wrapper.get('.mode-actions .primary').trigger('click')
    expect(wrapper.get('dialog').text()).toContain(sessions[1]!.id)
    await wrapper.get('dialog .primary').trigger('click')
    await flushPromises()
    expect(service.setSts).toHaveBeenCalledWith({
      sessionId: sessions[1]!.id,
      expectedRevision: 's1',
      mode: 'review',
      level: 'guard',
      allowedPaths: null,
    })
    expect(sessions[0]!.mode).toBe('unconfirmed')
    expect(wrapper.get('.mode-receipt').text()).toContain('契约已保存并读回')
    wrapper.unmount()
  })
  it('requires a task for Guard and nonempty paths for Lock', async () => {
    const { sessions, service } = fixture()
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'stop-that-shit', service } })
    await flushPromises()
    await wrapper.get('[aria-label="STS 目标对话"]').setValue(sessions[0]!.id)
    await wrapper.get('[aria-label="STS 控制等级"]').setValue('guard')
    expect(wrapper.get('.mode-actions .primary').attributes('disabled')).toBeDefined()
    await wrapper.get('[aria-label="STS 任务模式"]').setValue('change')
    await wrapper.get('[aria-label="STS 控制等级"]').setValue('lock')
    expect(wrapper.get('.mode-actions .primary').attributes('disabled')).toBeDefined()
    await wrapper.get('[aria-label="STS 文件边界"]').setValue('src/a.ts\ntests/a.test.ts\nsrc/a.ts')
    await wrapper.get('.mode-actions .primary').trigger('click')
    await wrapper.get('dialog .primary').trigger('click')
    await flushPromises()
    expect(service.setSts).toHaveBeenCalledWith(
      expect.objectContaining({ allowedPaths: ['src/a.ts', 'tests/a.test.ts'], level: 'lock' }),
    )
    wrapper.unmount()
  })
  it('shows a conflict without pretending a stale change succeeded', async () => {
    const { service } = fixture()
    service.setPonytail = vi.fn().mockRejectedValue(new Error('配置已被外部修改'))
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'ponytail', service } })
    await flushPromises()
    await wrapper.findAll('[role=radio]')[0]!.trigger('click')
    await wrapper.get('.mode-actions .primary').trigger('click')
    await wrapper.get('dialog .primary').trigger('click')
    await flushPromises()
    expect(wrapper.get('.mode-error').text()).toContain('外部修改')
    expect(wrapper.text()).toContain('已保存：full')
    expect(wrapper.find('.mode-receipt').exists()).toBe(false)
    wrapper.unmount()
  })
  it('blocks environment-overridden Ponytail saves and disabled STS changes', async () => {
    const { pony, sessions, service } = fixture()
    pony.environmentMode = 'off'
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'ponytail', service } })
    await flushPromises()
    expect(wrapper.get('.mode-warning').text()).toContain('环境覆盖')
    expect(
      wrapper.findAll('[role=radio]').every((r) => r.attributes('disabled') !== undefined),
    ).toBe(true)
    wrapper.unmount()
    const sts = mount(PluginModes, {
      props: { ...props, enabled: false, pluginId: 'stop-that-shit', service },
    })
    await flushPromises()
    await sts.get('[aria-label="STS 目标对话"]').setValue(sessions[0]!.id)
    await sts.get('[aria-label="STS 任务模式"]').setValue('change')
    await sts.get('[aria-label="STS 控制等级"]').setValue('guard')
    expect(sts.get('.mode-actions .primary').attributes('disabled')).toBeDefined()
    sts.unmount()
  })
  it('shows an empty session list and can read an explicitly supplied conversation ID', async () => {
    const { sessions, service } = fixture()
    service.stsSessions = vi.fn(async () => ({ sessions: [], skipped: 0 }))
    const wrapper = mount(PluginModes, { props: { ...props, pluginId: 'stop-that-shit', service } })
    await flushPromises()
    expect(wrapper.text()).toContain('新对话发送一条普通消息')
    await wrapper.get('[aria-label="STS 对话 ID"]').setValue(sessions[0]!.id)
    await wrapper.get('.mode-id-row button').trigger('click')
    await flushPromises()
    expect(service.stsSession).toHaveBeenCalledWith(sessions[0]!.id)
    expect(wrapper.get('.mode-session-meta').text()).toContain(sessions[0]!.id)
    expect(service.setSts).not.toHaveBeenCalled()
    wrapper.unmount()
  })
})
