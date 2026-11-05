<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import type { ThreadDetail } from '../../types/threads'
import { compact } from './model'
const props = defineProps<{ detail: ThreadDetail }>()
const active = ref<number | null>(null)
watch(
  () => props.detail.id,
  () => (active.value = null),
)
const points = computed(() => props.detail.timeline ?? [])
const maximum = computed(() => Math.max(1, ...points.value.map((p) => p.total)))
const x = (i: number) => 36 + (points.value.length > 1 ? i / (points.value.length - 1) : 0.5) * 220
const y = (v: number) => 118 - (v / maximum.value) * 100
const line = computed(() => points.value.map((p, i) => `${x(i)},${y(p.total)}`).join(' '))
const area = computed(() =>
  points.value.length ? `${x(0)},118 ${line.value} ${x(points.value.length - 1)},118` : '',
)
const labels = computed(() =>
  [...new Set([0, Math.floor((points.value.length - 1) / 2), points.value.length - 1])].filter(
    (i) => i >= 0,
  ),
)
</script>
<template>
  <section class="thread-history">
    <div class="thread-section-label">
      <b>累计用量</b><span>{{ points.length }} 个记录日期 · UTC</span>
    </div>
    <svg
      v-if="points.length"
      viewBox="0 0 270 150"
      role="img"
      aria-label="按记录日期显示的累计 Token 用量曲线"
    >
      <g v-for="n in [0, 1, 2, 3, 4]" :key="n">
        <line x1="36" x2="256" :y1="118 - n * 25" :y2="118 - n * 25" class="history-grid" />
        <text x="29" :y="121 - n * 25" text-anchor="end">{{ compact((maximum * n) / 4) }}</text>
      </g>
      <polygon :points="area" class="history-area" />
      <polyline :points="line" class="history-line" />
      <circle
        v-for="(p, i) in points"
        :key="p.date"
        :cx="x(i)"
        :cy="y(p.total)"
        r="4"
        tabindex="0"
        :aria-label="`${p.date}：${p.total} Token`"
        @focus="active = i"
        @blur="active = null"
        @mouseenter="active = i"
        @mouseleave="active = null"
      >
        <title>{{ p.date }} · {{ p.total.toLocaleString() }} Token</title>
      </circle>
      <text v-for="i in labels" :key="i" :x="x(i)" y="140" text-anchor="middle">
        {{ points[i]?.date.slice(5) }}
      </text>
    </svg>
    <p v-else class="thread-detail-note">暂无带日期的用量记录。</p>
    <p v-if="active !== null" class="history-readout" role="status">
      {{ points[active]?.date }} · {{ points[active]?.total.toLocaleString() }} Token
    </p>
    <div class="thread-section-label history-model-heading">
      <b>历史模型</b><span>按记录中的用量增量归属</span>
    </div>
    <div v-for="m in detail.modelUsage" :key="m.model" class="history-model">
      <span>{{ m.model }}</span
      ><strong>{{ compact(m.total) }}</strong>
    </div>
    <p v-if="!detail.modelUsage?.length" class="thread-detail-note">暂无可归属的模型用量。</p>
  </section>
</template>
<style scoped>
.thread-history {
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--thread-border);
}
.thread-section-label {
  gap: 8px;
  font-size: 10px;
}
.thread-section-label b {
  color: var(--ink);
}
svg {
  width: 100%;
  display: block;
  margin: 16px 0 0;
  overflow: visible;
}
svg text {
  fill: var(--muted);
  font: 9px var(--font-mono);
}
.history-grid {
  stroke: var(--thread-border);
  stroke-width: 1;
}
.history-line {
  fill: none;
  stroke: var(--thread-teal);
  stroke-width: 2;
  stroke-linejoin: round;
}
.history-area {
  fill: var(--thread-teal);
  opacity: 0.1;
}
circle {
  fill: var(--thread-teal);
  stroke: var(--paper);
  stroke-width: 2;
}
circle:focus {
  outline: none;
  stroke: var(--ink);
  stroke-width: 3;
}
.history-readout {
  font: 10px var(--font-mono);
  color: var(--ink);
}
.history-model-heading {
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid var(--thread-border);
}
.history-model {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 18px 0;
  border-bottom: 1px solid var(--thread-border);
  font-size: 11px;
}
.history-model span {
  overflow-wrap: anywhere;
}
.history-model strong {
  font-family: var(--font-mono);
  white-space: nowrap;
}
</style>
