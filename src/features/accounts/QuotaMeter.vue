<script setup lang="ts">
import { computed } from 'vue'
import HoverPreview from '../../shared/components/HoverPreview.vue'
import { countdown, fullDate } from './time'
import type { QuotaWindow } from '../../types/accounts'
const props = defineProps<{
  window: QuotaWindow | null
  fallback: string
  stale?: boolean
  loading?: boolean
  failed?: boolean
  checkedAt?: number
  now: number
}>()
const remaining = computed(() =>
  props.window?.usedPercent == null || !Number.isFinite(props.window.usedPercent)
    ? null
    : Math.max(0, Math.min(100, 100 - props.window.usedPercent)),
)
const band = computed(() => {
  const value = remaining.value
  if (value === null) return 'unknown'
  if (value === 0) return 'exhausted'
  if (value < 20) return 'low'
  if (value < 50) return 'medium'
  return 'healthy'
})
const state = computed(() =>
  props.loading
    ? 'loading'
    : props.failed
      ? 'failed'
      : remaining.value === null
        ? 'unknown'
        : props.stale
          ? 'stale'
          : band.value,
)
const status = computed(() => {
  switch (state.value) {
    case 'loading':
      return '刷新中'
    case 'failed':
      return '读取失败'
    case 'stale':
      return '旧快照'
    case 'unknown':
      return props.window ? '未知' : '未提供'
    case 'exhausted':
      return '已用尽'
    case 'low':
      return '偏低'
    case 'medium':
      return '留意'
    default:
      return '充足'
  }
})
const note = computed(() => {
  if (props.loading)
    return remaining.value === null
      ? '正在查询额度，尚无可显示的读数。'
      : '正在刷新，当前数字为上次快照。'
  if (props.failed)
    return remaining.value === null
      ? '本次读取失败，无法判断剩余额度；请重试。'
      : '本次读取失败，保留上次快照；请重试后确认实际额度。'
  if (!props.window) return '接口尚未提供此窗口，无法判断额度或计算重置时间。'
  if (remaining.value === null) return '接口未返回有效的额度读数；未知不表示已用尽。'
  if (props.stale) return '这是上次快照，请刷新后确认实际额度。'
  return '倒计时按接口重置时间计算；实际额度以刷新结果为准。'
})
const name = computed(() => {
  const mins = props.window?.durationMins
  return mins === 300
    ? '5小时'
    : mins === 10080
      ? '1周'
      : mins
        ? mins % 60 === 0
          ? String(mins / 60) + '小时'
          : String(mins) + '分钟'
        : props.fallback
})
const reset = computed(() =>
  props.window?.resetsAt
    ? new Date(props.window.resetsAt).toLocaleString('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        hour12: false,
      })
    : '—',
)
</script>
<template>
  <HoverPreview
    :label="name + '额度：' + status + '，查看详情与重置倒计时'"
    class="account-quota"
    :class="[band, state]"
    :data-quota-state="state"
    :aria-busy="loading || undefined"
  >
    <span
      class="account-quota-ring"
      :role="remaining === null ? 'img' : 'progressbar'"
      :aria-label="name + '剩余额度：' + status"
      :aria-valuetext="
        remaining === null
          ? undefined
          : (remaining > 0 && remaining < 1 ? '不足1%' : remaining.toFixed(1) + '%') + '，' + status
      "
      :aria-valuenow="remaining === null ? undefined : remaining"
      :aria-valuemin="remaining === null ? undefined : 0"
      :aria-valuemax="remaining === null ? undefined : 100"
    >
      <svg viewBox="0 0 64 64" aria-hidden="true">
        <circle class="account-ring-track" cx="32" cy="32" r="28" pathLength="100" />
        <circle
          v-if="remaining !== null && remaining > 0"
          class="account-ring-value"
          cx="32"
          cy="32"
          r="28"
          pathLength="100"
          :stroke-dasharray="remaining + ' 100'"
        />
        <circle
          v-if="loading"
          class="account-ring-loading"
          cx="32"
          cy="32"
          r="28"
          pathLength="100"
          stroke-dasharray="22 78"
        />
      </svg>
      <strong aria-hidden="true"
        ><span
          >{{
            remaining === null
              ? '—'
              : remaining > 0 && remaining < 1
                ? '<1'
                : Math.round(remaining)
          }}<small v-if="remaining !== null">%</small></span
        ><em class="account-quota-grade">{{ status }}</em></strong
      >
    </span>
    <span class="account-quota-caption" :title="note + ' ' + name + '重置时间：' + reset">
      <b>{{ name }}</b
      ><span> · </span><span>{{ loading ? '刷新中' : reset }}</span
      ><span v-if="!loading && window?.resetsAt"> 重置</span>
    </span>
    <template #preview>
      <div class="quota-preview-heading">
        <b>{{ name }}额度</b><span>重置倒计时</span>
      </div>
      <strong class="quota-preview-countdown">{{ countdown(window?.resetsAt, now) }}</strong>
      <dl class="quota-preview-facts">
        <div>
          <dt>读数状态</dt>
          <dd>{{ status }}</dd>
        </div>
        <div>
          <dt>剩余额度</dt>
          <dd>
            {{
              remaining === null
                ? '未知'
                : remaining > 0 && remaining < 1
                  ? '<1%'
                  : Number(remaining.toFixed(1)) + '%'
            }}
          </dd>
        </div>
        <div>
          <dt>重置时间</dt>
          <dd>{{ fullDate(window?.resetsAt) }}</dd>
        </div>
        <div>
          <dt>最近更新</dt>
          <dd>{{ fullDate(checkedAt) }}</dd>
        </div>
      </dl>
      <p class="quota-preview-note">{{ note }}</p>
    </template>
  </HoverPreview>
</template>
