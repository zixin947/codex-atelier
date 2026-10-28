import { afterEach, describe, expect, it } from 'vitest'
import { mount, type VueWrapper } from '@vue/test-utils'
import QuotaMeter from '../src/features/accounts/QuotaMeter.vue'

const now = 1_800_000_000_000
const wrappers: VueWrapper[] = []
function meter(usedPercent: number | null, extra = {}) {
  const wrapper = mount(QuotaMeter, {
    attachTo: document.body,
    props: {
      window: { usedPercent, durationMins: 300, resetsAt: now + 7_200_000 },
      fallback: '5小时',
      checkedAt: now,
      now,
      ...extra,
    },
  })
  wrappers.push(wrapper)
  return wrapper
}
afterEach(() => wrappers.splice(0).forEach((wrapper) => wrapper.unmount()))

describe('quota presentation', () => {
  it.each([
    [0, 'healthy', '充足'],
    [50, 'healthy', '充足'],
    [50.1, 'medium', '留意'],
    [80, 'medium', '留意'],
    [80.1, 'low', '偏低'],
    [99.9, 'low', '偏低'],
    [100, 'exhausted', '已用尽'],
  ])('grades used %s at exact thresholds', (used, state, label) => {
    const wrapper = meter(used)
    expect(wrapper.get('button.account-quota').attributes('data-quota-state')).toBe(state)
    expect(wrapper.get('.account-quota-grade').text()).toBe(label)
    expect(Number(wrapper.get('[role="progressbar"]').attributes('aria-valuenow'))).toBe(100 - used)
  })

  it('keeps zero, missing data and invalid numbers distinct', async () => {
    const wrapper = meter(100)
    expect(wrapper.get('.account-quota-ring strong').text()).toBe('0%已用尽')
    expect(wrapper.find('.account-ring-value').exists()).toBe(false)
    for (const usedPercent of [null, Number.NaN, Number.POSITIVE_INFINITY]) {
      await wrapper.setProps({ window: { usedPercent, durationMins: 300, resetsAt: null } })
      expect(wrapper.get('button.account-quota').attributes('data-quota-state')).toBe('unknown')
      expect(wrapper.find('[role="progressbar"]').exists()).toBe(false)
      expect(wrapper.get('.account-quota-ring strong').text()).toBe('—未知')
    }
    await wrapper.setProps({ window: null, stale: true })
    expect(wrapper.get('.account-quota-grade').text()).toBe('未提供')
  })

  it('shows tiny positive balances without calling them zero or exhausted', () => {
    const wrapper = meter(99.9)
    expect(wrapper.get('.account-quota-ring strong').text()).toBe('<1%偏低')
    expect(wrapper.get('.account-ring-value').attributes('stroke-dasharray')).not.toBe('0 100')
    expect(wrapper.get('[role="progressbar"]').attributes('aria-valuetext')).toContain('不足1%')
  })

  it('clamps finite out-of-range readings without overflowing the circle', () => {
    expect(meter(-5).get('[role="progressbar"]').attributes('aria-valuenow')).toBe('100')
    expect(meter(105).get('[role="progressbar"]').attributes('aria-valuenow')).toBe('0')
  })

  it('prioritizes loading and failures over old snapshot grade, then recovers', async () => {
    const wrapper = meter(100, { stale: true })
    expect(wrapper.get('.account-quota-ring strong').text()).toBe('0%旧快照')
    await wrapper.setProps({ failed: true })
    expect(wrapper.get('button.account-quota').attributes('data-quota-state')).toBe('failed')
    expect(wrapper.get('.account-quota-ring strong').text()).toBe('0%读取失败')
    await wrapper.setProps({ loading: true })
    expect(wrapper.get('button.account-quota').attributes('data-quota-state')).toBe('loading')
    expect(wrapper.get('button.account-quota').attributes('aria-busy')).toBe('true')
    expect(wrapper.find('.account-ring-loading').exists()).toBe(true)
    await wrapper.setProps({ loading: false, failed: false, stale: false })
    expect(wrapper.get('button.account-quota').attributes('data-quota-state')).toBe('exhausted')
    expect(wrapper.get('button.account-quota').attributes('aria-busy')).toBeUndefined()
    expect(wrapper.find('.account-ring-loading').exists()).toBe(false)
  })

  it('retains the old percentage and explains a failed read in keyboard preview', async () => {
    const wrapper = meter(15, { failed: true, stale: true })
    expect(wrapper.get('.account-quota-ring strong').text()).toBe('85%读取失败')
    await wrapper.get('button.account-quota').trigger('focus')
    const tooltip = document.querySelector('[role="tooltip"]')!
    expect(tooltip.textContent).toContain('保留上次快照')
    expect(tooltip.textContent).toContain('85%')
    expect(tooltip.textContent).toContain('重置时间')
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }))
    await wrapper.vm.$nextTick()
    expect(document.querySelector('[role="tooltip"]')).toBeNull()
  })

  it('does not invent a previous snapshot during the first loading or failed read', async () => {
    const wrapper = meter(null, { window: null, loading: true })
    await wrapper.get('button.account-quota').trigger('focus')
    expect(document.querySelector('[role="tooltip"]')!.textContent).toContain('尚无可显示的读数')
    await wrapper.setProps({ loading: false, failed: true })
    expect(wrapper.get('.account-quota-ring strong').text()).toBe('—读取失败')
    expect(document.querySelector('[role="tooltip"]')!.textContent).toContain('无法判断剩余额度')
    expect(wrapper.find('[role="progressbar"]').exists()).toBe(false)
  })
})
