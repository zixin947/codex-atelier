import { mount, flushPromises } from '@vue/test-utils'
import { beforeEach, expect, it, vi } from 'vitest'
const { invoke } = vi.hoisted(() => ({ invoke: vi.fn() }))
vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => true, invoke }))
vi.mock('@tauri-apps/api/event', () => ({ listen: vi.fn(async () => () => {}) }))
import DesktopPets from '../src/features/desktop-pets/DesktopPets.vue'
beforeEach(() => {
  invoke.mockReset()
  invoke.mockImplementation((_command, { action }) =>
    Promise.resolve(
      ['status', 'refresh-quota'].includes(action)
        ? { version: '0.3.0', installed: true, running: true }
        : { ok: true },
    ),
  )
})
it('queries status and uses the upstream desktop action', async () => {
  const page = mount(DesktopPets)
  await flushPromises()
  await page
    .findAll('button')
    .find((b) => b.text() === '独立桌面')!
    .trigger('click')
  await flushPromises()
  expect(invoke).toHaveBeenCalledWith('desktop_pet_action', { action: 'desktop' })
  expect(page.text()).toContain('运行中')
})
it('requires installation acknowledgement and exposes failures', async () => {
  const page = mount(DesktopPets)
  await flushPromises()
  expect(page.findAll('button').some((b) => b.text() === '确认安装')).toBe(false)
  await page
    .findAll('button')
    .find((b) => b.text() === '重新安装')!
    .trigger('click')
  const install = page.findAll('button').find((b) => b.text() === '确认安装')!
  invoke.mockRejectedValueOnce('安装失败')
  await install.trigger('click')
  await flushPromises()
  expect(page.find('[role="alert"]').text()).toBe('安装失败')
  expect(invoke).toHaveBeenCalledWith('desktop_pet_action', { action: 'install' })
})

it('refreshes quota through Atelier Accounts', async () => {
  const page = mount(DesktopPets)
  await flushPromises()
  await page
    .findAll('button')
    .find((b) => b.attributes('aria-label') === '刷新状态与额度')!
    .trigger('click')
  await flushPromises()
  expect(invoke).toHaveBeenCalledWith('desktop_pet_action', { action: 'refresh-quota' })
})
