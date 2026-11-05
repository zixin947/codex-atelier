<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import Icon from '../../shared/components/Icon.vue'
import Modal from '../../shared/components/Modal.vue'
import { threadsService as service } from '../../services/threads'
import type { ThreadCatalog, ThreadDetail, ThreadPlan, ThreadBackup } from '../../types/threads'
import { compact, filterThreads, projectName, usageSegments, exportMarkdown } from './model'
import './threads.css'
import UsageHistory from './UsageHistory.vue'
const catalog = ref<ThreadCatalog | null>(null)
const busy = ref(false),
  error = ref(''),
  notice = ref(''),
  selected = ref(''),
  detail = ref<ThreadDetail | null>(null),
  detailBusy = ref(false),
  detailError = ref('')
const search = ref(''),
  project = ref(''),
  archive = ref('all'),
  model = ref(''),
  start = ref(''),
  end = ref(''),
  sort = ref('updated'),
  range = ref('all'),
  tab = ref('usage'),
  grouped = ref(true),
  showDetail = ref(true)
const page = ref(1),
  checked = ref<string[]>([]),
  plan = ref<ThreadPlan | null>(null),
  confirmed = ref(false),
  inspecting = ref(false),
  backupOpen = ref(false),
  backups = ref<ThreadBackup[]>([])
let detailSequence = 0
let scanGeneration = 0
const scanning = ref(false),
  scanned = ref(0)
const usageCache = ref<Record<string, ThreadDetail>>({})
const scanFailures = ref(0)
const aggregate = computed(() => {
  const details = filtered.value
    .map((r) => usageCache.value[r.id])
    .filter((d): d is ThreadDetail => !!d)
  return {
    count: details.length,
    known: details.filter((d) => d.usage !== null).length,
    bytes: details.reduce((s, d) => s + d.bytes, 0),
    input: details.reduce((s, d) => s + (d.usage?.input ?? 0), 0),
    cached: details.reduce((s, d) => s + (d.usage?.cached ?? 0), 0),
    output: details.reduce((s, d) => s + (d.usage?.output ?? 0), 0),
  }
})
function cacheRate(id: string) {
  const u = usageCache.value[id]?.usage
  return u?.input ? `${((u.cached / u.input) * 100).toFixed(1)}%` : '—'
}
async function scanUsage() {
  if (scanning.value) {
    scanGeneration++
    scanning.value = false
    return
  }
  const gen = ++scanGeneration
  scanning.value = true
  scanned.value = 0
  scanFailures.value = 0
  const ids = visible.value.map((r) => r.id)
  for (const id of ids) {
    if (gen !== scanGeneration) break
    try {
      const d = await service.detail(id)
      if (gen === scanGeneration) usageCache.value[id] = d
    } catch {
      scanFailures.value++
    } finally {
      if (gen === scanGeneration) scanned.value++
    }
  }
  if (gen === scanGeneration) scanning.value = false
}
onBeforeUnmount(() => {
  scanGeneration++
  detailSequence++
})
const rows = computed(() =>
  (catalog.value?.threads ?? []).map((r) => ({
    ...r,
    model: r.model || usageCache.value[r.id]?.models.at(-1) || '',
  })),
)
const filtered = computed(() =>
  filterThreads(rows.value, {
    search: search.value,
    project: project.value,
    archive: archive.value,
    model: model.value,
    start: start.value,
    end: end.value,
    sort: sort.value,
  }),
)
const projects = computed(() =>
  [...new Set(rows.value.map((r) => r.cwd))].map((path) => ({
    path,
    name: projectName(path),
    count: rows.value.filter((r) => r.cwd === path).length,
  })),
)
const models = computed(() => [...new Set(rows.value.map((r) => r.model).filter(Boolean))])
const visible = computed(() => filtered.value.slice((page.value - 1) * 40, page.value * 40))
const groups = computed(() =>
  grouped.value
    ? [...new Set(visible.value.map((r) => r.cwd))].map((path) => ({
        path,
        name: projectName(path),
        rows: visible.value.filter((r) => r.cwd === path),
      }))
    : [{ path: 'all', name: '全部线程', rows: visible.value }],
)
const current = computed(() => rows.value.find((r) => r.id === selected.value))
const totals = computed(() => ({
  tokens: filtered.value.reduce((sum, r) => sum + (r.tokens ?? 0), 0),
  known: filtered.value.filter((r) => r.tokens !== null).length,
  archived: filtered.value.filter((r) => r.archived).length,
  mismatch: filtered.value.filter((r) => r.provider !== catalog.value?.provider).length,
}))
const mismatch = computed(() => rows.value.filter((r) => r.provider !== catalog.value?.provider))
const segments = computed(() => (detail.value?.usage ? usageSegments(detail.value.usage) : []))
const segmentLabels = ['输入（非缓存）', '缓存输入', '输出（非推理）', '推理输出']
const segmentTotal = computed(() => segments.value.reduce((a, b) => a + b, 0))
const targets = computed(() =>
  checked.value.length ? checked.value : selected.value ? [selected.value] : [],
)
const date = (n: number) =>
  n
    ? new Date(n).toLocaleString('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
      })
    : '未知'
const bytes = (n: number) => `${(n / 1024 / 1024).toFixed(1)} MB`
watch([search, project, archive, model, start, end, sort], () => {
  page.value = 1
  checked.value = []
})
watch(range, () => {
  end.value = ''
  if (range.value === 'all') {
    start.value = ''
    return
  }
  if (range.value !== 'custom') {
    const d = new Date()
    if (range.value === 'week') d.setDate(d.getDate() - 6)
    start.value = `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`
  }
})
async function load() {
  busy.value = true
  error.value = ''
  try {
    scanGeneration++
    scanning.value = false
    usageCache.value = {}
    catalog.value = await service.catalog()
    checked.value = checked.value.filter((id) => rows.value.some((r) => r.id === id))
    if (!rows.value.some((r) => r.id === selected.value)) selected.value = rows.value[0]?.id ?? ''
    if (selected.value) await readDetail(selected.value)
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function readDetail(id: string) {
  selected.value = id
  detail.value = null
  detailError.value = ''
  detailBusy.value = true
  const seq = ++detailSequence
  try {
    const d = await service.detail(id)
    if (seq === detailSequence) {
      detail.value = d
      usageCache.value[id] = d
    }
  } catch (e) {
    if (seq === detailSequence) detailError.value = String(e)
  } finally {
    if (seq === detailSequence) detailBusy.value = false
  }
}
function choose(id: string) {
  showDetail.value = true
  void readDetail(id)
}
async function preview(
  action: 'provider' | 'migration' | 'archive',
  value: string,
  ids = targets.value,
) {
  error.value = ''
  busy.value = true
  try {
    plan.value = await service.preview(ids, action, value)
    confirmed.value = false
    inspecting.value = false
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function apply() {
  if (!plan.value || !confirmed.value) return
  busy.value = true
  error.value = ''
  try {
    await service.apply(plan.value.ticket)
    plan.value = null
    notice.value = '索引操作已完成，备份已保存。请在 Desktop 核对实际状态。'
    await load()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function openBackups() {
  busy.value = true
  try {
    backups.value = await service.backups()
    backupOpen.value = true
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function restore(id: string) {
  try {
    plan.value = await service.restore(id)
    confirmed.value = false
    backupOpen.value = false
  } catch (e) {
    error.value = String(e)
  }
}
async function download(format: 'md' | 'json') {
  if (!detail.value || !current.value) return
  if (isTauri()) {
    try {
      const path = await invoke<string>('threads_export', { id: current.value.id, format })
      notice.value = `已导出到 ${path}`
    } catch (e) {
      error.value = String(e)
    }
    return
  }
  const content =
    format === 'md'
      ? exportMarkdown(current.value.title, detail.value.messages)
      : JSON.stringify({ thread: current.value, detail: detail.value }, null, 2)
  const url = URL.createObjectURL(
    new Blob([content], {
      type: format === 'md' ? 'text/markdown;charset=utf-8' : 'application/json',
    }),
  )
  const a = document.createElement('a')
  a.href = url
  a.download = `thread-${current.value.id.replace(/[^\w-]/g, '_')}.${format}`
  a.click()
  URL.revokeObjectURL(url)
  notice.value = detail.value.complete
    ? '已导出所选会话。'
    : '已导出解析到的部分记录；完整性提示见详情。'
}
async function copy() {
  try {
    await navigator.clipboard.writeText(selected.value)
    notice.value = '线程 ID 已复制'
  } catch {
    error.value = '无法访问剪贴板'
  }
}
function togglePage(event: Event) {
  checked.value = (event.target as HTMLInputElement).checked ? visible.value.map((r) => r.id) : []
}
onMounted(() => {
  void load().then(() => {
    if (catalog.value) return scanUsage()
  })
})
</script>
<template>
  <section class="threads-page">
    <header class="thread-heading">
      <div>
        <p class="thread-eyebrow">ARCHIVE / WORKSPACE RECORDS</p>
        <h1>线程管理<span class="thread-edition">06</span></h1>
        <p>保留上下文，让每一次探索都有迹可循。</p>
      </div>
      <div class="thread-heading-actions">
        <span class="thread-source">{{
          catalog?.source === 'demo' ? '演示数据 · 不连接本机' : catalog ? '本机索引' : '尚未读取'
        }}</span
        ><button class="button" :disabled="busy" @click="load">
          <Icon name="refresh" :class="{ spinning: busy }" />{{ busy ? '更新中…' : '刷新' }}
        </button>
      </div>
    </header>
    <p v-if="error" class="thread-alert" role="alert">{{ error }}</p>
    <p v-if="notice" class="thread-notice" role="status">
      {{ notice }}<button aria-label="关闭通知" @click="notice = ''">×</button>
    </p>
    <p v-for="w in catalog?.warnings" :key="w" class="thread-alert">{{ w }}</p>
    <div v-if="!catalog" class="thread-welcome" role="status">
      <Icon :name="busy ? 'refresh' : 'history'" :class="{ spinning: busy }" :size="32" />
      <h2>{{ busy ? '正在载入线程…' : '暂时无法载入线程' }}</h2>
      <button v-if="!busy" class="button" @click="load">重试</button>
    </div>
    <template v-else>
      <div class="thread-summary">
        <article class="thread-stat thread-stat-main">
          <span>THREADS / 当前筛选</span
          ><strong>{{ filtered.length.toString().padStart(2, '0') }}<small>条线程</small></strong>
          <div>{{ projects.length }} 个项目 <i /> {{ totals.archived }} 条已归档</div>
        </article>
        <article class="thread-stat">
          <span>已记录 TOKEN</span><strong>{{ compact(totals.tokens) }}</strong>
          <div>{{ totals.known }} / {{ filtered.length }} 条索引有记录</div>
        </article>
        <article class="thread-stat">
          <span>缓存命中</span
          ><strong>{{
            aggregate.input ? `${((aggregate.cached / aggregate.input) * 100).toFixed(1)}%` : '—'
          }}</strong>
          <div>已解析记录 · 缓存 / 输入</div>
        </article>
        <article class="thread-stat">
          <span>记录文件</span><strong>{{ aggregate.count ? bytes(aggregate.bytes) : '—' }}</strong>
          <div>{{ aggregate.count }} / {{ filtered.length }} 条已解析</div>
        </article>
        <article class="thread-stat">
          <span>输入 TOKEN</span
          ><strong>{{ aggregate.known ? compact(aggregate.input) : '—' }}</strong>
          <div>含缓存 · 已解析部分</div>
        </article>
        <article class="thread-stat">
          <span>缓存输入</span
          ><strong>{{ aggregate.known ? compact(aggregate.cached) : '—' }}</strong>
          <div>输入的一部分</div>
        </article>
        <article class="thread-stat">
          <span>输出 TOKEN</span
          ><strong>{{ aggregate.known ? compact(aggregate.output) : '—' }}</strong>
          <div>含推理 · 已解析部分</div>
        </article>
      </div>
      <div class="thread-toolbar">
        <div class="thread-segments" aria-label="归档筛选">
          <button
            v-for="item in [
              { v: 'all', t: '全部' },
              { v: 'active', t: '未归档' },
              { v: 'archived', t: '已归档' },
            ]"
            :key="item.v"
            :aria-pressed="archive === item.v"
            @click="archive = item.v"
          >
            {{ item.t }}
          </button>
        </div>
        <label class="thread-search"
          ><Icon name="search" /><input
            v-model="search"
            aria-label="搜索线程"
            placeholder="搜索线程、项目、路径或模型"
        /></label>
        <select v-model="model" aria-label="模型筛选">
          <option value="">全部模型</option>
          <option v-for="m in models" :key="m">{{ m }}</option>
        </select>
        <select v-model="sort" aria-label="排序">
          <option value="updated">最近更新</option>
          <option value="tokens">Token 用量</option>
          <option value="title">标题</option>
        </select>
        <button
          v-if="catalog.provider && mismatch.length"
          class="button"
          :disabled="busy"
          @click="inspecting = true"
        >
          <Icon name="alert" />Provider 差异 {{ mismatch.length }}
        </button>
      </div>
      <div class="thread-subtoolbar">
        <span>{{ checked.length ? `已选 ${checked.length} 条` : `${filtered.length} 条结果` }}</span
        ><button :disabled="!targets.length || busy" @click="preview('archive', 'true')">
          归档</button
        ><button :disabled="!targets.length || busy" @click="preview('archive', 'false')">
          取消归档</button
        ><button :disabled="busy" @click="openBackups">操作备份</button
        ><button @click="scanUsage">
          {{ scanning ? `停止解析 ${scanned}/${visible.length}` : '解析本页用量' }}</button
        ><span v-if="scanFailures">{{ scanFailures }} 条解析失败</span>
        <span class="thread-toolbar-spacer" /><select v-model="range" aria-label="时间范围">
          <option value="all">全部时间</option>
          <option value="today">今天</option>
          <option value="week">近 7 天</option>
          <option value="custom">自定义日期</option></select
        ><template v-if="range === 'custom'"
          ><input v-model="start" type="date" aria-label="开始日期" /><input
            v-model="end"
            type="date"
            aria-label="结束日期" /></template
        ><button
          :aria-pressed="grouped"
          title="项目分组"
          aria-label="项目分组"
          @click="grouped = !grouped"
        >
          <Icon name="box" /></button
        ><button
          :aria-pressed="showDetail"
          title="详情面板"
          aria-label="详情面板"
          @click="showDetail = !showDetail"
        >
          <Icon name="book" />
        </button>
      </div>
      <div class="thread-workspace" :class="{ 'without-detail': !showDetail }">
        <aside class="thread-projects" aria-label="项目目录">
          <div class="thread-section-label">
            项目 <span>{{ projects.length }}</span>
          </div>
          <button :aria-pressed="project === ' ' || project === ''" @click="project = ''">
            全部线程 <span>{{ rows.length }}</span></button
          ><button
            v-for="p in projects"
            :key="p.path"
            :aria-pressed="project === p.path"
            :title="p.path"
            @click="project = p.path"
          >
            <Icon name="box" :size="14" /><span>{{ p.name }}</span
            ><small>{{ p.count }}</small>
          </button>
          <div class="thread-project-footer">
            <span>LOCAL ARCHIVE</span>
            <p>上下文留在本机<br />按需读取，不上传</p>
            <i />
          </div>
        </aside>
        <div class="thread-list">
          <div class="thread-list-head">
            <input
              type="checkbox"
              aria-label="选择当前页全部线程"
              :checked="visible.length > 0 && visible.every((r) => checked.includes(r.id))"
              @change="togglePage"
            /><span>线程 / THREAD</span><span>Token</span><span>更新</span>
          </div>
          <div v-if="!filtered.length" class="thread-empty">
            <Icon name="search" :size="28" />
            <h3>没有匹配的线程</h3>
            <p>试试其他关键词或调整筛选范围。</p>
          </div>
          <section v-for="g in groups" :key="g.path" class="thread-group">
            <h3 v-if="grouped">
              <Icon name="chevron" :size="12" />{{ g.name }}<span>{{ g.rows.length }} 条</span>
            </h3>
            <div
              v-for="r in g.rows"
              :key="r.id"
              class="thread-row"
              :class="{ selected: r.id === selected }"
            >
              <input
                v-model="checked"
                type="checkbox"
                :value="r.id"
                :aria-label="`选择 ${r.title}`"
              /><button class="thread-row-main" @click="choose(r.id)">
                <span class="thread-row-title"
                  ><span class="thread-title-text">{{ r.title || '未命名线程' }}</span
                  ><Icon v-if="r.provider !== catalog.provider" name="alert" :size="13" /></span
                ><span class="thread-row-meta"
                  >{{ r.archived ? '已归档' : '未归档' }}<i />{{ r.model || '模型未记录' }}<i />{{
                    r.provider || 'Provider 未记录'
                  }}</span
                ></button
              ><span class="thread-row-number"
                >{{ compact(r.tokens) }}<small>{{ cacheRate(r.id) }}</small></span
              ><time>{{ date(r.updatedAt) }}</time>
            </div>
          </section>
          <footer class="thread-pagination">
            <span>{{ page }} / {{ Math.max(1, Math.ceil(filtered.length / 40)) }}</span
            ><button :disabled="page <= 1" @click="page--">上一页</button
            ><button :disabled="page * 40 >= filtered.length" @click="page++">下一页</button>
          </footer>
        </div>
        <aside v-if="showDetail" class="thread-detail" aria-label="线程详情">
          <template v-if="current"
            ><div class="thread-detail-top">
              <span>{{ projectName(current.cwd) }}</span
              ><button aria-label="关闭详情" @click="showDetail = false">
                <Icon name="close" :size="16" />
              </button>
            </div>
            <p class="thread-eyebrow">SELECTED RECORD</p>
            <h2>{{ current.title || '未命名线程' }}</h2>
            <div class="thread-detail-tags">
              <span>{{ current.archived ? '已归档' : '未归档' }}</span
              ><span>{{ detail?.turns ?? '—' }} 轮</span><span>{{ current.provider }}</span>
            </div>
            <div
              class="thread-tabs"
              role="tablist"
              aria-label="线程详情分类"
              @keydown="
                (event: KeyboardEvent) => {
                  if (event.key === 'ArrowRight' || event.key === 'ArrowLeft') {
                    event.preventDefault()
                    const tabs = ['usage', 'tools', 'metadata', 'messages']
                    tab = tabs[(tabs.indexOf(tab) + (event.key === 'ArrowRight' ? 1 : 3)) % 4]!
                    const buttons = (event.currentTarget as HTMLElement).querySelectorAll('button')
                    buttons[tabs.indexOf(tab)]?.focus()
                  }
                }
              "
            >
              <button
                v-for="t in [
                  { v: 'usage', t: '用量' },
                  { v: 'tools', t: '工具' },
                  { v: 'metadata', t: '元数据' },
                  { v: 'messages', t: '正文' },
                ]"
                :key="t.v"
                role="tab"
                :aria-selected="tab === t.v"
                :tabindex="tab === t.v ? 0 : -1"
                @click="tab = t.v"
              >
                {{ t.t }}
              </button>
            </div>
            <div v-if="detailBusy" class="thread-empty" role="status">
              <Icon name="refresh" class="spinning" />正在解析所选记录…
            </div>
            <p v-else-if="detailError" class="thread-alert" role="alert">{{ detailError }}</p>
            <template v-else-if="detail"
              ><p v-for="w in detail.warnings" :key="w" class="thread-detail-warning">{{ w }}</p>
              <p v-if="detail.encrypted" class="thread-detail-warning">
                包含加密历史。切换 Provider 后不保证能继续使用。
              </p>
              <div v-if="tab === 'usage'" class="thread-usage">
                <span class="thread-section-label"
                  >已记录 Token
                  <small>{{ detail.complete ? '已解析' : '部分 / 未知' }}</small></span
                ><strong>{{ compact(detail.usage?.total ?? null) }}<small>Tokens</small></strong>
                <div v-if="detail.usage" class="thread-token-bar" aria-label="Token 构成">
                  <i
                    v-for="(n, i) in segments"
                    :key="i"
                    :style="{ width: `${segmentTotal ? (n / segmentTotal) * 100 : 0}%` }"
                  />
                </div>
                <dl v-if="detail.usage">
                  <div v-for="(n, i) in segments" :key="i">
                    <dt><i :class="`segment-${i}`" />{{ segmentLabels[i] }}</dt>
                    <dd>{{ compact(n) }}</dd>
                  </div>
                </dl>
                <div class="thread-usage-foot">
                  <span
                    >缓存命中<strong>{{
                      detail.usage?.input
                        ? `${((detail.usage.cached / detail.usage.input) * 100).toFixed(1)}%`
                        : '—'
                    }}</strong></span
                  ><span
                    >记录文件<strong>{{ bytes(detail.bytes) }}</strong></span
                  >
                </div>
                <p class="thread-detail-note">
                  累计快照取最后一次记录，不将重复快照相加。与索引统计可能存在更新时间差。
                </p>
                <UsageHistory :detail="detail" />
              </div>
              <div v-else-if="tab === 'tools'" class="thread-tools">
                <div v-for="tool in detail.tools" :key="tool.name">
                  <code>{{ tool.name }}</code
                  ><b>{{ tool.count }}</b>
                </div>
                <p v-if="!detail.tools.length">没有解析到工具调用。</p>
              </div>
              <dl v-else-if="tab === 'metadata'" class="thread-metadata">
                <template
                  v-for="(v, k) in {
                    ID: current.id,
                    项目: current.cwd,
                    Provider: current.provider,
                    模型: detail.models.join(' / ') || current.model || '未知',
                    来源: current.source,
                    创建: date(current.createdAt),
                    更新: date(current.updatedAt),
                    文件: detail.resolvedPath || current.rolloutPath,
                    格式: detail.compressed ? 'Zstandard 压缩记录' : 'JSONL',
                  }"
                  :key="k"
                  ><dt>{{ k }}</dt>
                  <dd>{{ v }}</dd></template
                >
              </dl>
              <div v-else class="thread-messages">
                <article v-for="(m, i) in detail.messages" :key="i">
                  <b>{{ m.role }}</b>
                  <p>{{ m.text }}</p>
                </article>
                <p v-if="!detail.messages.length">没有可展示的消息正文。</p>
              </div>
            </template>
            <footer class="thread-detail-actions">
              <button class="button" :disabled="!detail" @click="tab = 'messages'">
                <Icon name="book" :size="14" />查看对话
              </button>
              <button class="button" @click="copy"><Icon name="copy" :size="14" />ID</button
              ><button class="button" :disabled="!detail || detailBusy" @click="download('md')">
                <Icon name="download" :size="14" />Markdown</button
              ><button class="button" :disabled="!detail || detailBusy" @click="download('json')">
                JSON
              </button>
            </footer></template
          >
          <div v-else class="thread-empty">选择一条线程查看详情。</div>
        </aside>
      </div>
      <footer class="thread-statusbar">
        <span><i />{{ catalog.source === 'demo' ? 'DEMO' : 'LOCAL' }} · {{ catalog.database }}</span
        ><span>索引读取 {{ date(catalog.checkedAt) }}</span>
      </footer>
    </template>
    <Modal v-if="inspecting" title="Provider 差异" @close="inspecting = false"
      ><p>
        当前配置 Provider：<strong>{{ catalog?.provider || '未知' }}</strong>
      </p>
      <p>
        发现 {{ mismatch.length }} 条不同 Provider 的索引记录。不同不一定是错误；只有确认需要在当前
        Provider 下显示时才对齐。
      </p>
      <div class="thread-diff">
        <div v-for="r in mismatch.slice(0, 100)" :key="r.id">
          <b>{{ r.title }}</b
          ><code>{{ r.provider || '空' }} → {{ catalog?.provider }}</code>
        </div>
      </div>
      <p class="thread-detail-warning">
        这是索引级修复，不改会话文件、不转换加密历史，不保证跨账号或跨模型续聊成功。
      </p>
      <button
        class="button primary"
        :disabled="!mismatch.length || !catalog?.provider || busy"
        @click="
          preview(
            'provider',
            catalog!.provider,
            mismatch.slice(0, 100).map((r) => r.id),
          )
        "
      >
        预览{{ mismatch.length > 100 ? '前 100 条' : '' }}修改<Icon name="arrow" /></button
      ><button
        class="button"
        :disabled="!mismatch.length || !catalog?.provider || busy"
        @click="
          preview(
            'migration',
            catalog!.provider,
            mismatch.slice(0, 100).map((r) => r.id),
          )
        "
      >
        预览完整迁移
      </button>
      <p>完整迁移同时修改会话文件元数据，执行前需退出客户端；非元数据记录保持原字节。</p>
    </Modal>
    <Modal v-if="plan" title="确认索引变更" @close="!busy && (plan = null)"
      ><p v-for="w in plan.warnings" :key="w" class="thread-detail-warning">{{ w }}</p>
      <div class="thread-diff">
        <div v-for="c in plan.changes" :key="c.id">
          <b>{{ c.title }}</b
          ><code>{{ c.before }} → {{ c.after }}</code>
        </div>
      </div>
      <p v-if="plan.rollouts?.length">
        另有 {{ plan.rollouts.length }} 个会话文件元数据将迁移，原文件已保存在预览备份中。
      </p>
      <p v-if="!plan.changes.length && !plan.rollouts?.length">没有需要修改的记录。</p>
      <label class="thread-confirm"
        ><input
          v-model="confirmed"
          type="checkbox"
        />我已结束所选对话的生成，并确认上述修改范围。</label
      >
      <p v-if="error" role="alert" class="thread-alert">{{ error }}</p>
      <button
        class="button primary"
        :disabled="!confirmed || (!plan.changes.length && !plan.rollouts?.length) || busy"
        @click="apply"
      >
        {{ busy ? '执行中…' : '保存备份并执行' }}
      </button></Modal
    >
    <Modal v-if="backupOpen" title="索引操作备份" @close="backupOpen = false"
      ><p>备份只保存本次修改字段；回滚前重新检查外部变化。</p>
      <div class="thread-backups">
        <div v-for="b in backups" :key="b.id">
          <span>{{ date(b.createdAt) }} · {{ b.action }} · {{ b.count }} 条</span
          ><button class="button" @click="restore(b.id)">预览回滚</button>
        </div>
        <p v-if="!backups.length">还没有索引操作备份。</p>
      </div></Modal
    >
  </section>
</template>
