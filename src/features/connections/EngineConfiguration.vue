<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke, isTauri } from '@tauri-apps/api/core'
import Modal from '../../shared/components/Modal.vue'
const emit = defineEmits<{ close: [] }>()
const text = ref(''),
  revision = ref(''),
  error = ref(''),
  busy = ref(false)
interface Document {
  revision: string
  configuration: Record<string, unknown>
}
async function load() {
  if (!isTauri()) {
    error.value = '请在 Atelier 桌面应用中配置路由引擎。'
    return
  }
  busy.value = true
  try {
    const d = await invoke<Document>('gateway_engine_configuration')
    revision.value = d.revision
    text.value = JSON.stringify(d.configuration, null, 2)
    error.value = ''
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
async function save() {
  busy.value = true
  try {
    const d = await invoke<Document>('gateway_engine_configuration_save', {
      configuration: JSON.parse(text.value),
      expectedRevision: revision.value,
    })
    revision.value = d.revision
    text.value = JSON.stringify(d.configuration, null, 2)
    emit('close')
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = false
  }
}
onMounted(load)
</script>
<template>
  <Modal title="协议与路由策略 / ROUTING CORE" :close-disabled="busy" @close="emit('close')">
    <h2>完整协议与路由策略</h2>
    <p>内置 Atelier 协议与路由引擎。停止网关后修改，重新启动生效。</p>
    <p class="muted">
      支持 Chat Completions、Responses /
      WebSocket、Anthropic、Gemini、Ollama、图像、音视频、嵌入、重排、Decisions 与原生透传。
    </p>
    <p class="muted">
      策略：smart、order、rotate、usage、pace、weight、free、fast、cheap、drain、expiring、manual。
    </p>
    <details>
      <summary>配置字段与示例</summary>
      <p>
        <code>providerOptions</code> 按已保存连接 ID 补充协议地址与策略；<code>providers</code>
        添加完整供应商定义；<code>groups</code> 设置路由组；<code>settings</code> 使用原生策略设置。
      </p>
      <pre>
{"providerOptions":{"连接 ID":{"chat":"https://example.com/v1","anthropic":"https://example.com","gemini":"https://example.com/v1beta","routing":"rotate"}},"groups":[{"id":"work","name":"工作路由","routing":"cheap","members":["连接 ID/模型 ID"]}]}</pre>
      <p>
        路由组以 <code>group/work</code> 作为模型 ID 调用。已有连接的主密钥由 Atelier
        注入，额外填写的密钥保存在本机私密配置中。
      </p>
      <p>
        <code>discoverAccounts: true</code>
        会允许核心读取本机已登录的订阅账号；默认关闭。启动时不运行原桌面程序的 Agent
        接管与配置同步链路。
      </p>
    </details>
    <div v-if="error" role="alert" class="error-state">{{ error }}</div>
    <label class="engine-editor"
      >原生配置 JSON<textarea
        v-model="text"
        :disabled="busy"
        aria-label="协议与策略配置 JSON"
        spellcheck="false"
      />
    </label>
    <div class="modal-actions">
      <button class="button" :disabled="busy" @click="load">重新读取</button
      ><button class="button primary" :disabled="busy || !revision" @click="save">保存配置</button>
    </div>
  </Modal>
</template>
<style scoped>
.engine-editor {
  display: grid;
  gap: 8px;
  margin: 18px 0;
}
textarea {
  width: 100%;
  min-height: 280px;
  resize: vertical;
  background: var(--paper);
  color: var(--ink);
  border: 1px solid var(--line);
  padding: 12px;
  font: 12px/1.6 var(--font-mono);
}
pre {
  white-space: pre-wrap;
  overflow-wrap: anywhere;
}
</style>
