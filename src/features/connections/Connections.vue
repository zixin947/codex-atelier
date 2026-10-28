<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import Panel from '../../shared/components/Panel.vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import { useAtelier } from '../../app/store'
import { gatewayService } from '../../services/gateway'
import type {
  GatewayService,
  GatewaySnapshot,
  GatewayProfile,
  GatewaySettings,
} from '../../types/gateway'
import '../../styles/gateway.css'
const props = defineProps<{ service?: GatewayService }>()
const service = props.service ?? gatewayService
const store = useAtelier()
const data = ref<GatewaySnapshot | null>(null),
  busy = ref(''),
  error = ref(''),
  query = ref('')
const formOpen = ref(false),
  editingId = ref<string | null>(null),
  formRevision = ref('')
const name = ref(''),
  baseUrl = ref('https://api.openai.com/v1'),
  apiKey = ref(''),
  modelText = ref(''),
  priority = ref(100)
const removing = ref<GatewayProfile | null>(null),
  launch = ref(false),
  preview = ref<string | null>(null),
  localKey = ref('')
const draft = ref<GatewaySettings>({
  port: 18910,
  autoSwitch: false,
  maxAttempts: 2,
  preferredProfileId: null,
  model: '',
})
const settingsDirty = ref(false),
  settingsRevision = ref('')
let alive = true,
  timer: ReturnType<typeof setTimeout> | undefined
const providers = computed(() => data.value?.profiles.filter((p) => p.kind === 'apiKey') ?? [])
const filtered = computed(() =>
  providers.value.filter((p) =>
    `${p.name} ${p.baseUrl}`.toLowerCase().includes(query.value.trim().toLowerCase()),
  ),
)
const selected = computed(() =>
  data.value?.profiles.find((p) => p.id === data.value?.settings.preferredProfileId),
)
const selectedDraft = computed(() =>
  data.value?.profiles.find((p) => p.id === draft.value.preferredProfileId),
)
const canWrite = computed(() => data.value?.available === true && !busy.value)
const running = computed(() => data.value?.runtime.state === 'running')
const runtimeLabel = computed(
  () =>
    ({
      running: '网关运行中',
      stopped: '网关已停止',
      stopping: '正在等待在途请求',
      error: '网关连接异常',
    })[data.value?.runtime.state ?? 'stopped'],
)
function apply(value: GatewaySnapshot) {
  if (data.value?.runtime.state !== value.runtime.state) localKey.value = ''
  data.value = value
  if (!settingsDirty.value) {
    draft.value = { ...value.settings }
    settingsRevision.value = value.revision
  }
}
function schedule() {
  clearTimeout(timer)
  if (alive && ['running', 'stopping'].includes(data.value?.runtime.state ?? ''))
    timer = setTimeout(() => {
      void refresh(false)
    }, 2500)
}
async function refresh(visible = true) {
  if (busy.value) {
    schedule()
    return
  }
  if (visible) {
    busy.value = 'refresh'
    error.value = ''
  }
  try {
    const result = await service.snapshot()
    if (alive) apply(result)
  } catch (e) {
    if (alive) error.value = String(e)
  } finally {
    if (visible) busy.value = ''
    schedule()
  }
}
async function mutate(key: string, task: () => Promise<GatewaySnapshot>, message: string) {
  if (!canWrite.value) return
  busy.value = key
  error.value = ''
  try {
    apply(await task())
    store.notify(message)
  } catch (e) {
    error.value = String(e)
    try {
      apply(await service.snapshot())
    } catch {
      /* Keep last confirmed state. */
    }
  } finally {
    busy.value = ''
    schedule()
  }
}
function beginEdit(p?: GatewayProfile) {
  editingId.value = p?.id ?? null
  name.value = p?.name ?? ''
  baseUrl.value = p?.baseUrl ?? 'https://api.openai.com/v1'
  apiKey.value = ''
  modelText.value = p?.models.map((m) => m.id).join('\n') ?? ''
  priority.value = p?.priority ?? 100
  formRevision.value = data.value?.revision ?? ''
  formOpen.value = true
  error.value = ''
}
function closeForm() {
  if (!busy.value) {
    formOpen.value = false
    apiKey.value = ''
  }
}
async function saveProvider() {
  if (!canWrite.value) return
  const input = {
    id: editingId.value,
    name: name.value.trim(),
    baseUrl: baseUrl.value.trim(),
    apiKey: apiKey.value || null,
    models: modelText.value
      .split(/[\n,]/)
      .map((s) => s.trim())
      .filter(Boolean),
    priority: priority.value,
  }
  await mutate(
    'save-provider',
    async () => {
      const result = await service.saveProvider(input, formRevision.value)
      apiKey.value = ''
      formOpen.value = false
      return result
    },
    'Provider 已保存；保存不代表连通性已经验证',
  )
}
function choose(p: GatewayProfile) {
  void mutate(
    p.id,
    () => service.selectProfile(p.id, data.value!.revision),
    `当前账号已设为 ${p.name}`,
  )
}
function toggle(p: GatewayProfile) {
  void mutate(
    p.id,
    () =>
      service.updateProfile(
        { id: p.id, name: p.name, enabled: !p.enabled, priority: p.priority },
        data.value!.revision,
      ),
    '账号池状态已更新',
  )
}
function toggleAutoSwitch() {
  editSettings()
  draft.value.autoSwitch = !draft.value.autoSwitch
}
function editSettings() {
  if (!settingsDirty.value) settingsRevision.value = data.value?.revision ?? ''
  settingsDirty.value = true
}
async function saveSettings() {
  const value = { ...draft.value }
  await mutate(
    'settings',
    async () => {
      const result = await service.saveSettings(value, settingsRevision.value)
      settingsDirty.value = false
      return result
    },
    '路由设置已保存，作用于后续请求',
  )
}
function resetSettings() {
  settingsDirty.value = false
  if (data.value) apply(data.value)
}
async function showPreview() {
  busy.value = 'preview'
  error.value = ''
  try {
    preview.value = await service.preview()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function showKey() {
  busy.value = 'key'
  error.value = ''
  try {
    localKey.value = await service.localKey()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    store.notify('已复制')
  } catch {
    error.value = '复制失败，可选中文本手动复制。'
  }
}
async function launchDesktop() {
  busy.value = 'launch'
  error.value = ''
  try {
    store.notify(await service.launchDesktop())
    launch.value = false
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function remove() {
  if (!removing.value) return
  const id = removing.value.id
  await mutate(
    id,
    async () => {
      const value = await service.removeProfile(id, data.value!.revision)
      removing.value = null
      return value
    },
    'Provider 已移除',
  )
}
function profileName(id: string | null) {
  return data.value?.profiles.find((p) => p.id === id)?.name ?? '未选账号'
}
function status(p: GatewayProfile) {
  if (!p.enabled) return '已停用'
  if (p.cooldownUntil && p.cooldownUntil > Date.now()) return '冷却中'
  return {
    ready: '目录已同步',
    unknown: '已配置 · 未验证',
    needsLogin: '凭据需更新',
    planDisabled: '套餐未授权',
  }[p.authStatus]
}
function time(at: number) {
  return new Date(at).toLocaleTimeString('zh-CN', { hour12: false })
}
onMounted(() => {
  void refresh()
})
onUnmounted(() => {
  alive = false
  clearTimeout(timer)
  apiKey.value = ''
  localKey.value = ''
})
</script>
<template>
  <div class="page gateway-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / MODELS & CONNECTIONS</p>
        <h1>让每次请求，各有所往<span class="heading-dot">.</span></h1>
        <p class="subtitle">
          API Provider、本地中转与自动切换，在同一个控制台。<RouterLink to="/accounts/official"
            >管理网关官方授权</RouterLink
          >
        </p>
      </div>
      <button class="button" :disabled="!!busy" @click="refresh()">
        <Icon name="refresh" :size="16" />刷新状态
      </button>
    </div>
    <div v-if="error" role="alert" class="error-state">{{ error }}</div>
    <div v-if="data && !data.available" class="gw-native-note inset" role="status">
      <Icon name="terminal" />{{ data.reason }}
    </div>
    <div v-if="!data" class="empty" role="status">正在读取 Atelier 连接设置…</div>
    <template v-else>
      <section class="gw-connection-overview" aria-label="本地网关控制台">
        <div class="gw-console">
          <div class="gw-console-head">
            <span>LOCAL GATEWAY / RESPONSES</span
            ><span class="gw-console-status"
              ><i class="dot" :class="{ 'gw-dot-off': !running }" />{{
                running ? 'ONLINE' : 'OFFLINE'
              }}</span
            >
          </div>
          <div class="gw-gateway-body">
            <p class="gw-screen-label">
              {{ data.available ? '本机回环 · 手动启停' : '桌面连接尚不可用' }}
            </p>
            <h2>{{ runtimeLabel }}</h2>
            <code>{{ data.runtime.address ?? `127.0.0.1:${data.settings.port} · 尚未监听` }}</code>
            <p>
              当前账号 <b>{{ selected?.name ?? '尚未选择' }}</b
              ><br />当前模型 <b>{{ data.settings.model || '尚未选择' }}</b>
            </p>
            <div class="gw-inline-actions">
              <button
                v-if="!running"
                class="button primary"
                :disabled="!canWrite || data.runtime.state === 'stopping' || settingsDirty"
                @click="mutate('start', () => service.start(), '本地网关已启动')"
              >
                <Icon name="plug" :size="16" />启动网关</button
              ><button
                v-else
                class="button"
                :disabled="!canWrite"
                @click="mutate('stop', () => service.stop(), '网关正在停止接收新请求')"
              >
                停止网关</button
              ><button
                class="button"
                :disabled="!canWrite || !running || settingsDirty"
                @click="launch = true"
              >
                通过网关启动 Desktop <Icon name="arrow" :size="16" />
              </button>
            </div>
            <p v-if="data.runtime.message" class="gw-runtime-message" role="status">
              {{ data.runtime.message }}
            </p>
          </div>
          <div class="gw-console-meters">
            <div>
              <span>在途请求</span><b>{{ data.runtime.activeRequests }}</b>
            </div>
            <div>
              <span>已完成</span><b>{{ data.runtime.completedRequests }}</b>
            </div>
            <div>
              <span>失败 / 未完成</span><b>{{ data.runtime.failedRequests }}</b>
            </div>
          </div>
        </div>
        <Panel title="自动切换" code="ROUTING POLICY"
          ><div class="gw-policy-body">
            <label class="gw-field"
              ><span>当前账号 / 首选路由</span
              ><select
                v-model="draft.preferredProfileId"
                :disabled="!canWrite"
                @change="editSettings"
              >
                <option :value="null">请选择账号</option>
                <option v-for="p in data.profiles" :key="p.id" :value="p.id">
                  {{ p.name }} · {{ p.kind === 'chatgpt' ? '订阅' : 'API'
                  }}{{ p.enabled ? '' : '（停用）' }}
                </option>
              </select></label
            >
            <label class="gw-field"
              ><span>请求模型</span
              ><input
                v-model="draft.model"
                list="gateway-models"
                placeholder="选择或填写该账号支持的模型 ID"
                :disabled="!canWrite"
                @input="editSettings"
              /><datalist id="gateway-models">
                <option v-for="m in selectedDraft?.models ?? []" :key="m.id" :value="m.id">
                  {{ m.name }}
                </option>
              </datalist></label
            >
            <div class="gw-form-grid">
              <label class="gw-field"
                ><span>本地端口</span
                ><input
                  v-model.number="draft.port"
                  type="number"
                  min="1024"
                  max="65535"
                  :disabled="!canWrite || running"
                  @input="editSettings" /></label
              ><label class="gw-field"
                ><span>最多尝试账号数</span
                ><input
                  v-model.number="draft.maxAttempts"
                  type="number"
                  min="1"
                  max="5"
                  :disabled="!canWrite"
                  @input="editSettings"
              /></label>
            </div>
            <div class="setting-row">
              <span><b>自动选择备用账号</b><small>同类账号池 · 数字较小者优先</small></span
              ><button
                role="switch"
                class="switch"
                :class="{ on: draft.autoSwitch }"
                :aria-checked="draft.autoSwitch"
                aria-label="自动切换账号"
                :disabled="!canWrite"
                @click="toggleAutoSwitch"
              >
                <span />
              </button>
            </div>
            <p class="gw-inline-note">
              只在明确拒绝且尚未开始输出时尝试备用账号。不会跨订阅/API计费，不会重发执行状态不明的请求。
            </p>
            <div class="gw-inline-actions">
              <button
                class="button primary"
                :disabled="!canWrite || !settingsDirty"
                @click="saveSettings"
              >
                保存路由设置</button
              ><button class="button" :disabled="!settingsDirty || !!busy" @click="resetSettings">
                放弃修改
              </button>
            </div>
          </div></Panel
        >
      </section>
      <div class="gw-section-heading">
        <div>
          <p class="eyebrow">UPSTREAM CONNECTIONS</p>
          <h2>API Key 与第三方中转</h2>
        </div>
        <div class="gw-heading-actions">
          <label class="search"
            ><Icon name="search" :size="16" /><input
              v-model="query"
              placeholder="搜索 Provider…"
              aria-label="搜索 Provider" /></label
          ><button class="button primary" :disabled="!canWrite" @click="beginEdit()">
            添加 Provider
          </button>
        </div>
      </div>
      <div v-if="!providers.length" class="gw-empty inset">
        <span class="square-icon"><Icon name="chip" :size="28" /></span>
        <h3>添加第一个上游连接</h3>
        <p>支持 OpenAI Responses 协议的 API 服务。订阅账号在「账号与额度」中授权。</p>
        <button class="button" :disabled="!canWrite" @click="beginEdit()">
          配置 API Key 与地址
        </button>
      </div>
      <div v-else-if="!filtered.length" class="gw-empty inset">没有匹配的 Provider。</div>
      <div v-else class="gw-provider-list">
        <article
          v-for="p in filtered"
          :key="p.id"
          class="gw-provider-row panel"
          :class="{ 'gw-current': selected?.id === p.id }"
        >
          <span class="square-icon"><Icon name="chip" :size="23" /></span>
          <div class="gw-provider-description">
            <h3>
              {{ p.name }}
              <span
                class="badge"
                :class="{
                  neutral: p.authStatus === 'unknown' || !p.enabled,
                  orange: p.authStatus === 'needsLogin',
                }"
                >{{ status(p) }}</span
              >
            </h3>
            <code :title="p.baseUrl">{{ p.baseUrl }}</code>
            <p class="tiny muted">
              {{ p.models.length }} 个模型 · 优先级 {{ p.priority }} · 本次 {{ p.requests }} 个请求
            </p>
            <p v-if="p.lastError" class="gw-profile-error">{{ p.lastError }}</p>
          </div>
          <div class="gw-provider-controls">
            <button
              class="button"
              :disabled="
                !canWrite || !p.enabled || p.authStatus === 'needsLogin' || selected?.id === p.id
              "
              @click="choose(p)"
            >
              {{ selected?.id === p.id ? '当前使用' : '设为当前' }}</button
            ><button
              class="button"
              :disabled="!canWrite"
              @click="
                mutate(
                  p.id,
                  () => service.refreshModels(p.id),
                  '模型目录已同步；不代表所有模型均可推理',
                )
              "
            >
              同步模型</button
            ><button class="button" :disabled="!canWrite" @click="beginEdit(p)">编辑</button
            ><button
              class="switch"
              :class="{ on: p.enabled }"
              role="switch"
              :aria-checked="p.enabled"
              :aria-label="`${p.name} 参与账号池`"
              :disabled="!canWrite"
              @click="toggle(p)"
            >
              <span /></button
            ><button
              class="icon-button"
              :disabled="!canWrite"
              :aria-label="`移除 ${p.name}`"
              @click="removing = p"
            >
              <Icon name="close" :size="16" />
            </button>
          </div>
        </article>
      </div>
      <section class="gw-bottom-grid">
        <Panel title="接入你的客户端" code="CLIENT ACCESS"
          ><div class="gw-access">
            <p>
              网关支持 Responses HTTP/SSE。客户端工具仍由 Codex 执行；本版不提供 WebSocket 或 Chat
              Completions 协议转换。
            </p>
            <div class="gw-inline-actions">
              <button class="button" :disabled="!canWrite" @click="showPreview">查看配置预览</button
              ><button class="button" :disabled="!canWrite || !running" @click="showKey">
                显示本地网关密钥
              </button>
            </div>
            <div v-if="localKey" class="gw-key-row">
              <input :value="localKey" readonly aria-label="本地网关访问密钥" /><button
                class="button"
                @click="copy(localKey)"
              >
                复制</button
              ><button class="icon-button" aria-label="隐藏网关密钥" @click="localKey = ''">
                <Icon name="close" />
              </button>
            </div>
            <p class="tiny muted">
              密钥仅用于本机客户端接入，每次启动网关重新生成。停止并重启网关后，已接入客户端需要更新密钥。
            </p>
          </div></Panel
        ><Panel title="最近路由记录" code="THIS RUN"
          ><div class="gw-route-log">
            <p v-if="!data.events.length" class="muted">
              暂无请求。这里仅记录状态与账号选择，不保存对话正文。
            </p>
            <div v-for="(e, i) in data.events.slice(0, 6)" :key="`${e.at}-${i}`" class="gw-log-row">
              <time class="mono">{{ time(e.at) }}</time>
              <div>
                <b>{{ profileName(e.profileId) }}</b
                ><code>{{ e.code }}</code>
              </div>
              <span class="badge" :class="{ orange: e.status !== 'completed' }"
                >{{ e.status === 'completed' ? '完成' : '未完成' }} · {{ e.attempts }} 次</span
              >
            </div>
          </div></Panel
        >
      </section>
      <p class="gw-page-footnote">
        <Icon name="shield" :size="15" />网关默认关闭，只监听本机。凭据保存在 Atelier
        私有目录，不扫描或覆盖 Codex 的账号文件。
      </p>
    </template>
    <Modal
      v-if="formOpen"
      :title="editingId ? '编辑 Provider / CONNECTION' : '添加 Provider / CONNECTION'"
      @close="closeForm"
      ><div class="gw-form-grid">
        <label class="gw-field"
          ><span>连接名称</span
          ><input v-model="name" maxlength="80" placeholder="例如：工作 API" /></label
        ><label class="gw-field"
          ><span>优先级（越小越优先）</span
          ><input v-model.number="priority" type="number" min="0" max="1000"
        /></label>
      </div>
      <label class="gw-field"
        ><span>Base URL · OpenAI Responses 兼容</span
        ><input
          v-model="baseUrl"
          placeholder="https://api.example.com/v1"
          autocomplete="off" /></label
      ><label class="gw-field"
        ><span
          >API Key <small>{{ editingId ? '留空保留原密钥' : '仅提交到本机后端' }}</small></span
        ><input
          v-model="apiKey"
          type="password"
          autocomplete="new-password"
          spellcheck="false" /></label
      ><label class="gw-field"
        ><span>模型 ID <small>每行一个，或保存后手动同步模型</small></span
        ><textarea
          v-model="modelText"
          rows="4"
          placeholder="填写此服务提供的实际模型 ID"
          spellcheck="false"
        />
      </label>
      <p class="muted">
        凭据写入 Atelier 自己的受用户权限保护的本地文件。保存不会发送测试推理请求。
      </p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="closeForm">取消</button
        ><button
          class="button primary"
          :disabled="!canWrite || !name.trim() || !baseUrl.trim() || (!editingId && !apiKey)"
          @click="saveProvider"
        >
          保存 Provider
        </button>
      </div></Modal
    >
    <Modal v-if="removing" title="移除 Provider" @close="!busy && (removing = null)"
      ><h2>移除 {{ removing.name }}？</h2>
      <p>将删除 Atelier 保存的此连接与 API Key。请先停止网关并等待在途请求完成。</p>
      <p v-if="error" role="alert" class="error-state">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="removing = null">取消</button
        ><button class="button" :disabled="!canWrite" @click="remove">确认移除</button>
      </div></Modal
    >
    <Modal v-if="preview !== null" title="接入配置 / PREVIEW" @close="preview = null"
      ><p>此处只展示配置。可由你复制应用，也可使用主面板的 Desktop 启动入口，按进程参数接入。</p>
      <pre class="gw-code-preview" tabindex="0">{{ preview }}</pre>
      <div class="modal-actions">
        <button class="button" @click="preview = null">关闭</button
        ><button class="button primary" @click="copy(preview)">复制配置</button>
      </div></Modal
    >
    <Modal v-if="launch" title="通过本地网关启动 Desktop" @close="!busy && (launch = false)"
      ><h2>由你接入，保持原有聊天。</h2>
      <p>
        请先自行退出 Desktop。此操作会通过 Atelier 桥接重新启动客户端，并只为新进程设置
        Provider、模型与本地网关密钥，不写入你的 Codex 配置文件。
      </p>
      <p>
        请保持 Atelier 和网关运行。当前首选：{{ selected?.name ?? '未选择' }}；模型：{{
          data?.settings.model || '未选择'
        }}。订阅通道的模型和工具仍受官方授权限制。
      </p>
      <p class="muted">
        真实客户端和旧聊天的兼容性待你验收。恢复普通接入时，退出 Desktop，再正常打开即可。
      </p>
      <p v-if="error" class="error-state" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="launch = false">暂不接入</button
        ><button class="button primary" :disabled="!canWrite || !running" @click="launchDesktop">
          已退出 Desktop，启动接入
        </button>
      </div></Modal
    >
  </div>
</template>
