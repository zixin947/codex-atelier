import { beforeEach, expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import OriginalAgents from '../src/features/connections/OriginalAgents.vue'

/* 管理页进入即加载：不再有「打开」按钮，失败只报告一次由用户重试，
   并发调用不会重复启动引擎，卸载后也不再写状态。 */

const bridge = vi.hoisted(() => ({
  calls: [] as string[],
  uiReady: true,
  startMode: 'ok' as 'ok' | 'fail' | 'pending',
  pendingStart: undefined as undefined | (() => void),
}))

vi.mock('@tauri-apps/api/core', () => ({
  isTauri: () => true,
  invoke: async (command: string) => {
    bridge.calls.push(command)
    if (command === 'gateway_original_ui') {
      if (!bridge.uiReady) throw new Error('请先启动网关。')
      return 'magpie://localhost/?view=agents'
    }
    if (command === 'gateway_start') {
      if (bridge.startMode === 'fail') throw new Error('管理服务启动失败。')
      if (bridge.startMode === 'pending')
        await new Promise<void>((resolve) => (bridge.pendingStart = resolve))
      bridge.uiReady = true
      return {}
    }
    throw new Error(`unexpected command ${command}`)
  },
}))

beforeEach(() => {
  bridge.calls = []
  bridge.uiReady = true
  bridge.startMode = 'ok'
  bridge.pendingStart = undefined
})

it('loads the management page on entry without an open button', async () => {
  const wrapper = mount(OriginalAgents, { props: { running: true } })
  await flushPromises()
  expect(wrapper.find('iframe').exists()).toBe(true)
  expect(wrapper.findAll('button').some((b) => b.text().includes('打开'))).toBe(false)
  expect(bridge.calls).toContain('gateway_original_ui')
  expect(bridge.calls).not.toContain('gateway_start')
  wrapper.unmount()
})

it('starts the service once when it is not running and retries only on demand', async () => {
  bridge.uiReady = false
  bridge.startMode = 'fail'
  const wrapper = mount(OriginalAgents)
  await flushPromises()
  expect(wrapper.find('iframe').exists()).toBe(false)
  expect(wrapper.get('[role="alert"]').text()).toContain('管理服务启动失败')
  // 单次尝试，没有隐藏的三次自动重试。
  expect(bridge.calls.filter((c) => c === 'gateway_start')).toHaveLength(1)
  bridge.startMode = 'ok'
  await wrapper.get('[role="alert"] button').trigger('click')
  await flushPromises()
  expect(wrapper.find('iframe').exists()).toBe(true)
  expect(bridge.calls.filter((c) => c === 'gateway_start')).toHaveLength(2)
  wrapper.unmount()
})

it('does not start the engine twice for concurrent entries', async () => {
  bridge.uiReady = false
  bridge.startMode = 'pending'
  const wrapper = mount(OriginalAgents)
  await flushPromises()
  // 并发触发：第二次调用必须被进行中的启动挡住。
  await wrapper.setProps({ running: true })
  await wrapper.setProps({ running: false })
  await flushPromises()
  expect(bridge.calls.filter((c) => c === 'gateway_start')).toHaveLength(1)
  bridge.pendingStart?.()
  await flushPromises()
  expect(wrapper.find('iframe').exists()).toBe(true)
  wrapper.unmount()
})

it('does not write state after unmount', async () => {
  bridge.uiReady = false
  bridge.startMode = 'pending'
  const changed = vi.fn()
  const wrapper = mount(OriginalAgents, { props: { onChanged: changed } })
  await flushPromises()
  expect(bridge.calls.filter((c) => c === 'gateway_start')).toHaveLength(1)
  wrapper.unmount()
  bridge.pendingStart?.()
  await flushPromises()
  // 卸载后既不回传 changed，也不再渲染地址（不写已销毁组件的状态）。
  expect(changed).not.toHaveBeenCalled()
  expect(wrapper.find('iframe').exists()).toBe(false)
  expect(wrapper.find('[role="alert"]').exists()).toBe(false)
})
