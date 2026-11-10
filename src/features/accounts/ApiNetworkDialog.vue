<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import { accountsService } from '../../services/accounts'
import type { ApiNetworkSettings, ApiNetworkView, ApiNetworkProbe } from '../../types/accounts'
const props = defineProps<{ value: ApiNetworkView }>()
const emit = defineEmits<{ close: []; saved: [value: ApiNetworkView] }>()
const mode = ref(props.value.settings.mode)
const address = ref(props.value.settings.url ?? '')
const busy = ref<'detect' | 'test' | 'save' | null>(null)
const error = ref('')
const note = ref('')
const probe = ref<ApiNetworkProbe | null>(null)
const candidates = ref<string[]>([])
let generation = 0
let alive = true
const draft = computed<ApiNetworkSettings>(() => ({
  mode: mode.value,
  url: mode.value === 'manual' ? address.value.trim() : null,
}))
const invalid = computed(() => {
  if (mode.value !== 'manual') return false
  try {
    const url = new URL(address.value.trim())
    return (
      !['http:', 'https:', 'socks5:', 'socks5h:'].includes(url.protocol) ||
      !url.hostname ||
      (!url.port && !['http:', 'https:'].includes(url.protocol)) ||
      !!url.username ||
      !!url.password ||
      !!url.search ||
      !!url.hash ||
      !['', '/'].includes(url.pathname) ||
      /[\u0000-\u001f\u007f]/.test(address.value)
    )
  } catch {
    return true
  }
})
watch(
  [mode, address],
  () => {
    generation++
    probe.value = null
    error.value = ''
    note.value = ''
  },
  { flush: 'sync' },
)
function chooseCandidate(event: Event) {
  address.value = (event.target as HTMLSelectElement).value
  mode.value = 'manual'
}
async function detect() {
  if (busy.value) return
  const token = ++generation
  busy.value = 'detect'
  error.value = ''
  probe.value = null
  try {
    const result = await accountsService.detectNetwork()
    if (!alive || token !== generation) return
    candidates.value = result.candidates
    if (result.candidates[0]) {
      mode.value = 'manual'
      address.value = result.candidates[0]
    }
    note.value = result.message
  } catch (e) {
    if (alive && token === generation) error.value = String(e)
  } finally {
    if (alive) busy.value = null
  }
}
async function test() {
  if (busy.value || invalid.value) return
  const token = ++generation
  busy.value = 'test'
  error.value = ''
  probe.value = null
  try {
    const result = await accountsService.testNetwork({ ...draft.value })
    if (alive && token === generation) probe.value = result
  } catch (e) {
    if (alive && token === generation) error.value = String(e)
  } finally {
    if (alive) busy.value = null
  }
}
async function save() {
  if (busy.value || invalid.value) return
  const token = ++generation
  busy.value = 'save'
  error.value = ''
  try {
    const result = await accountsService.saveNetwork({ ...draft.value }, props.value.revision)
    if (alive && token === generation) emit('saved', result)
  } catch (e) {
    if (alive && token === generation) error.value = String(e)
  } finally {
    if (alive) busy.value = null
  }
}
onBeforeUnmount(() => {
  alive = false
  generation++
})
</script>
<template>
  <Modal title="订阅 API 网络配置" @close="emit('close')">
    <div class="account-network-dialog">
      <p class="eyebrow">SUBSCRIPTION API / NETWORK</p>
      <p class="muted">
        仅配置 Atelier 查询订阅元数据的网络代理，不改变 Codex
        额度接口、聊天连接、本地网关或系统代理。
      </p>
      <label class="account-network-field"
        >代理模式
        <select v-model="mode" aria-label="订阅 API 代理模式" :disabled="!!busy">
          <option value="system">跟随系统</option>
          <option value="direct">直连（不使用客户端代理）</option>
          <option value="manual">手动代理</option>
        </select>
      </label>
      <p v-if="mode === 'system'" class="tiny muted">
        读取已有系统/环境代理；不下载或执行 PAC，只有 PAC 时会提示手动配置。
      </p>
      <label v-if="mode === 'manual'" class="account-network-field"
        >代理地址
        <input
          v-model="address"
          aria-label="订阅 API 代理地址"
          placeholder="http://127.0.0.1:7890"
          autocomplete="off"
          spellcheck="false"
          :disabled="busy === 'save'"
        />
      </label>
      <p v-if="mode === 'manual' && invalid" class="maintenance-warning">
        请输入 HTTP / HTTPS / SOCKS5 / SOCKS5H 主机和端口；不接受账号密码、路径或查询。
      </p>
      <div v-if="candidates.length > 1" class="account-network-field">
        <label for="network-candidate">检测到的候选</label>
        <select
          id="network-candidate"
          aria-label="代理候选"
          :value="address"
          :disabled="!!busy"
          @change="chooseCandidate"
        >
          <option v-for="candidate in candidates" :key="candidate" :value="candidate">
            {{ candidate }}
          </option>
        </select>
      </div>
      <p v-if="note" role="status" class="muted">{{ note }}</p>
      <div
        v-if="probe"
        role="status"
        class="inset account-network-result"
        :data-result="probe.reachable ? 'reachable' : 'failed'"
      >
        <b>{{ probe.reachable ? '网络收到响应' : '网络测试未通过' }}</b>
        <span>{{ probe.message }}</span
        ><small
          >{{ probe.elapsedMs }} ms{{
            probe.statusCode ? ` · HTTP ${probe.statusCode}` : ''
          }}</small
        >
      </div>
      <p class="tiny muted">
        测试只向固定官方域名发送无令牌请求；收到 401/403 也只说明收到 HTTP
        响应，不代表账号或订阅有效。检测和测试均不会保存设置。
      </p>
      <p v-if="error" role="alert" class="error-state">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="busy === 'save'" @click="emit('close')">取消</button>
        <button class="button" :disabled="!!busy" @click="detect">
          <Icon name="search" :size="14" />{{ busy === 'detect' ? '检测中…' : '一键检测' }}
        </button>
        <button class="button" :disabled="!!busy || invalid" @click="test">
          {{ busy === 'test' ? '测试中…' : '测试' }}
        </button>
        <button class="button primary" :disabled="!!busy || invalid" @click="save">
          {{ busy === 'save' ? '保存中…' : '保存' }}
        </button>
      </div>
    </div>
  </Modal>
</template>
