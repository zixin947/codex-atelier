<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import Panel from '../../shared/components/Panel.vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import { useAtelier } from '../../app/store'
import { gatewayService } from '../../services/gateway'
import type {
  GatewayProfile,
  GatewayService,
  GatewaySnapshot,
  LoginAttempt,
} from '../../types/gateway'
import '../../styles/gateway.css'

const props = defineProps<{ service?: GatewayService }>()
const service = props.service ?? gatewayService
const store = useAtelier()
const snapshot = ref<GatewaySnapshot | null>(null)
const busy = ref('')
const error = ref('')
const query = ref('')
const loginOpen = ref(false)
const loginName = ref('')
const loginAccountId = ref<string>()
const login = ref<LoginAttempt | null>(null)
const loginError = ref('')
const polling = ref(false)
const editing = ref<GatewayProfile | null>(null)
const editName = ref('')
const editPriority = ref(0)
const editRevision = ref('')
const removal = ref<GatewayProfile | null>(null)
const signout = ref<GatewayProfile | null>(null)
let timer: ReturnType<typeof setTimeout> | undefined
let alive = true

const accounts = computed(() => snapshot.value?.profiles.filter((p) => p.kind === 'chatgpt') ?? [])
const filtered = computed(() =>
  accounts.value.filter((p) =>
    `${p.name} ${p.email ?? ''}`.toLowerCase().includes(query.value.trim().toLowerCase()),
  ),
)
const preferred = computed(() =>
  snapshot.value?.profiles.find((p) => p.id === snapshot.value?.settings.preferredProfileId),
)
const readyCount = computed(
  () => accounts.value.filter((p) => p.enabled && p.authStatus === 'ready').length,
)
const authUrl = computed(() => {
  try {
    const url = new URL(login.value?.authUrl ?? '')
    return url.protocol === 'https:' && ['auth.openai.com', 'chatgpt.com'].includes(url.hostname)
      ? url.href
      : null
  } catch {
    return null
  }
})
const canMutate = computed(() => snapshot.value?.available === true && !busy.value)
const pending = computed(() => login.value?.state === 'pending')
function label(profile: GatewayProfile) {
  if (!profile.enabled) return '已停用'
  if (profile.cooldownUntil && profile.cooldownUntil > Date.now()) return '冷却中'
  return {
    ready: '已授权',
    needsLogin: '需要登录',
    planDisabled: '套餐访问未启用',
    unknown: '状态待确认',
  }[profile.authStatus]
}
function time(value: number) {
  return new Date(value).toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}
function count(value: number) {
  return new Intl.NumberFormat('zh-CN').format(value)
}
async function refresh() {
  if (busy.value) return
  busy.value = 'refresh'
  error.value = ''
  try {
    snapshot.value = await service.snapshot()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function change(id: string, action: () => Promise<GatewaySnapshot>, message: string) {
  if (!canMutate.value) return
  busy.value = id
  error.value = ''
  try {
    snapshot.value = await action()
    editing.value = null
    removal.value = null
    signout.value = null
    store.notify(message)
  } catch (e) {
    error.value = String(e)
    // Readback exposes any partial result without pretending the operation succeeded.
    try {
      snapshot.value = await service.snapshot()
    } catch {
      /* Preserve last known state. */
    }
  } finally {
    busy.value = ''
  }
}
function edit(profile: GatewayProfile) {
  editing.value = profile
  editName.value = profile.name
  editPriority.value = profile.priority
  editRevision.value = snapshot.value?.revision ?? ''
  error.value = ''
}
function saveEdit() {
  if (!editing.value || !editName.value.trim()) return
  const profile = editing.value
  void change(
    profile.id,
    () =>
      service.updateProfile(
        {
          id: profile.id,
          name: editName.value.trim(),
          enabled: profile.enabled,
          priority: editPriority.value,
        },
        editRevision.value,
      ),
    '账号档案已保存',
  )
}
function toggle(profile: GatewayProfile) {
  const revision = snapshot.value?.revision ?? ''
  void change(
    profile.id,
    () =>
      service.updateProfile(
        {
          id: profile.id,
          name: profile.name,
          enabled: !profile.enabled,
          priority: profile.priority,
        },
        revision,
      ),
    `${profile.name} ${profile.enabled ? '已退出候选账号池' : '已加入候选账号池'}`,
  )
}
function choose(profile: GatewayProfile) {
  const revision = snapshot.value?.revision ?? ''
  void change(
    profile.id,
    () => service.selectProfile(profile.id, revision),
    `当前账号已设为 ${profile.name}`,
  )
}
function openLogin(profile?: GatewayProfile) {
  if (pending.value) {
    loginOpen.value = true
    return
  }
  login.value = null
  loginError.value = ''
  loginName.value = profile?.name ?? ''
  loginAccountId.value = profile?.id
  loginOpen.value = true
}
function schedulePoll() {
  clearTimeout(timer)
  if (alive && pending.value)
    timer = setTimeout(() => {
      void pollLogin()
    }, 2000)
}
async function pollLogin() {
  if (!login.value || polling.value || !alive) return
  polling.value = true
  try {
    const result = await service.pollLogin(login.value.id)
    if (!alive) return
    login.value = result
    loginError.value = ''
    if (result.state === 'completed') {
      snapshot.value = await service.snapshot()
      store.notify('ChatGPT 账号已连接；使用额度以官方页面为准')
    } else if (result.state === 'pending') schedulePoll()
  } catch (e) {
    if (alive) loginError.value = `授权结果尚未确认：${String(e)}`
  } finally {
    polling.value = false
  }
}
async function beginLogin() {
  if (!canMutate.value || pending.value) return
  busy.value = 'login'
  loginError.value = ''
  try {
    login.value = await service.beginLogin(
      loginName.value.trim() || 'ChatGPT 账号',
      loginAccountId.value,
    )
    schedulePoll()
  } catch (e) {
    loginError.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function cancelLogin() {
  if (!login.value || busy.value) return
  busy.value = 'cancel-login'
  loginError.value = ''
  clearTimeout(timer)
  try {
    await service.cancelLogin(login.value.id)
    login.value = { ...login.value, state: 'cancelled', authUrl: null, message: '本次授权已取消。' }
  } catch (e) {
    loginError.value = String(e)
    schedulePoll()
  } finally {
    busy.value = ''
  }
}
async function launchLogin() {
  if (!login.value) return
  try {
    await service.openLogin(login.value.id)
  } catch (e) {
    loginError.value = String(e)
  }
}
async function copyAuthUrl() {
  if (!authUrl.value) return
  try {
    await navigator.clipboard.writeText(authUrl.value)
    store.notify('官方授权链接已复制')
  } catch {
    loginError.value = '复制失败，请使用下方官方授权链接。'
  }
}
function confirmRemoval() {
  if (!removal.value) return
  const profile = removal.value,
    revision = snapshot.value?.revision ?? ''
  void change(
    profile.id,
    () => service.removeProfile(profile.id, revision),
    '账号已从 Atelier 移除',
  )
}
function confirmSignout() {
  if (!signout.value) return
  const profile = signout.value
  void change(profile.id, () => service.signOut(profile.id), '此账号在 Atelier 的授权已退出')
}
onMounted(() => {
  void refresh()
})
onUnmounted(() => {
  alive = false
  clearTimeout(timer)
})
</script>

<template>
  <div class="page gateway-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / ACCOUNTS & USAGE</p>
        <h1>账号归位，专注继续<span class="heading-dot">.</span></h1>
        <p class="subtitle">
          管理网关使用的独立官方授权。<RouterLink to="/accounts">返回 Codex 账号管理</RouterLink>
        </p>
      </div>
      <div class="gw-heading-actions">
        <button class="button" :disabled="!!busy" @click="refresh">
          <Icon
            name="refresh"
            :size="16"
            :class="{ spinning: busy === 'refresh' }"
          />刷新状态</button
        ><button class="button primary" :disabled="!canMutate" @click="openLogin()">
          <Icon name="user" :size="16" />{{ pending ? '继续授权' : '添加订阅账号' }}
        </button>
      </div>
    </div>
    <div v-if="error" role="alert" class="error-state">{{ error }}</div>
    <div v-if="snapshot && !snapshot.available" role="status" class="gw-native-note inset">
      <Icon name="terminal" :size="19" /><span>{{ snapshot.reason }}</span>
    </div>
    <div v-if="!snapshot" class="empty" role="status">正在读取 Atelier 账号档案…</div>
    <template v-else>
      <section class="gw-account-overview" aria-label="订阅账号概况">
        <div class="gw-console gw-account-console">
          <div class="gw-console-head">
            <span>CHATGPT / ACCOUNT DIRECTORY</span
            ><span class="gw-console-status"
              ><i class="dot" />{{ snapshot.available ? 'LOCAL DATA' : 'NOT CONNECTED' }}</span
            >
          </div>
          <div class="gw-account-console-body">
            <div>
              <p class="gw-screen-label">已添加订阅账号</p>
              <strong class="gw-large-number">{{
                snapshot.available ? String(accounts.length).padStart(2, '0') : '—'
              }}</strong
              ><span class="gw-number-unit">ACCOUNTS</span>
            </div>
            <div class="gw-console-side">
              <b>{{ snapshot.available ? `${readyCount} 个已启用且授权就绪` : '等待桌面连接' }}</b>
              <p>你决定使用哪个账号。<br />自动切换在「模型与连接」中单独开启。</p>
            </div>
          </div>
          <div class="gw-console-foot">
            <span>当前路由</span
            ><b>{{
              preferred
                ? `${preferred.name}${preferred.kind === 'apiKey' ? ' · API Provider' : ''}`
                : '尚未选择账号'
            }}</b
            ><RouterLink to="/connections">管理路由 <Icon name="arrow" :size="14" /></RouterLink>
          </div>
        </div>
        <Panel title="套餐用量" code="OFFICIAL USAGE"
          ><div class="gw-usage-intro">
            <Icon name="pulse" :size="25" />
            <div>
              <h2>额度，以官方为准。</h2>
              <p>
                当前不显示未经确认的剩余额度百分比。请求与 Token 计数只覆盖本次 Atelier 运行，不等于
                ChatGPT 套餐总用量。
              </p>
              <a
                class="button"
                href="https://chatgpt.com/settings/usage"
                target="_blank"
                rel="noopener noreferrer"
                >进入 ChatGPT 查看用量 <Icon name="arrow" :size="15"
              /></a>
            </div></div
        ></Panel>
      </section>
      <div v-if="pending" class="gw-pending-strip inset" role="status">
        <Icon name="history" :size="18" />
        <div>
          <b>有一项登录授权正在进行</b>
          <p>关闭弹窗不会取消授权。结果仅在此页等待期间自动检查。</p>
        </div>
        <button class="button" :disabled="!!busy" @click="loginOpen = true">查看授权进度</button>
      </div>
      <div class="gw-section-heading">
        <div>
          <p class="eyebrow">YOUR SUBSCRIPTIONS</p>
          <h2>订阅账号</h2>
        </div>
        <label class="search"
          ><Icon name="search" :size="16" /><input
            v-model="query"
            aria-label="搜索订阅账号"
            placeholder="搜索名称或邮箱…"
        /></label>
      </div>
      <div v-if="!accounts.length" class="gw-empty inset">
        <span class="square-icon"><Icon name="user" :size="28" /></span>
        <h3>从你的第一个账号开始</h3>
        <p>
          通过 ChatGPT 官方页面授权，账号会出现在这里。<br />API Key
          与第三方中转请在「模型与连接」添加。
        </p>
        <button class="button primary" :disabled="!canMutate" @click="openLogin()">
          Continue with ChatGPT <Icon name="arrow" :size="16" />
        </button>
      </div>
      <div v-else-if="!filtered.length" class="gw-empty inset">
        <Icon name="search" :size="27" />
        <h3>没有匹配的账号</h3>
        <button class="button" @click="query = ''">清除搜索</button>
      </div>
      <section v-else class="gw-account-grid" aria-label="ChatGPT 订阅账号列表">
        <article
          v-for="profile in filtered"
          :key="profile.id"
          class="gw-account-card panel"
          :class="{ 'gw-current': preferred?.id === profile.id }"
        >
          <div class="gw-profile-heading">
            <span class="square-icon"><Icon name="user" :size="22" /></span>
            <div>
              <h3 :title="profile.name">{{ profile.name }}</h3>
              <p :title="profile.email ?? ''">{{ profile.email ?? '未提供邮箱' }}</p>
            </div>
            <span
              class="badge"
              :class="{
                neutral: !profile.enabled || profile.authStatus === 'unknown',
                orange: profile.enabled && profile.authStatus !== 'ready',
              }"
              >{{ label(profile) }}</span
            >
          </div>
          <div class="gw-account-selected">
            <span class="eyebrow">{{
              preferred?.id === profile.id ? 'CURRENT ROUTE / 当前账号' : 'CHATGPT SUBSCRIPTION'
            }}</span
            ><span class="tiny mono">优先级 {{ profile.priority }}</span>
          </div>
          <div class="gw-profile-usage">
            <div>
              <span>本次请求</span><b class="mono">{{ count(profile.requests) }}</b>
            </div>
            <div>
              <span>输入 Token</span><b class="mono">{{ count(profile.inputTokens) }}</b>
            </div>
            <div>
              <span>输出 Token</span><b class="mono">{{ count(profile.outputTokens) }}</b>
            </div>
          </div>
          <p
            v-if="profile.cooldownUntil && profile.cooldownUntil > Date.now()"
            class="gw-inline-note"
          >
            冷却至 {{ time(profile.cooldownUntil) }}，期间不参与自动选择。
          </p>
          <p v-if="profile.lastError" class="gw-profile-error" role="status">
            {{ profile.lastError }}
          </p>
          <div class="gw-profile-actions">
            <button
              class="button"
              :class="{ primary: preferred?.id !== profile.id }"
              :disabled="
                !canMutate ||
                !profile.enabled ||
                profile.authStatus !== 'ready' ||
                preferred?.id === profile.id
              "
              @click="choose(profile)"
            >
              {{ preferred?.id === profile.id ? '当前使用' : '设为当前' }}</button
            ><button class="button" :disabled="!canMutate || pending" @click="openLogin(profile)">
              {{ profile.authStatus === 'ready' ? '重新授权' : '登录授权' }}</button
            ><button
              class="button"
              :disabled="!canMutate || profile.authStatus !== 'ready'"
              @click="change(profile.id, () => service.refreshModels(profile.id), '可用模型已同步')"
            >
              同步模型</button
            ><button class="button" :disabled="!canMutate" @click="edit(profile)">编辑档案</button>
          </div>
          <div class="gw-card-bottom">
            <label class="gw-switch-label"
              ><span>参与账号池</span
              ><button
                class="switch"
                :class="{ on: profile.enabled }"
                role="switch"
                :aria-checked="profile.enabled"
                :aria-label="`${profile.name} 参与账号池`"
                :disabled="!canMutate"
                @click="toggle(profile)"
              >
                <span /></button
            ></label>
            <div class="gw-text-actions">
              <button
                :disabled="!canMutate || profile.authStatus !== 'ready'"
                @click="signout = profile"
              >
                退出授权</button
              ><button :disabled="!canMutate" @click="removal = profile">移除</button>
            </div>
          </div>
        </article>
      </section>
      <p class="gw-page-footnote">
        <Icon name="shield" :size="15" />此处只管理 Atelier
        自己的授权账号。自动切换只在同类账号之间进行，订阅与 API 计费不会自动互换。
      </p>
    </template>
    <Modal v-if="loginOpen" title="连接 ChatGPT / AUTHORIZE" @close="loginOpen = false">
      <p class="eyebrow">CHATGPT PLAN ACCESS</p>
      <h2>{{ loginAccountId ? '重新连接这个账号。' : '把你的账号加入工作间。' }}</h2>
      <p class="muted">
        授权将在 ChatGPT 官方页面完成。可用模型和套餐访问范围由实际授予的权限决定。
      </p>
      <label v-if="!pending && login?.state !== 'completed'" class="gw-field"
        ><span>账号名称 <small>仅在 Atelier 显示</small></span
        ><input v-model="loginName" maxlength="80" placeholder="例如：日常工作" :disabled="!!busy"
      /></label>
      <div v-if="pending" class="inset gw-login-progress">
        <span class="badge orange">等待官方授权</span>
        <p>{{ login?.message ?? '请在浏览器打开授权页面，完成后此处会自动更新。' }}</p>
        <p class="tiny muted">本次授权有效至 {{ time(login!.expiresAt) }}</p>
        <div class="gw-inline-actions">
          <button v-if="authUrl" class="button primary" @click="launchLogin">
            打开官方授权页面 <Icon name="arrow" :size="15" /></button
          ><button class="button" :disabled="!authUrl" @click="copyAuthUrl">复制登录链接</button>
        </div>
        <p v-if="!authUrl" class="error-state">未取得可识别的官方授权地址，请取消后重试。</p>
      </div>
      <div v-if="login?.state === 'completed'" class="gw-success-note inset">
        <Icon name="check" :size="20" /><span>{{
          login.message ?? '账号已连接，请先同步模型，再设为当前使用。'
        }}</span>
      </div>
      <p v-if="login && ['error', 'cancelled'].includes(login.state)" class="gw-inline-note">
        {{
          login.message ??
          (login.state === 'cancelled' ? '本次授权已取消。' : '授权未完成，请重试。')
        }}
      </p>
      <p v-if="loginError" role="alert" class="error-state">{{ loginError }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="loginOpen = false">
          {{ pending ? '后台等待' : '关闭' }}</button
        ><button v-if="pending" class="button" :disabled="!!busy" @click="cancelLogin">
          取消本次授权</button
        ><button
          v-if="pending && loginError"
          class="button primary"
          :disabled="polling || !!busy"
          @click="pollLogin"
        >
          重新检查结果</button
        ><button
          v-if="!pending && login?.state !== 'completed'"
          class="button primary"
          :disabled="!canMutate"
          @click="beginLogin"
        >
          {{ busy === 'login' ? '正在准备授权…' : 'Continue with ChatGPT' }}
        </button>
      </div>
    </Modal>
    <Modal v-if="editing" title="账号档案 / PROFILE" @close="!busy && (editing = null)"
      ><div class="gw-form-grid">
        <label class="gw-field"
          ><span>账号名称</span
          ><input v-model="editName" maxlength="80" :disabled="!!busy" /></label
        ><label class="gw-field"
          ><span>优先级</span
          ><input
            v-model.number="editPriority"
            type="number"
            min="0"
            max="1000"
            step="1"
            :disabled="!!busy"
        /></label>
      </div>
      <p class="muted">优先级参与同类账号池的选择，不改变套餐授权或费用归属。</p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="editing = null">取消</button
        ><button
          class="button primary"
          :disabled="!canMutate || !editName.trim()"
          @click="saveEdit"
        >
          保存档案
        </button>
      </div></Modal
    >
    <Modal v-if="removal" title="移除账号 / REMOVE" @close="!busy && (removal = null)"
      ><h2>移除 {{ removal.name }}？</h2>
      <p>
        请先停止网关。此操作只删除 Atelier
        本地档案和凭据，不撤销官方授权；需要撤销时请先退出授权或前往 ChatGPT 管理。
      </p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="removal = null">保留账号</button
        ><button class="button" :disabled="!canMutate" @click="confirmRemoval">确认移除</button>
      </div></Modal
    >
    <Modal v-if="signout" title="退出 Atelier 授权 / SIGN OUT" @close="!busy && (signout = null)"
      ><h2>退出 {{ signout.name }} 的授权？</h2>
      <p>此账号将无法继续为 Atelier 网关提供请求，需要重新登录授权后才能恢复。</p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="signout = null">取消</button
        ><button class="button" :disabled="!canMutate" @click="confirmSignout">退出授权</button>
      </div></Modal
    >
  </div>
</template>
