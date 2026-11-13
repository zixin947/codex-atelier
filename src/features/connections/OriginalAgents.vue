<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { theme } from '../../app/theme'

/* 本组件只负责「进入即加载」：挂载后直接读取本机管理服务地址，读不到才启动它。
   没有「打开」按钮，也没有自动重试——失败一次就如实报告，由用户点「重试」再来，
   避免重复启动引擎或在卸载后继续写状态。

   副作用边界（已核对 src-tauri/src/gateway/mod.rs `gateway_start`）：
   `originalUi: true` 只启动原版管理界面所需的本机引擎，不写 ~/.codex/config.toml，
   不注入供应商与路由组，不启动桌面接入，也不改变 Codex 现有请求走向。
   把 Codex 指向本地网关需要用户另行在「模型与连接」里显式启动接入，且只对新进程生效。 */

const emit = defineEmits<{ changed: [] }>()
const props = withDefaults(
  defineProps<{
    running?: boolean
    view?: string
    title?: string
    usageTab?: 'usage' | 'requests' | 'sessions' | 'context'
  }>(),
  {
    running: false,
    view: 'agents',
    title: 'Agent 接入与模型选择',
  },
)
const url = ref(''),
  error = ref(''),
  preparing = ref(false)
let alive = true
let starting = false

async function loadAddress() {
  const address = new URL(await invoke<string>('gateway_original_ui'))
  address.searchParams.set('view', props.view)
  if (['providers', 'routing', 'groups', 'gateway', 'usage'].includes(props.view))
    address.searchParams.set('atelierContent', '1')
  if (props.view === 'usage') {
    address.searchParams.set('atelierUsage', 'chatgpt')
    address.searchParams.set('tab', props.usageTab || 'usage')
  }
  if (theme.value !== 'millennium') address.searchParams.set('atelierStyle', 'endfield')
  if (!alive) return
  url.value = address.toString()
}

/* 单次尝试：先读地址，读不到才启动管理服务。并发进入只会启动一次。 */
async function ensureManagement() {
  if (starting) return
  starting = true
  preparing.value = true
  error.value = ''
  try {
    try {
      await loadAddress()
    } catch {
      if (!alive) return
      await invoke('gateway_start', { originalUi: true })
      if (!alive) return
      emit('changed')
      await loadAddress()
    }
  } catch (e) {
    if (alive) error.value = String(e)
  } finally {
    starting = false
    if (alive) preparing.value = false
  }
}

watch(
  () => [props.view, props.usageTab] as const,
  () => {
    if (!isTauri()) return
    url.value = ''
    void ensureManagement()
  },
  { immediate: true },
)
watch(
  () => props.running,
  (running) => {
    if (!isTauri() || !running || url.value) return
    void ensureManagement()
  },
)
onMounted(() => {
  if (!isTauri()) error.value = '打开 Atelier 桌面应用才能查看本页；浏览器预览不连接本机管理服务。'
})
onUnmounted(() => {
  alive = false
})
</script>

<template>
  <section class="original-agents">
    <h2>{{ title }}</h2>
    <p v-if="view === 'usage'" class="usage-scope">
      仅统计 ChatGPT 账号记录；无法确认账号来源的历史不会按模型名称推断。
    </p>
    <p v-if="preparing" class="original-status" role="status">正在读取本机管理服务…</p>
    <p v-else-if="error" class="original-error" role="alert">
      {{ error }}
      <button v-if="isTauri()" type="button" class="button" @click="ensureManagement">重试</button>
    </p>
    <iframe v-if="url" :src="url" :title="title" referrerpolicy="no-referrer" />
  </section>
</template>

<style scoped>
.original-agents {
  margin-bottom: 24px;
}
.original-agents h2 {
  font-size: 20px;
  margin-bottom: 12px;
}
.original-agents p {
  padding: 18px;
  border: 1px solid var(--border);
}
.original-agents .usage-scope {
  padding: 0 0 12px;
  border: 0;
  color: var(--muted);
  font-size: 13px;
}
.original-agents .original-error {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 12px;
  color: var(--danger, #b3261e);
}
.original-agents iframe {
  width: 100%;
  height: 900px;
  border: 1px solid var(--border);
  border-radius: 12px;
  background: white;
}
</style>
