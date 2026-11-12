<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import { theme } from '../../app/theme'
const emit = defineEmits<{ changed: [] }>()
const busy = ref(false)
async function open() {
  busy.value = true
  error.value = ''
  try {
    await invoke('gateway_start', { originalUi: true })
    emit('changed')
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
const props = withDefaults(
  defineProps<{
    running: boolean
    view?: string
    title?: string
    usageTab?: 'usage' | 'requests' | 'sessions' | 'context'
  }>(),
  {
    view: 'agents',
    title: 'Agent 接入与模型选择',
  },
)
const url = ref(''),
  error = ref('')
watch(
  () => [props.running, props.view, props.usageTab] as const,
  async ([running]) => {
    url.value = ''
    error.value = ''
    if (!running || !isTauri()) return
    try {
      const address = new URL(await invoke<string>('gateway_original_ui'))
      address.searchParams.set('view', props.view)
      if (['providers', 'routing', 'groups', 'gateway', 'usage'].includes(props.view))
        address.searchParams.set('atelierContent', '1')
      if (props.view === 'usage') {
        address.searchParams.set('atelierUsage', 'chatgpt')
        address.searchParams.set('tab', props.usageTab || 'usage')
      }
      if (theme.value !== 'millennium') address.searchParams.set('atelierStyle', 'endfield')
      url.value = address.toString()
    } catch (e) {
      error.value = String(e)
    }
  },
  { immediate: true },
)
</script>

<template>
  <section class="original-agents">
    <h2>{{ title }}</h2>
    <p v-if="!running">
      {{
        view === 'usage'
          ? '连接本机引擎后查看 ChatGPT 用量、请求、会话与上下文。'
          : '打开页面时启动本机网关，并读取本机已登录的供应商账号。'
      }}
    </p>
    <p v-if="view === 'usage'" class="usage-scope">
      仅统计 ChatGPT 账号记录；无法确认账号来源的历史不会按模型名称推断。
    </p>
    <button v-if="!running" class="button" :disabled="busy || !isTauri()" @click="open">
      {{ busy ? '正在打开…' : `打开${title}` }}
    </button>
    <p v-if="error" role="alert">{{ error }}</p>
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
.original-agents iframe {
  width: 100%;
  height: 900px;
  border: 1px solid var(--border);
  border-radius: 12px;
  background: white;
}
</style>
