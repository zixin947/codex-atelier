<script setup lang="ts">
// Vue adaptation of magpie routing/client.js stage, candidates, counters and try log.
// Copyright (c) 2026 yetone, MIT. See vendor/ATELIER-LICENSE.txt.
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import Icon from '../../shared/components/Icon.vue'
import { readRouting } from '../../services/routing'
import type { RouteTrace, RoutingSnapshot } from '../../types/routing'
import license from './vendor/ATELIER-LICENSE.txt?raw'

const props = defineProps<{ service?: () => Promise<RoutingSnapshot | null> }>()
const data = ref<RoutingSnapshot | null>(null)
const loaded = ref(false),
  error = ref(''),
  masked = ref(true),
  selected = ref<string | null>(null)
const filter = ref('all')
const stage = ref<HTMLElement | null>(null)
const wires = ref<{ d: string; lit: boolean; live: boolean }[]>([])
const current = computed(
  () => data.value?.routes.find((r) => r.id === selected.value) ?? data.value?.routes[0],
)
const routes = computed(
  () =>
    data.value?.routes.filter(
      (r) =>
        filter.value === 'all' ||
        (filter.value === 'rerouted' ? r.attempts.length > 1 : r.status === 'failed'),
    ) ?? [],
)
const accounts = computed(() => [
  ...new Set(data.value?.routes.flatMap((r) => r.candidates.map((c) => c.profileId)) ?? []),
])
const running = computed(() => data.value?.runtime.state === 'running')
const total = computed(() =>
  data.value
    ? data.value.runtime.completedRequests +
      data.value.runtime.failedRequests +
      data.value.runtime.activeRequests
    : 0,
)
function name(id: string) {
  if (masked.value) return `账号 ${accounts.value.indexOf(id) + 1}`
  return current.value?.candidates.find((c) => c.profileId === id)?.name ?? '账号已移除'
}
function state(r: RouteTrace) {
  return {
    routing: '正在选择账号',
    forwarding: '正在响应',
    completed: '响应完成',
    failed: '请求失败',
  }[r.status]
}
function time(at: number) {
  return new Date(at).toLocaleTimeString('zh-CN', { hour12: false })
}
function outcome(id: string) {
  const a = current.value?.attempts.find((a) => a.profileId === id)
  if (!a) return ''
  if (a.durationMs === null) return '正在尝试…'
  return `${a.httpStatus ? `HTTP ${a.httpStatus} · ` : ''}${a.code} · ${a.durationMs} ms`
}
function layout() {
  const root = stage.value
  if (!root) return
  const rect = root.getBoundingClientRect()
  const hub = root.querySelector('.rt-hub')!.getBoundingClientRect()
  const src = root.querySelector('.rt-src')!.getBoundingClientRect()
  const path = (x: number, y: number, endX: number, endY: number) =>
    `M ${x - rect.left} ${y - rect.top} C ${(x + endX) / 2 - rect.left} ${y - rect.top}, ${(x + endX) / 2 - rect.left} ${endY - rect.top}, ${endX - rect.left} ${endY - rect.top}`
  const live = !!current.value && current.value.durationMs === null
  wires.value = [
    {
      d: path(src.right, src.top + src.height / 2, hub.left, hub.top + hub.height / 2),
      lit: !!current.value,
      live,
    },
    ...Array.from(root.querySelectorAll<HTMLElement>('.rt-accts li')).map((el, index) => {
      const row = el.getBoundingClientRect()
      const id = current.value?.candidates[index]?.profileId
      const a = current.value?.attempts.find((a) => a.profileId === id)
      return {
        d: path(hub.right, hub.top + hub.height / 2, row.left, row.top + row.height / 2),
        lit: !!a,
        live: !!a && a.durationMs === null,
      }
    }),
  ]
}
let alive = true,
  pending = false,
  timer: ReturnType<typeof setTimeout> | undefined
let observer: ResizeObserver
async function refresh() {
  clearTimeout(timer)
  if (pending || (document.hidden && loaded.value) || !alive) return
  pending = true
  try {
    const result = await (props.service ?? readRouting)()
    if (alive) {
      data.value = result
      error.value = ''
    }
  } catch (e) {
    if (alive) error.value = `路由状态读取失败：${String(e)}。当前内容可能已过期。`
  } finally {
    pending = false
    if (alive) {
      loaded.value = true
      if (!document.hidden) timer = setTimeout(refresh, 1000)
    }
  }
}
function visibility() {
  if (document.hidden) clearTimeout(timer)
  else void refresh()
}
watch(current, async () => {
  await nextTick()
  layout()
})
onMounted(() => {
  observer = new ResizeObserver(layout)
  observer.observe(stage.value!)
  document.addEventListener('visibilitychange', visibility)
  void refresh()
})
onUnmounted(() => {
  alive = false
  clearTimeout(timer)
  observer.disconnect()
  document.removeEventListener('visibilitychange', visibility)
})
</script>

<template>
  <div class="page routing-page">
    <header class="routing-header">
      <div>
        <p class="eyebrow">ATELIER / LIVE ROUTING</p>
        <h1>路由<span class="heading-dot">.</span></h1>
      </div>
      <div class="routing-actions">
        <RouterLink class="button" to="/connections"
          >路由设置 <Icon name="chevron" :size="14"
        /></RouterLink>
        <button class="button" :aria-pressed="masked" @click="masked = !masked">
          <Icon name="user" :size="14" />{{ masked ? '账号已打码' : '隐藏账号' }}
        </button>
      </div>
    </header>
    <div v-if="error" class="routing-error" role="alert">
      {{ error }} <button class="button" @click="refresh">重试</button>
    </div>
    <div class="routing-status">
      <i :class="{ on: running && !error }" />{{
        !loaded
          ? '正在读取路由状态…'
          : !data
            ? '请在 Atelier 桌面应用中查看真实路由，浏览器预览不连接本机数据。'
            : error
              ? '连接异常 · 已暂停实时状态展示'
              : running
                ? '实时 · 网关正在做的决定'
                : data.runtime.state === 'error'
                  ? '路由引擎异常退出，请检查配置后重新启动'
                  : '网关已停止 · 启动后记录经过 Atelier 的请求'
      }}
    </div>
    <section class="rt" aria-label="实时路由流程">
      <div class="rt-top">
        <div class="rt-what">
          <b>{{ current ? `${current.model || '未指定模型'} · ${state(current)}` : '等待请求' }}</b>
        </div>
        <p class="rt-mode">
          {{
            current
              ? `${time(current.at)} · /v1/${current.endpoint} · ${current.durationMs === null ? '进行中' : `${current.durationMs} ms`}`
              : '经 Atelier 的 Agent 发出请求后，这里实时展示：路由把谁排第一及原因、每次尝试和各自的响应。'
          }}
        </p>
      </div>
      <div ref="stage" class="rt-stage">
        <svg class="rt-wires" aria-hidden="true">
          <path
            v-for="(wire, i) in wires"
            :key="i"
            :d="wire.d"
            :class="{ lit: wire.lit, live: wire.live && running && !error }"
          />
        </svg>
        <div class="rt-srcs">
          <div class="rt-node rt-src"><Icon name="box" :size="26" /><b>你的 Agent</b></div>
        </div>
        <div class="rt-node rt-hub">
          <Icon name="chip" :size="28" /><b>Atelier</b
          ><small>{{ data?.runtime.address?.replace('/v1', '') ?? '本地网关' }}</small
          ><i v-if="data?.runtime.activeRequests">{{ data.runtime.activeRequests }} 个在途请求</i>
        </div>
        <ol class="rt-accts">
          <li v-if="!current?.candidates.length" class="idle">
            {{ current ? '没有可用候选计划' : '还没有请求' }}
          </li>
          <li
            v-for="(candidate, i) in current?.candidates"
            :key="candidate.profileId"
            :class="{
              on: current?.attempts.at(-1)?.profileId === candidate.profileId,
              rest: !candidate.eligible,
              now:
                current?.attempts.at(-1)?.profileId === candidate.profileId &&
                current.durationMs === null &&
                running &&
                !error,
            }"
          >
            <i class="dot" /><b
              ><span class="who">{{ name(candidate.profileId) }}</span></b
            ><em>{{ outcome(candidate.profileId) || candidate.reason }}</em>
            <small class="routing-reason">{{ i + 1 }} · {{ candidate.reason }}</small>
          </li>
        </ol>
      </div>
      <div class="rt-foot">
        <p class="rt-cap" aria-live="polite">
          {{
            current
              ? current.code || '网关正在执行上方候选计划'
              : 'Agent 发给 Atelier 的每个请求都会在此显示，均为真实路由。'
          }}
        </p>
        <div class="rt-stats">
          <span
            ><b>{{ total }}</b
            ><span>请求</span></span
          ><span
            ><b>{{ data?.reroutedRequests ?? 0 }}</b
            ><span>改道</span></span
          ><span
            ><b>{{ data?.runtime.failedRequests ?? 0 }}</b
            ><span>Agent 收到的错误</span></span
          >
        </div>
      </div>
      <div v-if="current" class="rt-log">
        <div class="rt-log-head">
          <b>逐次尝试</b><span>{{ current.attempts.length }} 次 · {{ state(current) }}</span>
        </div>
        <ol class="rt-steps">
          <li
            v-for="(attempt, i) in current.attempts"
            :key="i"
            :class="{
              wait: attempt.durationMs === null && !error,
              ok: attempt.code === 'response.completed',
              bad: attempt.durationMs !== null && attempt.code !== 'response.completed',
            }"
          >
            <b>{{ i + 1 }}. {{ name(attempt.profileId) }}</b> · {{ outcome(attempt.profileId)
            }}<span v-if="i + 1 < current.attempts.length"> → 改道到下一候选</span>
          </li>
          <li v-if="!current.attempts.length">{{ current.code || '正在选择账号' }}</li>
        </ol>
      </div>
    </section>
    <section v-if="data?.routes.length" class="routing-history" aria-label="请求记录">
      <div class="routing-actions">
        <h2>请求记录</h2>
        <select v-model="filter" aria-label="筛选请求">
          <option value="all">全部请求</option>
          <option value="rerouted">发生改道</option>
          <option value="failed">请求失败</option></select
        ><button v-if="selected" class="button" @click="selected = null">跟随最新请求</button
        ><span class="muted">本次运行 · 最近 100 条及在途请求</span>
      </div>
      <div class="routing-requests">
        <button
          v-for="r in routes"
          :key="r.id"
          class="routing-request"
          :class="r.status"
          :aria-pressed="current?.id === r.id"
          @click="selected = r.id"
        >
          <time>{{ time(r.at) }}</time
          ><code>{{ r.model || '未指定模型' }}</code
          ><span>{{ state(r) }}{{ r.attempts.length > 1 ? ' · 已改道' : '' }}</span
          ><span
            >{{ r.attempts.length }} 次尝试 ·
            {{ r.durationMs === null ? '进行中' : `${r.durationMs} ms` }}</span
          >
        </button>
        <p v-if="!routes.length" class="muted">没有符合筛选条件的请求。</p>
      </div>
    </section>
    <details class="routing-license">
      <summary>第三方来源与许可</summary>
      <p>
        Atelier
        负责候选决策、请求转发和事件记录。第三方来源：yetone/magpie（MIT）。路由设置在「模型与连接」中管理。
      </p>
      <pre>{{ license }}</pre>
    </details>
  </div>
</template>

<style scoped src="./vendor/routing-stage.css"></style>
<style scoped src="./routing.css"></style>
