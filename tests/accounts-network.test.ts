import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount, type VueWrapper } from '@vue/test-utils'
import Dialog from '../src/features/accounts/ApiNetworkDialog.vue'
import { accountsService } from '../src/services/accounts'
import type { ApiNetworkProbe, ApiNetworkView } from '../src/types/accounts'
vi.mock('../src/services/accounts', () => ({
  accountsService: { saveNetwork: vi.fn(), detectNetwork: vi.fn(), testNetwork: vi.fn() },
}))
const initial: ApiNetworkView = {
  revision: 'r1',
  settings: { mode: 'system', url: null },
  subscription: null,
}
let wrapper: VueWrapper | undefined
beforeEach(() => {
  vi.resetAllMocks()
  HTMLDialogElement.prototype.showModal = function () {
    this.setAttribute('open', '')
  }
  vi.mocked(accountsService.detectNetwork).mockResolvedValue({
    candidates: ['http://127.0.0.1:7890/'],
    message: '仅填入草稿，不保存',
  })
  vi.mocked(accountsService.testNetwork).mockResolvedValue({
    code: 'http_response',
    reachable: true,
    statusCode: 403,
    elapsedMs: 10,
    message: '网络响应不代表订阅成功',
  })
  vi.mocked(accountsService.saveNetwork).mockResolvedValue({ ...initial, revision: 'r2' })
})
afterEach(() => {
  wrapper?.unmount()
  wrapper = undefined
  document.body.innerHTML = ''
})
async function open() {
  wrapper = mount(Dialog, { props: { value: structuredClone(initial) }, attachTo: document.body })
  await flushPromises()
  return wrapper
}
function button(name: string) {
  return wrapper!.findAll('button').find((b) => b.text() === name)!
}
describe('subscription API network confirmation', () => {
  it('opening and cancelling never probes or persists', async () => {
    await open()
    expect(accountsService.testNetwork).not.toHaveBeenCalled()
    expect(accountsService.detectNetwork).not.toHaveBeenCalled()
    expect(accountsService.saveNetwork).not.toHaveBeenCalled()
    await button('取消').trigger('click')
    expect(wrapper!.emitted('close')).toHaveLength(1)
  })
  it('detects into draft only; testing never saves; save does not send a request', async () => {
    await open()
    await button('一键检测').trigger('click')
    await flushPromises()
    expect(wrapper!.get('[aria-label="订阅 API 代理地址"]').element.value).toBe(
      'http://127.0.0.1:7890/',
    )
    expect(wrapper!.text()).toContain('仅填入草稿')
    expect(accountsService.saveNetwork).not.toHaveBeenCalled()
    expect(accountsService.testNetwork).not.toHaveBeenCalled()
    await button('测试').trigger('click')
    await flushPromises()
    expect(accountsService.testNetwork).toHaveBeenCalledWith({
      mode: 'manual',
      url: 'http://127.0.0.1:7890/',
    })
    expect(wrapper!.text()).toContain('网络响应不代表订阅成功')
    expect(accountsService.saveNetwork).not.toHaveBeenCalled()
    await button('保存').trigger('click')
    await flushPromises()
    expect(accountsService.saveNetwork).toHaveBeenCalledWith(
      { mode: 'manual', url: 'http://127.0.0.1:7890/' },
      'r1',
    )
    expect(accountsService.testNetwork).toHaveBeenCalledOnce()
    expect(wrapper!.emitted('saved')).toHaveLength(1)
  })
  it('rejects credentials and nonproxy URLs and clears prior test when draft changes', async () => {
    await open()
    await wrapper!.get('[aria-label="订阅 API 代理模式"]').setValue('manual')
    await wrapper!
      .get('[aria-label="订阅 API 代理地址"]')
      .setValue('http://user:secret@localhost:1')
    expect(button('保存').attributes('disabled')).toBeDefined()
    expect(button('测试').attributes('disabled')).toBeDefined()
    await wrapper!.get('[aria-label="订阅 API 代理地址"]').setValue('http://localhost:7890')
    await button('测试').trigger('click')
    await flushPromises()
    expect(wrapper!.find('.account-network-result').exists()).toBe(true)
    await wrapper!.get('[aria-label="订阅 API 代理地址"]').setValue('http://localhost:7891')
    expect(wrapper!.find('.account-network-result').exists()).toBe(false)
  })
  it('late test reply cannot overwrite a changed draft or unmounted dialog', async () => {
    let resolve!: (r: ApiNetworkProbe) => void
    vi.mocked(accountsService.testNetwork).mockReturnValueOnce(
      new Promise((r) => {
        resolve = r
      }),
    )
    await open()
    await wrapper!.get('[aria-label="订阅 API 代理模式"]').setValue('manual')
    await wrapper!.get('[aria-label="订阅 API 代理地址"]').setValue('http://localhost:7890')
    await button('测试').trigger('click')
    await wrapper!.get('[aria-label="订阅 API 代理地址"]').setValue('http://localhost:7891')
    resolve({
      code: 'reachable',
      reachable: true,
      statusCode: 200,
      elapsedMs: 1,
      message: 'old reply',
    })
    await flushPromises()
    expect(wrapper!.text()).not.toContain('old reply')
    expect(wrapper!.emitted('saved')).toBeUndefined()
  })
  it('save conflict keeps draft and displays the real error', async () => {
    vi.mocked(accountsService.saveNetwork).mockRejectedValueOnce(Error('设置已变化'))
    await open()
    await button('保存').trigger('click')
    await flushPromises()
    expect(wrapper!.text()).toContain('设置已变化')
    expect(wrapper!.emitted('saved')).toBeUndefined()
    expect(accountsService.testNetwork).not.toHaveBeenCalled()
  })
})
