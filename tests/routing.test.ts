import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises, mount, type VueWrapper } from '@vue/test-utils'
import Routing from '../src/features/routing/Routing.vue'
import type { RoutingSnapshot } from '../src/types/routing'

let wrapper: VueWrapper | undefined
beforeEach(() => {
  vi.useFakeTimers()
  vi.stubGlobal(
    'ResizeObserver',
    class {
      observe() {}
      disconnect() {}
    },
  )
})
afterEach(() => {
  wrapper?.unmount()
  wrapper = undefined
  vi.useRealTimers()
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
})
function snapshot(): RoutingSnapshot {
  return {
    runtime: {
      state: 'running',
      address: 'http://127.0.0.1:18910/v1',
      activeRequests: 0,
      completedRequests: 1,
      failedRequests: 0,
      message: null,
    },
    reroutedRequests: 1,
    routes: [
      {
        id: 'request-a',
        at: 1800000000000,
        model: 'fixture-model',
        endpoint: 'responses',
        status: 'completed',
        code: 'response.completed',
        durationMs: 25,
        candidates: [
          { profileId: 'a', name: 'private-first-account', eligible: true, reason: '当前首选账号' },
          {
            profileId: 'b',
            name: 'private-second-account',
            eligible: true,
            reason: '备用账号 · 优先级 100',
          },
        ],
        attempts: [
          {
            profileId: 'a',
            startedAt: 1800000000000,
            durationMs: 10,
            httpStatus: 429,
            code: 'rate_limited',
          },
          {
            profileId: 'b',
            startedAt: 1800000000010,
            durationMs: 15,
            httpStatus: 200,
            code: 'response.completed',
          },
        ],
      },
    ],
  }
}
function open(service: () => Promise<RoutingSnapshot | null>) {
  wrapper = mount(Routing, {
    props: { service },
    global: { stubs: { RouterLink: { template: '<a><slot /></a>' } } },
  })
  return wrapper
}
describe('Atelier 路由观测', () => {
  it('shows candidate reasons, attempts and counters with account identities masked by default', async () => {
    const w = open(vi.fn().mockResolvedValue(snapshot()))
    await flushPromises()
    expect(w.text()).toContain('当前首选账号')
    expect(w.text()).toContain('HTTP 429')
    expect(w.text()).toContain('改道到下一候选')
    expect(w.html()).not.toContain('private-first-account')
    expect(w.findAll('.rt-stats b').map((e) => e.text())).toEqual(['1', '1', '0'])
    await w.get('[aria-pressed="true"].button').trigger('click')
    expect(w.text()).toContain('private-first-account')
    await w.get('select').setValue('failed')
    expect(w.text()).toContain('没有符合筛选条件的请求')
  })
  it('shows in-flight state and follows completion on the next poll', async () => {
    const active = snapshot()
    active.routes[0]!.status = 'forwarding'
    active.routes[0]!.durationMs = null
    active.routes[0]!.attempts[1]!.durationMs = null
    const service = vi.fn().mockResolvedValueOnce(active).mockResolvedValue(snapshot())
    const w = open(service)
    await flushPromises()
    expect(w.text()).toContain('正在响应')
    await vi.advanceTimersByTimeAsync(1000)
    await flushPromises()
    expect(w.text()).toContain('响应完成')
  })
  it('pauses polling when hidden or unmounted and reports stale data on read failure', async () => {
    const service = vi.fn().mockResolvedValueOnce(snapshot()).mockRejectedValue(Error('offline'))
    const w = open(service)
    await flushPromises()
    vi.spyOn(document, 'hidden', 'get').mockReturnValue(true)
    document.dispatchEvent(new Event('visibilitychange'))
    await vi.advanceTimersByTimeAsync(3000)
    expect(service).toHaveBeenCalledTimes(1)
    vi.spyOn(document, 'hidden', 'get').mockReturnValue(false)
    document.dispatchEvent(new Event('visibilitychange'))
    await flushPromises()
    expect(w.get('[role="alert"]').text()).toContain('当前内容可能已过期')
    expect(w.text()).toContain('fixture-model')
    w.unmount()
    wrapper = undefined
    await vi.advanceTimersByTimeAsync(3000)
    expect(service).toHaveBeenCalledTimes(2)
  })
  it('does not present browser preview as a running gateway', async () => {
    const w = open(async () => null)
    await flushPromises()
    expect(w.text()).toContain('浏览器预览不连接本机数据')
    expect(w.text()).toContain('等待请求')
    expect(w.find('.routing-status i.on').exists()).toBe(false)
  })
})
