<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import Icon from '../../shared/components/Icon.vue'
import Modal from '../../shared/components/Modal.vue'
import type { CodexAccount, ResetCredit } from '../../types/accounts'
import { countdown, fullDate } from './time'
const props = defineProps<{ account: CodexAccount; loading: boolean; error: string }>()
const emit = defineEmits<{ close: []; refresh: [] }>()
const now = ref(Date.now())
let timer: ReturnType<typeof setInterval> | undefined
onMounted(() => {
  timer = setInterval(() => {
    now.value = Date.now()
  }, 30_000)
})
onUnmounted(() => clearInterval(timer))
const summary = computed(() => props.account.quota?.resetCredits)
const count = computed(() => summary.value?.availableCount)
const credits = computed(() => summary.value?.credits)
const nextExpiry = computed(() => {
  const times = (credits.value ?? [])
    .filter((credit) => credit.status === 'available' && credit.expiresAt != null)
    .map((credit) => credit.expiresAt as number)
  const future = times.filter((time) => time > now.value)
  return future.length ? Math.min(...future) : times.length ? Math.max(...times) : null
})
const urgent = computed(
  () => nextExpiry.value != null && nextExpiry.value - now.value <= 12 * 3_600_000,
)
function status(credit: ResetCredit) {
  if (credit.status === 'available' && credit.expiresAt != null && credit.expiresAt <= now.value)
    return '已到期 · 待刷新'
  return (
    (
      { available: '可用', redeeming: '处理中', redeemed: '已使用', expired: '已到期' } as Record<
        string,
        string
      >
    )[credit.status] ?? '状态未知'
  )
}
</script>
<template>
  <Modal title="预览重置卡" @close="emit('close')">
    <section class="reset-credit-preview" :aria-busy="loading">
      <p class="reset-credit-account">{{ account.email ?? account.name }}</p>
      <div class="reset-credit-summary">
        <div>
          <span>可用重置卡</span
          ><strong>{{ count == null ? '—' : count }}<small> 张</small></strong>
        </div>
        <div :class="{ urgent }">
          <span>最近到期</span
          ><b>{{
            nextExpiry == null
              ? credits?.some((c) => c.status === 'available' && c.noExpiration)
                ? '无到期限制'
                : '尚无时间'
              : countdown(nextExpiry, now)
          }}</b
          ><small v-if="nextExpiry != null">{{ fullDate(nextExpiry) }}</small>
        </div>
      </div>
      <p class="reset-credit-explanation">
        重置卡用于恢复 Codex
        额度。这里仅预览，不会使用或消耗重置卡。数量以接口返回为准，明细可能只包含部分记录。
      </p>
      <p v-if="error" class="error-state" role="alert">{{ error }} 上次快照已保留。</p>
      <div v-if="credits == null" class="reset-credit-empty">
        <Icon name="ticket" :size="28" />
        <b>{{ count == null ? '尚未取得重置卡信息' : '已获取数量，尚无明细' }}</b>
        <p>点击“刷新详情”读取此账号的重置卡数量、有效期和状态。</p>
      </div>
      <div v-else-if="!credits.length" class="reset-credit-empty">
        <Icon name="ticket" :size="28" /><b>接口未返回重置卡明细</b>
        <p>{{ count ? '可用数量与明细独立返回，请以顶部数量为准。' : '目前没有可展示的记录。' }}</p>
      </div>
      <ol v-else class="reset-credit-list">
        <li v-for="credit in credits" :key="credit.id">
          <div class="reset-credit-row-title">
            <b>{{ credit.title || 'Codex 额度重置卡' }}</b
            ><span
              :class="{
                available:
                  credit.status === 'available' &&
                  (credit.noExpiration || (credit.expiresAt != null && credit.expiresAt > now)),
              }"
              >{{ status(credit) }}</span
            >
          </div>
          <strong class="reset-credit-countdown">{{
            credit.noExpiration
              ? '无到期限制'
              : credit.expiresAt == null
                ? '有效期未知'
                : countdown(credit.expiresAt, now)
          }}</strong>
          <dl class="quota-preview-facts">
            <div>
              <dt>到期时间</dt>
              <dd>{{ credit.noExpiration ? '无到期限制' : fullDate(credit.expiresAt) }}</dd>
            </div>
            <div>
              <dt>发放时间</dt>
              <dd>{{ fullDate(credit.grantedAt) }}</dd>
            </div>
          </dl>
          <p v-if="credit.description" class="reset-credit-description">{{ credit.description }}</p>
        </li>
      </ol>
      <div class="reset-credit-updated">
        <span>数量更新：{{ summary ? fullDate(account.quota?.checkedAt) : '尚未获取' }}</span
        ><span>明细更新：{{ fullDate(summary?.detailsCheckedAt) }}</span>
      </div>
      <div class="modal-actions">
        <button class="button" :disabled="loading" @click="emit('close')">关闭</button
        ><button class="button primary" :disabled="loading" @click="emit('refresh')">
          <Icon name="refresh" :class="{ spinning: loading }" />{{
            loading ? '读取中…' : '刷新详情'
          }}
        </button>
      </div>
    </section>
  </Modal>
</template>
