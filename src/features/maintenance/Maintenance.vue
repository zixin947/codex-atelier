<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import Icon from '../../shared/components/Icon.vue'
import Modal from '../../shared/components/Modal.vue'
import DiagnosticDialog from './DiagnosticDialog.vue'
import { maintenanceService } from '../../services/maintenance'
import type {
  DiagnosticBundle,
  DiagnosticReport,
  MaintenanceAction,
  MaintenanceResult,
  MaintenanceSnapshot,
  MaintenanceUtilitySettings,
} from '../../types/maintenance'
import './maintenance.css'

/* 排版对齐 AiMaMi 1.2.7 维护页：单张卡片 + 分隔行，行内为图标、标题、描述与右侧动作或开关。
   文案取自二进制常量区；仅把发送目标里的产品名换成 Atelier，因为接收地址由用户填写。
   本版按使用频率重排：诊断与生图声明常驻，诊断包与高级维护各自折叠。 */

type Tool = MaintenanceAction | 'export' | 'send' | 'image'

interface Row {
  key: Tool
  icon: string
  title: string
  description: string
  action: string
  busy?: string
  confirm?: string
  confirmText?: string
  confirmAccept?: string
}

const diagnoseRow: Row = {
  key: 'diagnose',
  icon: 'pulse',
  title: '系统诊断',
  description:
    '一站式排查并修复 Codex 运行问题：第三方污染、配置/认证/线程异常、旧库残留等，发现异常可一键修复并自动重启 Codex。',
  action: '运行诊断',
  busy: '诊断中...',
}

const packageRows: Row[] = [
  {
    key: 'export',
    icon: 'download',
    title: '调试日志',
    description:
      '导出脱敏诊断包，包含路由状态、线程一致性、系统诊断、进程占用和崩溃日志，方便定位异常原因。',
    action: '导出日志',
    busy: '导出中...',
  },
  {
    key: 'send',
    icon: 'upload',
    title: '发送诊断信息',
    description: '生成脱敏诊断包并安全发送至指定接收地址，用于定位特殊设备和兼容性问题。',
    action: '发送诊断',
    busy: '发送中...',
  },
]

const advancedRows: Row[] = [
  {
    key: 'clean',
    icon: 'trash',
    title: '清理数据',
    description: '删除多余的认证备份、注册表备份和失效的快照条目，释放磁盘空间。',
    action: '开始清理',
    busy: '清理中...',
  },
  {
    key: 'rebuild',
    icon: 'gem',
    title: '重建注册表',
    description: '扫描本地快照文件重新生成注册表，修复注册表损坏或数据不一致。',
    action: '开始重建',
    busy: '重建中...',
  },
  {
    key: 'forceKill',
    icon: 'close',
    title: '关闭 Codex',
    description:
      '只请求 Codex 正常退出并等待。超时未退出即中止并报告仍存在的进程，不会结束任何进程，也不会自动重启客户端。',
    action: '关闭 Codex',
    busy: '关闭中...',
    confirm: '确定关闭 Codex？',
    confirmText:
      '将请求 Codex 正常退出并等待；超时未退出会中止并报告。不会强制结束进程，也不会自动重启。',
    confirmAccept: '关闭 Codex',
  },
  {
    key: 'restart',
    icon: 'refresh',
    title: '重启 Codex',
    description: '按原版流程退出并重新启动本地 Codex 进程，应用配置变更或恢复异常状态。',
    action: '重启',
    busy: '运行中...',
    confirm: '确定重启 Codex？',
    confirmText: '将终止当前 Codex 进程并重新启动。正在进行的操作可能会中断。',
    confirmAccept: '重启',
  },
]

const imageRow: Row = {
  key: 'image',
  icon: 'grid',
  title: 'Image 声明过滤',
  description:
    '开启：在 ChatGPT 配置中关闭生图声明，并从中转请求中移除生图工具，避免不支持生图的上游返回 403；关闭：取消该限制并恢复声明透传，由模型和上游决定是否支持。此开关不会新增生图能力。',
  action: '切换',
}

const snapshot = ref<MaintenanceSnapshot | null>(null)
const settings = ref<MaintenanceUtilitySettings | null>(null)
const loading = ref(true)
const busy = ref<Tool | null>(null)
const loadError = ref('')
const settingsError = ref('')
const errors = ref<Partial<Record<Tool, string>>>({})
const results = ref<Partial<Record<Tool, string>>>({})
const diagnosticOpen = ref(false)
const diagnosticReport = ref<DiagnosticReport | null>(null)
const confirmRow = ref<Row | null>(null)
const sendOpen = ref(false)
const endpoint = ref('')
const userNote = ref('')
const sendAccepted = ref(false)
const sendError = ref('')
let disposed = false

const locked = computed(() => loading.value || busy.value !== null || !snapshot.value)

const message = (e: unknown) => (e instanceof Error ? e.message : String(e))

async function load() {
  if (busy.value) return
  loading.value = true
  loadError.value = ''
  settingsError.value = ''
  const [a, b] = await Promise.allSettled([
    maintenanceService.snapshot(),
    maintenanceService.utilitySettings(),
  ])
  if (disposed) return
  if (a.status === 'fulfilled') snapshot.value = a.value
  else loadError.value = message(a.reason)
  if (b.status === 'fulfilled') {
    settings.value = b.value
    endpoint.value = b.value.diagnosticEndpoint ?? ''
  } else settingsError.value = message(b.reason)
  loading.value = false
}

/* 原版是一次点击直接执行；预览与写入在后端串成一步，用户不再逐步确认。 */
async function execute(action: MaintenanceAction): Promise<MaintenanceResult | null> {
  const plan = await maintenanceService.preview(action)
  return maintenanceService.apply(plan.ticket)
}

async function run(row: Row) {
  if (busy.value) return
  errors.value[row.key] = ''

  if (row.key === 'diagnose') {
    diagnosticOpen.value = true
    return
  }
  if (row.key === 'send') {
    sendOpen.value = true
    sendAccepted.value = false
    sendError.value = ''
    return
  }
  if (row.confirm) {
    confirmRow.value = row
    return
  }
  await perform(row.key)
}

async function perform(key: Tool) {
  busy.value = key
  try {
    if (key === 'export') {
      if (maintenanceService.saveBundle) {
        const saved = await maintenanceService.saveBundle()
        if (!disposed && saved.saved)
          results.value.export = `调试诊断包已导出：${saved.path ?? saved.summary}`
      } else {
        const bundle: DiagnosticBundle = await maintenanceService.exportBundle()
        if (disposed) return
        download(bundle)
      }
      return
    }
    if (key === 'image') {
      if (!settings.value) return
      const plan = await maintenanceService.previewImageCompat(!settings.value.imageCompat)
      const value = await maintenanceService.applyImageCompat(plan.ticket)
      if (disposed) return
      settings.value = value
      results.value.image = value.imageCompat
        ? '已更新，将在下一次中转请求立即生效'
        : '已更新，将在下一次中转请求立即生效'
      return
    }
    if (key === 'restart' || key === 'forceKill' || key === 'clean' || key === 'rebuild') {
      const value = await execute(key)
      if (disposed) return
      results.value[key] =
        key === 'restart'
          ? 'Codex 已重启'
          : key === 'forceKill'
            ? value?.summary || `已关闭 ${value?.changed ?? 0} 个进程`
            : (value?.summary ?? '已执行')
      try {
        snapshot.value = await maintenanceService.snapshot()
      } catch {
        /* 结果已写入，概览刷新失败不覆盖成功事实 */
      }
    }
  } catch (e) {
    if (!disposed) errors.value[key] = message(e)
  } finally {
    if (!disposed) busy.value = null
    confirmRow.value = null
  }
}

function download(bundle: DiagnosticBundle) {
  let url: string | undefined
  try {
    url = URL.createObjectURL(new Blob([new Uint8Array(bundle.bytes)], { type: 'application/zip' }))
    const a = document.createElement('a')
    a.href = url
    a.download = bundle.fileName
    a.click()
    results.value.export = `${bundle.summary} 已请求保存 ${bundle.fileName}。`
  } catch (e) {
    errors.value.export = `保存失败：${message(e)}`
  } finally {
    if (url) {
      const objectUrl = url
      setTimeout(() => URL.revokeObjectURL(objectUrl), 30000)
    }
  }
}

function diagnosticCompleted(value: DiagnosticReport) {
  diagnosticReport.value = value
  const issues = value.items.filter((i) => ['error', 'warning'].includes(i.status)).length
  results.value.diagnose = `${value.items.length} 项检查 · ${issues} 项需处理`
}

const endpointValid = computed(() => {
  try {
    const url = new URL(endpoint.value)
    return (
      (url.protocol === 'https:' ||
        (url.protocol === 'http:' && ['localhost', '127.0.0.1', '[::1]'].includes(url.hostname))) &&
      !url.username &&
      !url.password &&
      !url.search &&
      !url.hash
    )
  } catch {
    return false
  }
})

async function send() {
  if (!endpointValid.value || !sendAccepted.value || busy.value) return
  busy.value = 'send'
  sendError.value = ''
  try {
    const value = await maintenanceService.sendBundle(endpoint.value.trim(), userNote.value.trim())
    if (disposed) return
    results.value.send = `${value.message}${value.reportId ? ` · ${value.reportId}` : ''}`
    sendOpen.value = false
    userNote.value = ''
  } catch (e) {
    if (!disposed) sendError.value = message(e)
  } finally {
    if (!disposed) busy.value = null
  }
}

const imageOn = computed(() => !!settings.value?.imageCompat)

onMounted(load)
onBeforeUnmount(() => {
  disposed = true
})
</script>

<template>
  <section class="mnt-page" aria-labelledby="maintenance-title">
    <header class="mnt-head">
      <h1 id="maintenance-title">维护工具</h1>
      <p>先运行系统诊断；诊断包与低频的清理、重建、进程操作收纳在各自折叠区。</p>
      <span v-if="snapshot?.mode === 'demo'" class="mnt-demo">演示数据 · 不连接本机</span>
    </header>

    <p v-if="loadError" class="mnt-error" role="alert">
      {{ loadError }}<button type="button" class="mnt-text-button" @click="load">重试</button>
    </p>

    <div class="mnt-card mnt-feature">
      <article :data-row="diagnoseRow.key" class="mnt-row">
        <Icon :name="diagnoseRow.icon" :size="18" class="mnt-row-icon" />
        <div class="mnt-row-copy">
          <h2>{{ diagnoseRow.title }}</h2>
          <p>{{ diagnoseRow.description }}</p>
          <p v-if="results.diagnose" class="mnt-row-result">{{ results.diagnose }}</p>
          <p v-if="errors.diagnose" class="mnt-row-error" role="alert">{{ errors.diagnose }}</p>
        </div>
        <button
          type="button"
          class="mnt-action mnt-action-primary"
          :disabled="locked || (busy !== null && busy !== 'diagnose')"
          :aria-busy="busy === 'diagnose'"
          @click="run(diagnoseRow)"
        >
          <Icon v-if="busy === 'diagnose'" name="refresh" :size="13" class="mnt-spin" />{{
            busy === 'diagnose' ? (diagnoseRow.busy ?? '处理中...') : diagnoseRow.action
          }}
        </button>
      </article>

      <article :data-row="imageRow.key" class="mnt-row">
        <Icon :name="imageRow.icon" :size="18" class="mnt-row-icon" />
        <div class="mnt-row-copy">
          <h2>{{ imageRow.title }}</h2>
          <p>{{ imageRow.description }}</p>
          <p v-if="errors.image" class="mnt-row-error" role="alert">{{ errors.image }}</p>
          <p v-if="settingsError" class="mnt-row-error" role="alert">{{ settingsError }}</p>
        </div>
        <button
          type="button"
          class="mnt-switch"
          role="switch"
          :aria-checked="imageOn"
          :aria-label="imageRow.title"
          :disabled="locked"
          @click="run(imageRow)"
        >
          <span class="mnt-switch-track"><i /></span>
          <small>{{ !settings ? '未读取' : imageOn ? '正在过滤' : '允许透传' }}</small>
        </button>
      </article>
    </div>

    <details class="mnt-fold" data-section="package">
      <summary>
        <Icon name="download" :size="16" />诊断包
        <small>导出或发送脱敏诊断信息</small>
      </summary>
      <div class="mnt-card">
        <article v-for="row in packageRows" :key="row.key" :data-row="row.key" class="mnt-row">
          <Icon :name="row.icon" :size="18" class="mnt-row-icon" />
          <div class="mnt-row-copy">
            <h2>{{ row.title }}</h2>
            <p>{{ row.description }}</p>
            <p v-if="results[row.key]" class="mnt-row-result">{{ results[row.key] }}</p>
            <p v-if="errors[row.key]" class="mnt-row-error" role="alert">{{ errors[row.key] }}</p>
          </div>
          <button
            type="button"
            class="mnt-action"
            :disabled="locked || (busy !== null && busy !== row.key)"
            :aria-busy="busy === row.key"
            @click="run(row)"
          >
            <Icon v-if="busy === row.key" name="refresh" :size="13" class="mnt-spin" />{{
              busy === row.key ? (row.busy ?? '处理中...') : row.action
            }}
          </button>
        </article>
      </div>
    </details>

    <details class="mnt-fold" data-section="advanced">
      <summary>
        <Icon name="settings" :size="16" />高级维护
        <small>低频：清理、重建与进程操作</small>
      </summary>
      <div class="mnt-card">
        <article v-for="row in advancedRows" :key="row.key" :data-row="row.key" class="mnt-row">
          <Icon :name="row.icon" :size="18" class="mnt-row-icon" />
          <div class="mnt-row-copy">
            <h2>{{ row.title }}</h2>
            <p>{{ row.description }}</p>
            <p v-if="results[row.key]" class="mnt-row-result">{{ results[row.key] }}</p>
            <p v-if="errors[row.key]" class="mnt-row-error" role="alert">{{ errors[row.key] }}</p>
          </div>
          <button
            type="button"
            class="mnt-action"
            :class="{ 'mnt-action-danger': row.key === 'forceKill' }"
            :disabled="locked || (busy !== null && busy !== row.key)"
            :aria-busy="busy === row.key"
            @click="run(row)"
          >
            <Icon v-if="busy === row.key" name="refresh" :size="13" class="mnt-spin" />{{
              busy === row.key ? (row.busy ?? '处理中...') : row.action
            }}
          </button>
        </article>
      </div>
    </details>

    <p class="mnt-foot">
      ATELIER / MAINTENANCE<span v-if="snapshot"
        >{{ snapshot.platform }} · {{ snapshot.version }}</span
      >
    </p>

    <Modal
      v-if="confirmRow"
      title="请确认"
      :close-disabled="busy === confirmRow.key"
      @close="confirmRow = null"
    >
      <div class="mnt-confirm">
        <h2>{{ confirmRow.confirm }}</h2>
        <p>{{ confirmRow.confirmText }}</p>
        <div class="mnt-dialog-footer">
          <button type="button" class="mnt-text-button" @click="confirmRow = null">取消</button>
          <button
            type="button"
            class="mnt-primary-button"
            :disabled="!!busy"
            @click="perform(confirmRow.key)"
          >
            {{ confirmRow.confirmAccept }}
          </button>
        </div>
      </div>
    </Modal>

    <Modal
      v-if="sendOpen"
      title="发送诊断信息"
      :close-disabled="busy === 'send'"
      @close="sendOpen = false"
    >
      <div class="mnt-send">
        <p class="mnt-dialog-desc">可以补充问题现象、发生时间和操作步骤，便于快速定位。</p>
        <textarea
          v-model="userNote"
          class="mnt-textarea"
          rows="4"
          placeholder="例如：开启路由后关闭失败，发生于刚才重启 ChatGPT 之后…"
        />
        <label class="mnt-field">
          <span>接收地址</span>
          <input v-model="endpoint" type="url" placeholder="https://" />
        </label>
        <p class="mnt-hint">发送内容经过脱敏，不包含 Prompt、Token、API Key、Cookie 或完整对话。</p>
        <label class="mnt-check">
          <input v-model="sendAccepted" type="checkbox" />
          <span>我确认向上述地址发送脱敏诊断包。</span>
        </label>
        <p v-if="sendError" class="mnt-error" role="alert">{{ sendError }}</p>
        <div class="mnt-dialog-footer">
          <button type="button" class="mnt-text-button" @click="sendOpen = false">取消</button>
          <button
            type="button"
            class="mnt-primary-button"
            :disabled="!endpointValid || !sendAccepted || !!busy"
            @click="send"
          >
            {{ busy === 'send' ? '发送中...' : '确认发送' }}
          </button>
        </div>
      </div>
    </Modal>

    <DiagnosticDialog
      v-if="diagnosticOpen"
      @close="diagnosticOpen = false"
      @completed="diagnosticCompleted"
    />
  </section>
</template>
