<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import Icon from '../components/Icon.vue'
import HudMap from './HudMap.vue'

// Layout and rounded / circular-sector outlines ported from EndfieldHUD's
// HUDNavigation.swift (MIT, Copyright (c) 2026 DDDuoDuo).
const props = defineProps<{
  items: { path: string; label: string; icon: string }[]
  expanded: boolean
}>()
const emit = defineEmits<{ open: []; overview: [] }>()
const route = useRoute()
const clock = ref('')
let clockTimer: ReturnType<typeof setInterval>
function updateClock() {
  clock.value = new Date().toLocaleTimeString('zh-CN', { hour12: false, timeZone: 'Asia/Shanghai' })
}
onMounted(() => {
  updateClock()
  clockTimer = setInterval(updateClock, 1000)
})
onUnmounted(() => clearInterval(clockTimer))
function layout(index: number) {
  if (index < 4)
    return {
      x: [54, 16, 22, 60][index]!,
      y: 147 + index * 95,
      w: 180,
      h: 84,
      slant: [9, 5, 1, -8][index]!,
      group: 'left',
    }
  if (index < 6)
    return { x: index === 4 ? 350 : 506, y: 463, w: 144, h: 65, slant: 0, group: 'bottom' }
  const slot = index - 6,
    y = 174 + Math.floor(slot / 2) * 90
  return {
    x: 508 + Math.sqrt(306 ** 2 - (y + 37 - 320) ** 2) + (slot % 2) * 84,
    y,
    w: 78,
    h: 74,
    slant: -2,
    group: 'right',
  }
}
function platePath(w: number, h: number, slant: number, group: string) {
  if (group === 'bottom') {
    const outer = -143 + Math.sqrt(208 ** 2 - (w + 6) ** 2)
    const inner = -143 + Math.sqrt(208 ** 2 - 6 ** 2)
    return `M0 ${outer} L${w * 0.62} 3 L${w - 2} 8 Q${w} 8 ${w} 10 L${w} ${inner} A208 208 0 0 1 0 ${outer} Z`
  }
  const m = 1.5,
    radius = h > 65 ? 4 : 3
  const points: [number, number][] = [
    [m + Math.max(slant, 0), m],
    [w - m + Math.min(slant, 0), m],
    [w - m - Math.max(slant, 0), h - m],
    [m - Math.min(slant, 0), h - m],
  ]
  return (
    points
      .map(([x, y], i) => {
        const previous = points[(i + 3) % 4]!,
          next = points[(i + 1) % 4]!
        const a = radius / Math.hypot(previous[0] - x, previous[1] - y),
          b = radius / Math.hypot(next[0] - x, next[1] - y)
        return `${i ? 'L' : 'M'}${x + (previous[0] - x) * a} ${y + (previous[1] - y) * a} Q${x} ${y} ${x + (next[0] - x) * b} ${y + (next[1] - y) * b}`
      })
      .join(' ') + ' Z'
  )
}
const tiles = computed(() =>
  props.items.map((item, index) => {
    const box = layout(index)
    return { ...item, ...box, index, outline: platePath(box.w, box.h, box.slant, box.group) }
  }),
)
</script>

<template>
  <aside class="hud-navigation">
    <div class="hud-orbit-plane" aria-hidden="true">
      <svg class="hud-orbits" viewBox="0 0 1000 640" fill="none" aria-hidden="true">
        <circle cx="500" cy="320" r="306" stroke="currentColor" opacity=".25" />
        <circle cx="500" cy="320" r="292" stroke="currentColor" opacity=".5" />
        <circle cx="500" cy="320" r="278" stroke="#151818" stroke-width="14" />
        <circle cx="500" cy="320" r="266" stroke="currentColor" opacity=".32" />
        <g class="hud-ring-detail">
          <g class="hud-ring-rotor">
            <circle
              cx="500"
              cy="320"
              r="287"
              stroke="currentColor"
              stroke-dasharray="1 14"
              stroke-width="3"
              opacity=".4"
            />
          </g>
        </g>
        <g class="hud-ring-detail">
          <g class="hud-ring-mesh">
            <circle
              cx="500"
              cy="320"
              r="258"
              stroke="#e7df54"
              stroke-dasharray="180 400 65 450"
              stroke-width="2"
              opacity=".7"
            />
          </g>
        </g>
        <circle
          cx="500"
          cy="320"
          r="242"
          stroke="currentColor"
          stroke-dasharray="230 120 70 340"
          stroke-width="5"
          opacity=".28"
        />
        <circle cx="500" cy="320" r="214" stroke="#e4e5e0" opacity=".6" />
        <path
          class="hud-indicator-glow"
          d="M484 46l16-17 16 17z M484 594l16 17 16-17z"
          stroke="#efdf47"
        />
        <path
          d="M207 143l-73-27-73 94 29 15M203 489l-71 30-66-84 29-15M808 142l96-28 57 122-39 15"
          stroke="currentColor"
          opacity=".28"
        />
        <path
          v-for="n in 5"
          :key="n"
          :d="`M180 ${315 + n * 3}H230M760 ${315 + n * 3}H828`"
          stroke="currentColor"
          opacity=".28"
        />
      </svg>
    </div>
    <HudMap v-if="!expanded" />
    <div v-if="!expanded" class="hud-stage-brand">
      <b>ATELIER HUD</b><small>PERSONAL CONTROL INTERFACE</small>
    </div>
    <div v-if="!expanded" class="hud-stage-clock">
      <small>WORKSTATION TIME</small><b>{{ clock }}</b
      ><span>LOCAL / UTC+08</span>
    </div>
    <div v-if="!expanded" class="hud-stage-identity">
      <span>A</span>
      <div><b>CODEX ATELIER</b><small>本地工作站 / PERSONAL WORKSPACE</small></div>
    </div>
    <div class="hud-nav-heading">
      <span>MODULE INDEX</span><b>{{ String(items.length).padStart(2, '0') }}</b>
    </div>
    <nav aria-label="主导航">
      <RouterLink
        v-for="item in tiles"
        :key="item.path"
        :to="item.path"
        @click="emit('open')"
        class="hud-module"
        data-hud-tile
        :data-tile-group="item.group"
        :class="{
          active:
            expanded &&
            (route.path === item.path ||
              (item.path === '/' && route.path.startsWith('/workbench/')) ||
              (item.path === '/accounts' && route.path.startsWith('/accounts/'))),
        }"
        :style="{
          '--hud-x': `${item.x / 10}%`,
          '--hud-y': `${item.y / 6.4}%`,
          '--hud-w': `${item.w / 10}%`,
          '--hud-h': `${item.h / 6.4}%`,
        }"
      >
        <span class="hud-module-plate" aria-hidden="true">
          <svg class="hud-module-surface" :viewBox="`0 0 ${item.w} ${item.h}`" overflow="visible">
            <defs>
              <linearGradient :id="`hud-tile-${item.index}`" x2=".35" y2="1">
                <stop class="hud-face-high" />
                <stop offset=".6" class="hud-face-mid" />
                <stop offset="1" class="hud-face-low" />
              </linearGradient>
            </defs>
            <g :transform="item.index === 5 ? `translate(${item.w} 0) scale(-1 1)` : ''">
              <path
                :d="item.outline"
                :transform="`translate(${item.group === 'right' ? -4 : 4} 5)`"
                fill="#555953"
                stroke="#8e9389"
                stroke-width=".6"
              />
              <path
                :d="item.outline"
                :fill="`url(#hud-tile-${item.index})`"
                stroke="#ededdf"
                stroke-width=".7"
              />
              <path
                :d="item.outline"
                :transform="`translate(3 3) scale(${(item.w - 6) / item.w} ${(item.h - 6) / item.h})`"
                fill="none"
                stroke="#62675c"
                stroke-width=".6"
                opacity=".6"
              />
            </g>
          </svg>
        </span>
        <small>{{ String(item.index + 1).padStart(2, '0') }}</small>
        <Icon :name="item.icon" :size="22" />
        <span>{{ item.label }}</span>
      </RouterLink>
    </nav>
    <RouterLink v-if="expanded" to="/" class="hud-return" @click="emit('overview')"
      ><Icon name="grid" :size="15" />返回环形概览<Icon name="arrow" :size="14"
    /></RouterLink>
    <p class="hud-nav-caption">ATELIER / PERSONAL WORKSTATION</p>
  </aside>
</template>
