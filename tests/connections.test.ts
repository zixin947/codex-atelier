import { expect, it, vi } from 'vitest'
import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import Connections from '../src/features/connections/Connections.vue'
import { unavailableGateway } from '../src/services/gateway'
import type { GatewayService, GatewaySettings } from '../src/types/gateway'

it('selects an existing Codex login and saves its model as the gateway route', async () => {
  const snapshot = unavailableGateway()
  snapshot.available = true
  snapshot.profiles = [
    {
      id: 'fixture-account',
      kind: 'codex',
      name: '已登录账号',
      email: null,
      enabled: true,
      priority: 100,
      baseUrl: 'https://chatgpt.com/backend-api/codex',
      authStatus: 'ready',
      models: [{ id: 'fixture-model', name: '模型' }],
      cooldownUntil: null,
      lastError: null,
      requests: 0,
      inputTokens: 0,
      outputTokens: 0,
      createdAt: 1,
    },
  ]
  const saveSettings = vi.fn(async (settings: GatewaySettings) => ({ ...snapshot, settings }))
  const service = { snapshot: async () => snapshot, saveSettings } as unknown as GatewayService
  const wrapper = mount(Connections, {
    props: { service },
    global: { plugins: [createPinia()], stubs: { RouterLink: true } },
  })
  await flushPromises()
  const select = wrapper.find('select')
  expect(select.text()).toContain('已登录账号 · Codex 登录账号')
  await select.setValue('fixture-account')
  expect((wrapper.find('input[list="gateway-models"]').element as HTMLInputElement).value).toBe(
    'fixture-model',
  )
  await wrapper
    .findAll('button')
    .find((button) => button.text() === '保存路由设置')!
    .trigger('click')
  await flushPromises()
  expect(saveSettings).toHaveBeenCalledWith(
    expect.objectContaining({ preferredProfileId: 'fixture-account', model: 'fixture-model' }),
    snapshot.revision,
  )
  wrapper.unmount()
})

it('keeps the selected login and uses the new revision after synchronizing models', async () => {
  const snapshot = unavailableGateway()
  snapshot.available = true
  snapshot.revision = 'before'
  snapshot.profiles = [
    {
      id: 'fixture',
      kind: 'codex',
      name: '已登录账号',
      email: null,
      enabled: true,
      priority: 100,
      baseUrl: 'https://chatgpt.com/backend-api/codex',
      authStatus: 'ready',
      models: [],
      cooldownUntil: null,
      lastError: null,
      requests: 0,
      inputTokens: 0,
      outputTokens: 0,
      createdAt: 1,
    },
  ]
  const refreshed = {
    ...snapshot,
    revision: 'after',
    profiles: [{ ...snapshot.profiles[0]!, models: [{ id: 'synced-model', name: '同步模型' }] }],
  }
  const saveSettings = vi.fn(async (settings: GatewaySettings) => ({ ...refreshed, settings }))
  const service = {
    snapshot: async () => snapshot,
    refreshModels: async () => refreshed,
    saveSettings,
  } as unknown as GatewayService
  const wrapper = mount(Connections, {
    props: { service },
    global: { plugins: [createPinia()], stubs: { RouterLink: true } },
  })
  await flushPromises()
  await wrapper.find('select').setValue('fixture')
  await wrapper
    .findAll('button')
    .find((button) => button.text() === '同步此账号模型')!
    .trigger('click')
  await flushPromises()
  await wrapper
    .findAll('button')
    .find((button) => button.text() === '保存路由设置')!
    .trigger('click')
  await flushPromises()
  expect(saveSettings).toHaveBeenCalledWith(
    expect.objectContaining({ preferredProfileId: 'fixture', model: 'synced-model' }),
    'after',
  )
  wrapper.unmount()
})
