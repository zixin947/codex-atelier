<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import Icon from '../../shared/components/Icon.vue'
import Modal from '../../shared/components/Modal.vue'
import QuotaMeter from './QuotaMeter.vue'
import ResetCreditsPreview from './ResetCreditsPreview.vue'
import { accountPlanBadge } from './plan'
import { useAtelier } from '../../app/store'
import { accountsService as service } from '../../services/accounts'
import type {
  AccountLogin,
  AccountResult,
  AccountsSnapshot,
  AccountSwitch,
  CodexAccount,
  QuotaWindow,
} from '../../types/accounts'
import '../../styles/gateway.css'
import './accounts.css'

const store = useAtelier()
const refreshIntervals = [0, 30_000, 60_000, 180_000, 300_000]
function savedInterval() {
  try {
    const value = Number(localStorage.getItem('atelier.accounts.refreshInterval'))
    return refreshIntervals.includes(value) ? value : 0
  } catch {
    return 0
  }
}
const autoRefreshInterval = ref(savedInterval())
const clock = ref(Date.now())
let clockTimer: ReturnType<typeof setInterval> | undefined
let refreshTimer: ReturnType<typeof setTimeout> | undefined
const data = ref<AccountsSnapshot | null>(null),
  busy = ref(''),
  error = ref('')
const query = ref(''),
  filter = ref('all'),
  groupFilter = ref('all'),
  sort = ref('created'),
  ascending = ref(false)
const view = ref<'grid' | 'list'>('grid'),
  selected = ref<string[]>([])
const add = ref(false),
  addTab = ref<'oauth' | 'import'>('oauth'),
  loginName = ref(''),
  reauthId = ref<string>()
const login = ref<AccountLogin | null>(null),
  loginError = ref(''),
  importText = ref('')
const refreshingIds = ref<string[]>([])
const resetPreviewId = ref<string | null>(null)
const resetPreviewError = ref('')
const resetPreviewAccount = computed(() =>
  accounts.value.find((a) => a.id === resetPreviewId.value),
)
function previewResetCredits(account: CodexAccount) {
  resetPreviewError.value = ''
  resetPreviewId.value = account.id
}
async function refreshResetCredits() {
  const id = resetPreviewId.value
  if (!id || busy.value) return
  await action('reset-preview', async () => {
    resetPreviewError.value = ''
    try {
      apply(await service.resetCreditPreview(id))
    } catch (e) {
      resetPreviewError.value = String(e)
      try {
        apply(await service.snapshot())
      } catch {
        /* Preserve the visible cached preview. */
      }
    }
  })
}
const detectingCurrent = ref(false)
const detectionError = ref('')
let detectionTimer: ReturnType<typeof setInterval> | undefined
const reports = ref<AccountResult[]>([]),
  reportOpen = ref(false)
const editing = ref<CodexAccount | null>(null),
  editName = ref(''),
  editGroup = ref(''),
  editNote = ref(''),
  editRevision = ref('')
const removeIds = ref<string[]>([]),
  removeRevision = ref(''),
  exportOpen = ref(false)
const groupOpen = ref(false),
  bulkGroup = ref(''),
  groupRevision = ref('')
const switching = ref<AccountSwitch | null>(null),
  switchAcknowledged = ref(false)
let alive = true,
  timer: ReturnType<typeof setTimeout> | undefined
const canWrite = computed(() => !!data.value?.available && !busy.value && !detectingCurrent.value)
const accounts = computed(() => data.value?.accounts ?? [])
const groups = computed(() =>
  [...new Set(accounts.value.map((a) => a.group).filter(Boolean))].sort(),
)
const rows = computed(() =>
  accounts.value
    .filter((a) => {
      const matches = [a.name, a.email, a.group, a.note]
        .join(' ')
        .toLowerCase()
        .includes(query.value.trim().toLowerCase())
      return (
        matches &&
        (filter.value === 'all' ||
          (filter.value === 'ready'
            ? a.status === 'ready'
            : filter.value === 'current'
              ? a.id === data.value?.currentAccount.accountId
              : a.status !== 'ready')) &&
        (groupFilter.value === 'all' || a.group === groupFilter.value)
      )
    })
    .sort((a, b) => {
      let result =
        sort.value === 'name'
          ? (a.email ?? a.name).localeCompare(b.email ?? b.name)
          : sort.value === 'quota'
            ? (remaining(a) ?? -1) - (remaining(b) ?? -1)
            : a.createdAt - b.createdAt
      return ascending.value ? result : -result
    }),
)
const allSelected = computed(
  () => rows.value.length > 0 && rows.value.every((a) => selected.value.includes(a.id)),
)
const pending = computed(() => login.value?.state === 'pending')
function remaining(a: CodexAccount) {
  const values = [a.quota?.primary?.usedPercent, a.quota?.secondary?.usedPercent].filter(
    (n): n is number => typeof n === 'number',
  )
  return values.length ? 100 - Math.max(...values) : null
}
function quotaSlots(account: CodexAccount): [QuotaWindow | null, QuotaWindow | null] {
  const windows = [account.quota?.primary, account.quota?.secondary].filter(
    (w): w is QuotaWindow => !!w,
  )
  const slots: [QuotaWindow | null, QuotaWindow | null] = [
    windows.find((w) => w.durationMins === 300) ?? null,
    windows.find((w) => w.durationMins === 10080) ?? null,
  ]
  for (const window of windows) {
    if (slots.includes(window)) continue
    if (!slots[0]) slots[0] = window
    else if (!slots[1]) slots[1] = window
  }
  return slots
}
function toggleAccount(id: string) {
  selected.value = selected.value.includes(id)
    ? selected.value.filter((value) => value !== id)
    : [...selected.value, id]
}
async function copyAccount(account: CodexAccount) {
  await action('copy-account', async () => {
    await navigator.clipboard.writeText(account.email ?? account.name)
    store.notify('账号名称已复制')
  })
}
function isCurrent(account: CodexAccount) {
  return data.value?.currentAccount.accountId === account.id
}
function displayName(account: CodexAccount) {
  const alias =
    account.name !== account.email && !/^(Codex 账号|导入账号 [0-9]+)$/.test(account.name)
      ? account.name
      : null
  return alias ?? account.metadata.accountName ?? account.metadata.profileName ?? '未提供'
}
function subscriptionStatus(account: CodexAccount) {
  const m = account.metadata,
    old = m.error ? '（旧）' : ''
  if (m.isDelinquent === true) return '付款待处理' + old
  if (m.hasActiveSubscription === true)
    return (m.source === 'subscriptionApi' ? '有效订阅' : '快照有效') + old
  if (m.hasActiveSubscription === false)
    return (m.source === 'subscriptionApi' ? '无有效订阅' : '快照未激活') + old
  if (m.subscriptionExpiresAt) {
    if (m.subscriptionExpiresAt <= clock.value) return '到期待更新'
    return (m.source === 'subscriptionApi' ? '有效期内' : '快照未到期') + old
  }
  return m.error ? '获取失败' : account.plan ? '已识别套餐' : '未知'
}
function quotaStale(account: CodexAccount) {
  if (account.status !== 'ready' || !account.quota) return true
  return (
    clock.value - account.quota.checkedAt > 300_000 ||
    [account.quota.primary, account.quota.secondary].some(
      (window) => window?.resetsAt != null && window.resetsAt <= clock.value,
    )
  )
}
function metadataHint(account: CodexAccount) {
  const m = account.metadata
  const source =
    m.source === 'subscriptionApi'
      ? '订阅接口'
      : m.source === 'tokenSnapshot'
        ? '登录令牌快照'
        : '尚无来源'
  const renewal =
    m.subscriptionWillRenew == null
      ? '续订状态未知'
      : m.subscriptionWillRenew
        ? '将自动续订'
        : '不自动续订'
  return (
    source +
    (m.sourceAt ? ' · ' + date(m.sourceAt) : '') +
    ' · ' +
    renewal +
    (m.error ? ' · ' + m.error : '')
  )
}
function dialogOpen() {
  return (
    editing.value ||
    add.value ||
    switching.value ||
    removeIds.value.length ||
    exportOpen.value ||
    groupOpen.value ||
    reportOpen.value ||
    resetPreviewId.value ||
    pending.value
  )
}
async function detectCurrent() {
  if (
    !alive ||
    !data.value?.available ||
    busy.value ||
    detectingCurrent.value ||
    dialogOpen() ||
    document.visibilityState !== 'visible'
  )
    return
  detectingCurrent.value = true
  detectionError.value = ''
  try {
    const value = await service.detectCurrent()
    if (alive) apply(value)
  } catch (e) {
    if (alive) detectionError.value = String(e)
  } finally {
    detectingCurrent.value = false
  }
}
function detectOnReturn() {
  void detectCurrent()
}
function scheduleAutoRefresh() {
  clearTimeout(refreshTimer)
  if (!alive || !autoRefreshInterval.value) return
  refreshTimer = setTimeout(async () => {
    if (
      !busy.value &&
      !detectingCurrent.value &&
      document.visibilityState === 'visible' &&
      accounts.value.length &&
      !editing.value &&
      !add.value &&
      !switching.value &&
      !removeIds.value.length &&
      !exportOpen.value &&
      !groupOpen.value &&
      !reportOpen.value &&
      !resetPreviewId.value
    )
      await refresh(
        accounts.value.map((a) => a.id),
        true,
      )
    scheduleAutoRefresh()
  }, autoRefreshInterval.value)
}
watch(autoRefreshInterval, (value) => {
  try {
    localStorage.setItem('atelier.accounts.refreshInterval', String(value))
  } catch {
    /* Refresh still works for this page. */
  }
  scheduleAutoRefresh()
})
function date(value: number) {
  return new Date(value).toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  })
}
function apply(value: AccountsSnapshot) {
  data.value = value
  if (value.pendingLogin) {
    login.value = value.pendingLogin
    schedule()
  }
  selected.value = selected.value.filter((id) => value.accounts.some((a) => a.id === id))
}
function toggleAll() {
  selected.value = allSelected.value
    ? selected.value.filter((id) => !rows.value.some((a) => a.id === id))
    : [...new Set([...selected.value, ...rows.value.map((a) => a.id)])]
}
function resetFilter() {
  query.value = ''
  filter.value = 'all'
  groupFilter.value = 'all'
}
async function action(key: string, fn: () => Promise<void>) {
  if (busy.value || detectingCurrent.value) return
  busy.value = key
  error.value = ''
  try {
    await fn()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function read() {
  await action('read', async () => {
    apply(await service.snapshot())
  })
  await detectCurrent()
}
function showReport(results: AccountResult[]) {
  reports.value = results
  reportOpen.value = true
}
async function refresh(ids: string[], quiet = false) {
  if (!ids.length) return
  await action('quota', async () => {
    refreshingIds.value = [...new Set(ids)]
    try {
      const result = await service.refresh(ids, !quiet)
      apply(result.snapshot)
      if (!quiet && result.results.some((item) => !item.ok)) showReport(result.results)
      else if (!quiet) store.notify('账号信息与额度已刷新')
    } finally {
      refreshingIds.value = []
    }
  })
}
function openAdd(mode: 'oauth' | 'import' = 'oauth', account?: CodexAccount) {
  if (pending.value) {
    add.value = true
    return
  }
  addTab.value = mode
  add.value = true
  loginName.value = account?.name ?? ''
  reauthId.value = account?.id
  login.value = null
  loginError.value = ''
  importText.value = ''
}
function closeAdd() {
  if (busy.value) return
  add.value = false
  importText.value = ''
}
function schedule() {
  clearTimeout(timer)
  if (alive && pending.value) timer = setTimeout(() => void poll(), 1500)
}
async function poll() {
  if (!login.value || !alive) return
  try {
    const value = await service.poll(login.value.id)
    if (!alive) return
    login.value = value
    loginError.value = ''
    if (value.state === 'completed') {
      apply(await service.snapshot())
      store.notify('Codex 账号已保存')
    }
    schedule()
  } catch (e) {
    loginError.value = String(e)
  }
}
async function beginLogin() {
  await action('login', async () => {
    login.value = await service.login(loginName.value.trim() || 'Codex 账号', reauthId.value)
    schedule()
  })
}
async function cancelLogin() {
  if (!login.value) return
  const id = login.value.id
  await action('cancel', async () => {
    await service.cancel(id)
    clearTimeout(timer)
    login.value = await service.poll(id)
  })
}
async function openBrowser() {
  if (login.value) await action('open', async () => service.open(login.value!.id))
}
async function copyUrl() {
  if (!login.value?.url) return
  await action('copy', async () => {
    await navigator.clipboard.writeText(login.value!.url!)
    store.notify('授权链接已复制')
  })
}
async function chooseFiles(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files?.length) return
  const files = [...input.files]
  await action('files', async () => {
    if (files.length > 64 || files.reduce((n, f) => n + f.size, 0) > 4 * 1024 * 1024)
      throw new Error('最多选择 64 个文件，总计不超过 4 MiB。')
    const items: unknown[] = []
    for (const file of files) {
      const value: unknown = JSON.parse(await file.text())
      if (Array.isArray(value)) items.push(...value)
      else if (
        value &&
        typeof value === 'object' &&
        'accounts' in value &&
        Array.isArray(value.accounts)
      )
        items.push(...value.accounts)
      else items.push(value)
    }
    importText.value = JSON.stringify(items)
  })
  input.value = ''
}
async function importAccounts() {
  await action('import', async () => {
    const result = await service.import(importText.value)
    apply(result.snapshot)
    importText.value = ''
    add.value = false
    showReport(result.results)
  })
}
function reauthorize() {
  const account = editing.value
  if (account) {
    editing.value = null
    openAdd('oauth', account)
  }
}
function edit(a: CodexAccount) {
  editing.value = a
  editName.value = a.name
  editGroup.value = a.group
  editNote.value = a.note
  editRevision.value = data.value!.revision
}
async function saveEdit() {
  if (!editing.value) return
  await action('edit', async () => {
    apply(
      await service.edit(
        {
          id: editing.value!.id,
          name: editName.value,
          group: editGroup.value,
          note: editNote.value,
        },
        editRevision.value,
      ),
    )
    editing.value = null
  })
}
function askRemove(ids: string[]) {
  removeIds.value = [...ids]
  removeRevision.value = data.value!.revision
}
async function remove() {
  await action('remove', async () => {
    const result = await service.remove(removeIds.value, removeRevision.value)
    apply(result.snapshot)
    removeIds.value = []
    showReport(result.results)
  })
}
function askGroup() {
  groupOpen.value = true
  bulkGroup.value = ''
  groupRevision.value = data.value!.revision
}
async function saveGroup() {
  await action('group', async () => {
    apply(await service.group([...selected.value], bulkGroup.value, groupRevision.value))
    groupOpen.value = false
  })
}
async function prepareSwitch(id: string | null) {
  await action('preview', async () => {
    switching.value = await service.preview(id)
    switchAcknowledged.value = false
  })
}
async function applySwitch() {
  if (!switching.value || !switchAcknowledged.value) return
  const ticket = switching.value.ticket,
    rollback = switching.value.rollback
  await action('switch', async () => {
    apply(await service.apply(ticket))
    switching.value = null
    store.notify(
      rollback
        ? '文件凭据已回退；请重新打开使用该目录的客户端'
        : '文件凭据已切换；请重新打开使用该目录的客户端',
    )
  })
}
async function exportAccounts() {
  await action('export', async () => {
    const path = await service.export(
      selected.value.length ? [...selected.value] : accounts.value.map((a) => a.id),
    )
    exportOpen.value = false
    store.notify(`备份已保存：${path}`)
  })
}
onMounted(() => {
  void read()
  detectionTimer = setInterval(detectOnReturn, 10_000)
  window.addEventListener('focus', detectOnReturn)
  document.addEventListener('visibilitychange', detectOnReturn)
  clockTimer = setInterval(() => {
    clock.value = Date.now()
  }, 30_000)
  scheduleAutoRefresh()
})
onUnmounted(() => {
  alive = false
  clearTimeout(timer)
  clearTimeout(refreshTimer)
  clearInterval(clockTimer)
  clearInterval(detectionTimer)
  window.removeEventListener('focus', detectOnReturn)
  document.removeEventListener('visibilitychange', detectOnReturn)
  importText.value = ''
})
</script>
<template>
  <div class="accounts-workspace">
    <div class="page account-page">
      <h1 class="account-title">Codex 账号管理</h1>
      <div v-if="error" class="error-state" role="alert">
        {{ error }}<button class="button" :disabled="!!busy" @click="read">刷新列表</button>
      </div>
      <p v-if="data && !data.available" class="account-note inset">{{ data.reason }}</p>
      <div class="account-toolbar panel" aria-label="账号工具栏">
        <label class="search"
          ><Icon name="search" :size="16" /><input
            v-model="query"
            aria-label="搜索账号"
            placeholder="搜索账号…"
        /></label>
        <div class="account-view-toggle" aria-label="视图">
          <button
            class="icon-button"
            :class="{ active: view === 'list' }"
            :aria-pressed="view === 'list'"
            title="列表视图"
            aria-label="列表视图"
            @click="view = 'list'"
          >
            <Icon name="list" /></button
          ><button
            class="icon-button"
            :class="{ active: view === 'grid' }"
            :aria-pressed="view === 'grid'"
            title="卡片视图"
            aria-label="卡片视图"
            @click="view = 'grid'"
          >
            <Icon name="grid" />
          </button>
        </div>
        <select v-model="filter" aria-label="账号状态">
          <option value="all">全部 ({{ accounts.length }})</option>
          <option value="ready">额度已同步</option>
          <option value="attention">待处理</option>
          <option value="current">当前账号</option>
        </select>
        <select v-model="sort" aria-label="账号排序">
          <option value="created">按创建时间</option>
          <option value="name">按账号名称</option>
          <option value="quota">按剩余额度</option>
        </select>
        <button
          class="icon-button"
          :aria-label="ascending ? '改为降序' : '改为升序'"
          :title="ascending ? '升序' : '降序'"
          @click="ascending = !ascending"
        >
          <Icon name="arrow" class="account-sort-arrow" :class="{ ascending }" />
        </button>
        <div class="account-toolbar-actions">
          <button
            class="icon-button primary"
            :disabled="!canWrite"
            title="添加账号"
            aria-label="添加账号"
            @click="openAdd()"
          >
            <Icon name="plus" />
          </button>
          <button
            class="icon-button"
            :disabled="!canWrite || !accounts.length"
            title="刷新全部额度"
            aria-label="刷新全部额度"
            @click="refresh(accounts.map((a) => a.id))"
          >
            <Icon name="refresh" :class="{ spinning: busy === 'quota' }" />
          </button>
          <button
            class="icon-button"
            :disabled="!canWrite || !accounts.length"
            title="导出账号"
            aria-label="导出账号"
            @click="exportOpen = true"
          >
            <Icon name="download" />
          </button>
          <button
            class="icon-button"
            :disabled="!canWrite"
            title="导入账号"
            aria-label="导入账号"
            @click="openAdd('import')"
          >
            <Icon name="upload" />
          </button>
        </div>
      </div>
      <div class="account-current-bar" role="status">
        <span
          :title="
            [data?.currentAccount.path, data?.currentAccount.message].filter(Boolean).join('\n')
          "
        >
          <Icon v-if="detectingCurrent" name="refresh" :size="12" class="spinning" />
          {{
            detectingCurrent
              ? '正在自动检测当前登录…'
              : detectionError
                ? '自动检测失败'
                : data?.currentAccount.state === 'matched'
                  ? '当前文件账号 · 自动识别'
                  : data?.currentAccount.state === 'unavailable'
                    ? '暂未识别到文件登录'
                    : '等待自动检测'
          }}
        </span>
        <button class="button" :disabled="!canWrite" @click="detectCurrent">重新检测</button>
        <label
          >自动刷新<select v-model.number="autoRefreshInterval" aria-label="账号自动刷新频率">
            <option :value="0">手动</option>
            <option :value="30000">30 秒</option>
            <option :value="60000">1 分钟</option>
            <option :value="180000">3 分钟</option>
            <option :value="300000">5 分钟</option>
          </select></label
        >
      </div>
      <p
        v-if="detectionError || data?.currentAccount.state === 'unavailable'"
        class="account-detection-note"
      >
        {{ detectionError || data?.currentAccount.message }}
      </p>
      <div v-if="pending" class="account-pending" role="status">
        <span>有一项账号登录正在等待授权</span
        ><button class="button" @click="add = true">继续登录</button>
      </div>
      <div
        v-if="accounts.length && (selected.length || groups.length || data?.rollbackAvailable)"
        class="account-batchbar"
      >
        <label class="account-check"
          ><input
            type="checkbox"
            :checked="allSelected"
            :indeterminate="selected.length > 0 && !allSelected"
            :disabled="!!busy"
            @change="toggleAll"
          />全选当前结果</label
        >
        <template v-if="selected.length"
          ><span>已选 {{ selected.length }} 项</span
          ><button class="button" :disabled="!canWrite" @click="refresh([...selected])">
            刷新额度</button
          ><button class="button" :disabled="!canWrite" @click="askGroup">分组</button
          ><button class="button" :disabled="!canWrite" @click="askRemove(selected)">删除</button
          ><button class="button" :disabled="!!busy" @click="selected = []">
            取消选择
          </button></template
        >
        <select v-if="groups.length" v-model="groupFilter" aria-label="分组筛选">
          <option value="all">全部分组</option>
          <option v-for="group in groups" :key="group" :value="group">{{ group }}</option>
        </select>
        <button
          v-if="data?.rollbackAvailable"
          class="account-rollback"
          :disabled="!canWrite"
          @click="prepareSwitch(null)"
        >
          回退上次切换
        </button>
      </div>
      <div v-if="!data" class="empty" role="status">正在读取账号列表…</div>
      <div v-else-if="!accounts.length" class="account-empty">
        <Icon name="user" :size="32" />
        <h2>暂无 Codex 账号</h2>
        <p>添加账号，或导入你选择的 auth.json 文件。</p>
        <button class="button primary" :disabled="!canWrite" @click="openAdd()">添加账号</button>
      </div>
      <div v-else-if="!rows.length" class="account-empty">
        <h2>没有匹配的账号</h2>
        <button class="button" @click="resetFilter">清除筛选</button>
      </div>
      <div v-else class="account-cards" :class="{ 'account-list': view === 'list' }">
        <article
          v-for="account in rows"
          :key="account.id"
          class="account-card panel"
          :data-status="account.status"
          :aria-busy="refreshingIds.includes(account.id)"
          :class="{
            current: isCurrent(account),
            selected: selected.includes(account.id),
          }"
        >
          <header class="account-card-top">
            <div class="account-email-row">
              <button
                class="account-select-dot"
                :class="{
                  checked: selected.includes(account.id),
                  ready: account.status === 'ready',
                }"
                role="checkbox"
                :aria-checked="selected.includes(account.id)"
                :aria-label="'选择 ' + account.name"
                :disabled="!!busy"
                @click="toggleAccount(account.id)"
              >
                <span />
              </button>
              <button
                class="account-identity"
                :title="account.email ?? account.name"
                @click="edit(account)"
              >
                {{ account.email || account.name }}
              </button>
              <button
                class="account-edit"
                :disabled="!!busy"
                :aria-label="'编辑 ' + account.name"
                title="编辑账号"
                @click="edit(account)"
              >
                <Icon name="pencil" :size="12" />
              </button>
            </div>
            <div class="account-badges">
              <span
                class="account-plan"
                :class="'account-plan-' + accountPlanBadge(account).tone"
                :title="accountPlanBadge(account).title"
              >
                <Icon name="gem" :size="11" />{{ accountPlanBadge(account).label }}
              </span>
              <span
                class="account-current"
                :title="
                  isCurrent(account)
                    ? data.currentAccount.message
                    : '已保存到 Atelier 的独立账号档案'
                "
                ><i />{{ isCurrent(account) ? '当前账号' : '已添加' }}</span
              >
              <button
                class="account-reset-count"
                :disabled="!!busy"
                :aria-label="
                  '预览重置卡，可用数量' + (account.quota?.resetCredits?.availableCount ?? '未知')
                "
                title="预览重置卡数量与有效期"
                @click="previewResetCredits(account)"
              >
                <Icon name="ticket" :size="10" />{{
                  account.quota?.resetCredits?.availableCount ?? '—'
                }}
              </button>
            </div>
          </header>
          <dl class="account-detail-grid">
            <div>
              <Icon name="user" :size="12" />
              <dt>用户昵称</dt>
              <dd :title="account.metadata.accountName ?? account.name">
                {{ displayName(account) }}
              </dd>
            </div>
            <div>
              <Icon name="shield" :size="12" />
              <dt>认证模式</dt>
              <dd>ChatGPT OAuth</dd>
            </div>
            <div>
              <Icon name="subscription" :size="12" />
              <dt>订阅状态</dt>
              <dd :title="metadataHint(account)">{{ subscriptionStatus(account) }}</dd>
            </div>
            <div>
              <Icon name="calendar" :size="12" />
              <dt>到期时间</dt>
              <dd :title="metadataHint(account)">
                {{
                  account.metadata.subscriptionExpiresAt
                    ? date(account.metadata.subscriptionExpiresAt)
                    : '—'
                }}
              </dd>
            </div>
          </dl>
          <div class="account-card-quotas">
            <QuotaMeter
              v-for="(window, index) in quotaSlots(account)"
              :key="index"
              :window="window"
              :now="clock"
              :checked-at="account.quota?.checkedAt"
              :fallback="index === 0 ? '5小时' : '1周'"
              :stale="quotaStale(account)"
              :failed="account.status === 'error'"
              :loading="refreshingIds.includes(account.id)"
            />
          </div>
          <footer class="account-card-bottom">
            <span
              v-if="refreshingIds.includes(account.id)"
              class="account-footer-status"
              role="status"
              ><Icon name="refresh" :size="12" class="spinning" />正在刷新</span
            >
            <span
              v-else-if="isCurrent(account)"
              class="account-footer-status"
              :title="data.currentAccount.message"
              ><Icon name="check" :size="13" />当前账号</span
            >
            <button
              v-else
              class="account-switch-action"
              :disabled="!canWrite"
              @click="prepareSwitch(account.id)"
            >
              <Icon name="play" :size="12" />切换账号
            </button>
            <div class="account-card-actions">
              <button
                class="icon-button"
                :disabled="!canWrite"
                :aria-label="'刷新 ' + account.name + ' 额度'"
                title="刷新额度"
                @click="refresh([account.id])"
              >
                <Icon
                  name="refresh"
                  :size="14"
                  :class="{ spinning: refreshingIds.includes(account.id) }"
                />
              </button>
              <button
                class="icon-button"
                :disabled="!canWrite"
                :aria-label="'复制 ' + account.name"
                title="复制账号名称"
                @click="copyAccount(account)"
              >
                <Icon name="copy" :size="14" />
              </button>
              <button
                class="icon-button"
                :disabled="!canWrite || isCurrent(account)"
                :aria-label="'删除 ' + account.name"
                title="删除账号"
                @click="askRemove([account.id])"
              >
                <Icon name="trash" :size="14" />
              </button>
            </div>
          </footer>
          <p v-if="account.error" class="account-card-error" :title="account.error">
            {{ account.error }}
          </p>
        </article>
      </div>
      <p class="account-footnote">
        当前文件登录会自动加入列表，仅将副本保存到
        Atelier。额度需主动刷新，缺失值显示未知；不会修改客户端登录。
      </p>
    </div>
    <ResetCreditsPreview
      v-if="resetPreviewAccount"
      :account="resetPreviewAccount"
      :loading="busy === 'reset-preview'"
      :error="resetPreviewError"
      @close="!busy && (resetPreviewId = null)"
      @refresh="refreshResetCredits"
    />
    <Modal v-if="add" title="添加 Codex 账号" @close="closeAdd">
      <div class="account-dialog-tabs">
        <button
          :class="{ active: addTab === 'oauth' }"
          :disabled="!!busy || pending"
          @click="addTab = 'oauth'"
        >
          OAuth 授权</button
        ><button
          :class="{ active: addTab === 'import' }"
          :disabled="!!busy || pending || !!reauthId"
          @click="addTab = 'import'"
        >
          Token / JSON 导入
        </button>
      </div>
      <template v-if="addTab === 'oauth'">
        <label v-if="!pending && login?.state !== 'completed'" class="gw-field"
          ><span>账号名称</span
          ><input
            v-model="loginName"
            maxlength="80"
            :disabled="!!busy"
            placeholder="例如：日常工作"
        /></label>
        <p class="muted">
          通过官方 Codex 登录流程授权，凭据保存在 Atelier 的独立账号目录。不会替换当前客户端登录。
        </p>
        <div v-if="pending" class="account-login-box inset">
          <span class="badge orange">等待浏览器授权</span>
          <p>请在浏览器完成登录。窗口可以关闭，授权最多等待 10 分钟。</p>
          <div class="gw-inline-actions">
            <button class="button primary" :disabled="!!busy" @click="openBrowser">
              打开授权页面</button
            ><button class="button" :disabled="!!busy" @click="copyUrl">复制链接</button>
          </div>
        </div>
        <p v-if="login?.message" role="status">{{ login.message }}</p>
        <p v-if="loginError" class="error-state" role="alert">{{ loginError }}</p>
        <div class="modal-actions">
          <button class="button" :disabled="!!busy" @click="closeAdd">关闭</button
          ><button v-if="pending" class="button" :disabled="!!busy" @click="cancelLogin">
            取消授权</button
          ><button v-if="pending && loginError" class="button" @click="poll">重新检查</button
          ><button
            v-if="!pending && login?.state !== 'completed'"
            class="button primary"
            :disabled="!canWrite"
            @click="beginLogin"
          >
            {{ busy === 'login' ? '正在准备…' : '开始授权' }}
          </button>
        </div>
      </template>
      <template v-else-if="addTab === 'import'">
        <p class="muted">
          选择单个或多个 Codex auth.json，或粘贴同结构 JSON／JSON
          数组。仅导入主动提供的文件，不扫描客户端。
        </p>
        <label class="gw-field"
          ><span>选择 JSON 文件</span
          ><input
            type="file"
            accept=".json,application/json"
            multiple
            :disabled="!!busy"
            @change="chooseFiles"
        /></label>
        <label class="gw-field"
          ><span>凭据 JSON <small>包含敏感令牌，请勿截图或分享</small></span
          ><textarea
            v-model="importText"
            rows="7"
            spellcheck="false"
            autocomplete="off"
            :disabled="!!busy"
            placeholder='{"tokens":{"access_token":"…","refresh_token":"…","id_token":"…","account_id":"…"}}'
          />
        </label>
        <p class="tiny muted">
          不接受单独 Refresh Token；重复账号不会覆盖。导入后需刷新额度核实身份。
        </p>
        <div class="modal-actions">
          <button class="button" :disabled="!!busy" @click="closeAdd">取消</button
          ><button
            class="button primary"
            :disabled="!canWrite || !importText.trim()"
            @click="importAccounts"
          >
            {{ busy === 'import' ? '正在导入…' : '导入账号' }}
          </button>
        </div>
      </template>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
    </Modal>
    <Modal v-if="editing" title="账号详情" @close="!busy && (editing = null)">
      <label class="gw-field"
        ><span>名称</span><input v-model="editName" maxlength="80" :disabled="!!busy" /></label
      ><label class="gw-field"
        ><span>分组</span
        ><input
          v-model="editGroup"
          maxlength="40"
          :disabled="!!busy"
          list="account-groups" /><datalist id="account-groups">
          <option v-for="group in groups" :key="group" :value="group" /></datalist></label
      ><label class="gw-field"
        ><span>备注</span
        ><textarea v-model="editNote" maxlength="500" rows="3" :disabled="!!busy" />
      </label>
      <dl class="account-metadata-details inset">
        <dt>真实账户名称</dt>
        <dd>{{ editing.metadata.accountName ?? '未知' }}</dd>
        <dt>工作区</dt>
        <dd>{{ editing.metadata.workspaceName ?? '未知' }}</dd>
        <dt>订阅来源</dt>
        <dd>{{ metadataHint(editing) }}</dd>
        <dt>当前账号核对</dt>
        <dd>{{ data?.currentAccount.message }}</dd>
        <dt>最近额度更新</dt>
        <dd>{{ editing.quota ? date(editing.quota.checkedAt) : '尚未查询' }}</dd>
      </dl>
      <p v-if="editing.metadata.error" class="error-state">{{ editing.metadata.error }}</p>
      <p v-if="editing.error" class="error-state">{{ editing.error }}</p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!canWrite || pending" @click="reauthorize">
          重新授权</button
        ><button class="button" :disabled="!!busy" @click="editing = null">关闭</button
        ><button class="button primary" :disabled="!canWrite || !editName.trim()" @click="saveEdit">
          保存
        </button>
      </div>
    </Modal>
    <Modal v-if="removeIds.length" title="删除账号" @close="!busy && (removeIds = [])"
      ><h2>删除 {{ removeIds.length }} 个账号？</h2>
      <p>
        仅移除 Atelier 内的档案与凭据，不退出已登录的客户端，也不撤销官方授权。操作不能在这里撤销。
      </p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="removeIds = []">取消</button
        ><button class="button" :disabled="!canWrite" @click="remove">确认删除</button>
      </div></Modal
    >
    <Modal v-if="groupOpen" title="批量分组" @close="!busy && (groupOpen = false)"
      ><label class="gw-field"
        ><span>为 {{ selected.length }} 个账号设置分组，留空则清除</span
        ><input v-model="bulkGroup" maxlength="40" :disabled="!!busy"
      /></label>
      <p v-if="error" class="error-state">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="groupOpen = false">取消</button
        ><button class="button primary" :disabled="!canWrite" @click="saveGroup">保存分组</button>
      </div></Modal
    >
    <Modal
      v-if="switching"
      :title="switching.rollback ? '回退账号切换' : '切换账号预览'"
      @close="!busy && (switching = null)"
      ><p class="muted">{{ switching.warning }}</p>
      <dl class="account-switch-diff inset">
        <dt>目标文件</dt>
        <dd>{{ switching.target }}</dd>
        <dt>当前</dt>
        <dd>{{ switching.before }}</dd>
        <dt>切换后</dt>
        <dd>{{ switching.after }}</dd>
      </dl>
      <p class="tiny muted">
        预览有效至
        {{
          date(switching.expiresAt)
        }}。确认时检查外部变更，备份原文件并读回验证；不改模型路由和会话数据库。
      </p>
      <label class="account-check"
        ><input
          v-model="switchAcknowledged"
          type="checkbox"
          :disabled="!!busy"
        />我已保存工作并退出使用此目录的客户端，确认修改上述文件</label
      >
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="switching = null">取消</button
        ><button
          class="button primary"
          :disabled="!canWrite || !switchAcknowledged"
          @click="applySwitch"
        >
          {{ switching.rollback ? '确认回退' : '备份并切换' }}
        </button>
      </div></Modal
    >
    <Modal v-if="exportOpen" title="导出账号备份" @close="!busy && (exportOpen = false)"
      ><h2>导出 {{ selected.length || accounts.length }} 个账号</h2>
      <p>
        备份包含可用于登录的明文令牌。只保存到你信任的位置，不上传仓库或分享。继续后会打开系统保存对话框。
      </p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="exportOpen = false">取消</button
        ><button class="button primary" :disabled="!canWrite" @click="exportAccounts">
          选择位置并导出
        </button>
      </div></Modal
    >
    <Modal v-if="reportOpen" title="操作结果" @close="reportOpen = false"
      ><p>
        {{ reports.filter((r) => r.ok).length }} 项成功，{{ reports.filter((r) => !r.ok).length }}
        项失败。未列出的项目未执行。
      </p>
      <ul class="account-results">
        <li v-for="(r, i) in reports" :key="i">
          <Icon :name="r.ok ? 'check' : 'alert'" :size="16" /><span
            ><b>{{ accounts.find((a) => a.id === r.id)?.name ?? `项目 ${i + 1}` }}</b
            >{{ r.message }}</span
          >
        </li>
      </ul>
      <button class="button primary" @click="reportOpen = false">完成</button></Modal
    >
  </div>
</template>
