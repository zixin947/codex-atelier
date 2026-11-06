<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import Icon from '../../shared/components/Icon.vue'
import Modal from '../../shared/components/Modal.vue'
import DiagnosticDialog from './DiagnosticDialog.vue'
import { maintenanceService } from '../../services/maintenance'
import type {
  DiagnosticBundle,
  DiagnosticReport,
  ImageCompatPlan,
  MaintenanceAction,
  MaintenancePlan,
  MaintenanceResult,
  MaintenanceSnapshot,
  MaintenanceUtilitySettings,
} from '../../types/maintenance'
import './maintenance.css'

type Tool = MaintenanceAction | 'export' | 'send' | 'image' | 'searchCompat'
const snapshot = ref<MaintenanceSnapshot | null>(null)
const settings = ref<MaintenanceUtilitySettings | null>(null)
const loading = ref(true)
const busy = ref<Tool | null>(null)
const loadError = ref('')
const settingsError = ref('')
const errors = ref<Partial<Record<Tool, string>>>({})
const results = ref<Partial<Record<Tool, { text: string; at: number }>>>({})
const lastResult = ref<MaintenanceResult | null>(null)
const plan = ref<MaintenancePlan | null>(null)
const imagePlan = ref<ImageCompatPlan | null>(null)
const accepted = ref(false)
const applyError = ref('')
const isApplying = ref(false)
const diagnosticOpen = ref(false)
const diagnosticReport = ref<DiagnosticReport | null>(null)
const bundle = ref<DiagnosticBundle | null>(null)
const sendOpen = ref(false)
const endpoint = ref('')
const userNote = ref('')
const sendAccepted = ref(false)
const sendError = ref('')
const selected = ref<Tool>('diagnose')
let disposed = false
const locked = computed(
  () =>
    loading.value ||
    busy.value !== null ||
    !!plan.value ||
    !!imagePlan.value ||
    diagnosticOpen.value ||
    sendOpen.value ||
    !!bundle.value,
)
const actions: {
  key: Tool
  icon: string
  label: string
  tag: string
  description: string
  action: string
  tone?: string
}[] = [
  {
    key: 'diagnose',
    icon: 'pulse',
    label: '系统诊断',
    tag: 'INSPECT',
    description: '检查配置、认证、运行时与线程，查看问题详情并修复可处理的项目。',
    action: '开始诊断',
    tone: 'blue',
  },
  {
    key: 'export',
    icon: 'download',
    label: '导出诊断包',
    tag: 'EXPORT',
    description: '生成可检查的诊断资料，排除登录凭据、会话正文和个人文件。',
    action: '生成诊断包',
    tone: 'cyan',
  },
  {
    key: 'send',
    icon: 'upload',
    label: '发送诊断信息',
    tag: 'SUPPORT',
    description: '附上问题说明，将诊断包发送到你指定的接收地址。',
    action: '填写并发送',
    tone: 'green',
  },
  {
    key: 'clean',
    icon: 'trash',
    label: '清理数据',
    tag: 'CLEAN',
    description: '整理 Atelier 旧备份与失效账号索引，保留有效账号档案。',
    action: '预览清理',
    tone: 'orange',
  },
  {
    key: 'forceKill',
    icon: 'close',
    label: '强制结束 Desktop',
    tag: 'TERMINATE',
    description: '结束已识别的客户端进程。尚未保存的工作可能丢失。',
    action: '预览结束',
    tone: 'red',
  },
  {
    key: 'image',
    icon: 'grid',
    label: 'Image 声明过滤',
    tag: 'COMPATIBILITY',
    description:
      '关闭生图声明并过滤 Atelier 网关生图工具；不影响图片输入，不新增生图能力。关闭后移除限制并恢复透传。',
    action: '切换',
    tone: 'purple',
  },
  {
    key: 'searchCompat',
    icon: 'search',
    label: 'Claude 联网兼容',
    tag: 'PLANNED',
    description: '上游当前版本尚未开放，此入口暂不可用。',
    action: '尚未开放',
    tone: 'blue',
  },
  {
    key: 'restart',
    icon: 'refresh',
    label: '重启 Desktop',
    tag: 'RESTART',
    description: '正常退出并重新启动已识别的客户端，请先保存工作。',
    action: '预览重启',
    tone: 'red',
  },
]
const actionName = (key: Tool) =>
  key === 'rebuild' ? '重建账号注册表' : (actions.find((a) => a.key === key)?.label ?? key)
const time = (value: number) =>
  new Date(value).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  })
const message = (e: unknown) => (e instanceof Error ? e.message : String(e))
function record(key: Tool, text: string) {
  results.value[key] = { text, at: Date.now() }
  selected.value = key
}
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
async function run(action: Tool) {
  if (locked.value || action === 'searchCompat') return
  selected.value = action
  errors.value[action] = ''
  if (action === 'diagnose') {
    diagnosticOpen.value = true
    return
  }
  if (action === 'send') {
    sendOpen.value = true
    sendAccepted.value = false
    sendError.value = ''
    return
  }
  busy.value = action
  try {
    if (action === 'export') {
      if (maintenanceService.saveBundle) {
        const saved = await maintenanceService.saveBundle()
        if (!disposed && saved.saved)
          record('export', `${saved.summary}${saved.path ? ` · ${saved.path}` : ''}`)
      } else {
        const response = await maintenanceService.exportBundle()
        if (!disposed) bundle.value = response
      }
    } else if (action === 'image') {
      if (!settings.value) return
      const response = await maintenanceService.previewImageCompat(!settings.value.imageCompat)
      if (!disposed) {
        imagePlan.value = response
        accepted.value = false
        applyError.value = ''
      }
    } else {
      const response = await maintenanceService.preview(action)
      if (!disposed) {
        plan.value = response
        accepted.value = false
        applyError.value = ''
      }
    }
  } catch (e) {
    if (!disposed) errors.value[action] = message(e)
  } finally {
    if (!disposed) busy.value = null
  }
}
function closePreview() {
  if (!isApplying.value) {
    plan.value = null
    imagePlan.value = null
    accepted.value = false
    applyError.value = ''
  }
}
async function apply() {
  if ((!plan.value && !imagePlan.value) || !accepted.value || isApplying.value || applyError.value)
    return
  isApplying.value = true
  const current = plan.value
  const image = imagePlan.value
  try {
    if (image) {
      const value = await maintenanceService.applyImageCompat(image.ticket)
      if (disposed) return
      settings.value = value
      record(
        'image',
        value.imageCompat ? '已启用 Image 声明过滤。' : '已关闭 Image 声明过滤，恢复透传。',
      )
    } else if (current) {
      const value = await maintenanceService.apply(current.ticket)
      if (disposed) return
      lastResult.value = value
      record(current.action, value.summary)
      try {
        snapshot.value = await maintenanceService.snapshot()
      } catch (e) {
        loadError.value = `操作已完成，概览更新失败：${message(e)}`
      }
    }
    plan.value = null
    imagePlan.value = null
  } catch (e) {
    if (!disposed) applyError.value = message(e)
  } finally {
    if (!disposed) isApplying.value = false
  }
}
function diagnosticCompleted(value: DiagnosticReport) {
  diagnosticReport.value = value
  record(
    'diagnose',
    `${value.items.length} 项检查 · ${value.items.filter((i) => ['error', 'warning'].includes(i.status)).length} 项需处理`,
  )
}
function downloadBundle() {
  if (!bundle.value) return
  let url: string | undefined
  try {
    url = URL.createObjectURL(
      new Blob([new Uint8Array(bundle.value.bytes)], { type: 'application/zip' }),
    )
    const a = document.createElement('a')
    a.href = url
    a.download = bundle.value.fileName
    a.click()
    record('export', `${bundle.value.summary} 已请求保存 ${bundle.value.fileName}。`)
    bundle.value = null
  } catch (e) {
    errors.value.export = `保存失败：${message(e)}`
  } finally {
    if (url) {
      const objectUrl = url
      setTimeout(() => URL.revokeObjectURL(objectUrl), 30000)
    }
  }
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
function closeSend() {
  if (busy.value !== 'send') sendOpen.value = false
}
async function send() {
  if (!endpointValid.value || !sendAccepted.value || busy.value) return
  busy.value = 'send'
  sendError.value = ''
  try {
    const value = await maintenanceService.sendBundle(endpoint.value.trim(), userNote.value.trim())
    if (disposed) return
    record('send', `${value.message}${value.reportId ? ` · ${value.reportId}` : ''}`)
    sendOpen.value = false
    userNote.value = ''
  } catch (e) {
    if (!disposed) sendError.value = message(e)
  } finally {
    if (!disposed) busy.value = null
  }
}
onMounted(load)
onBeforeUnmount(() => {
  disposed = true
})
</script>

<template>
  <section class="maintenance-page" aria-labelledby="maintenance-title">
    <header class="maintenance-heading">
      <div>
        <p class="maintenance-eyebrow">WORKSTATION / MAINTENANCE</p>
        <h1 id="maintenance-title">维护工具 <span>09</span></h1>
        <p class="maintenance-subtitle">检查环境、整理数据，让工作恢复有序。</p>
      </div>
      <div class="maintenance-heading-tools">
        <span class="maintenance-mode">{{
          snapshot?.mode === 'demo' ? '演示环境' : snapshot ? '本机管理' : '环境载入中'
        }}</span
        ><button class="button" :disabled="locked" @click="load">
          <Icon name="refresh" :class="{ 'maintenance-spin': loading }" />刷新状态
        </button>
      </div>
    </header>
    <div v-if="loadError" class="maintenance-error" role="alert">
      {{ loadError }}<button class="button" :disabled="locked" @click="load">重试</button>
    </div>
    <p v-if="snapshot?.mode === 'demo'" class="maintenance-demo">
      <span class="maintenance-mode-dot" />演示数据 · 操作仅更新示例状态，不连接本机 Codex。
    </p>
    <div class="maintenance-layout">
      <div class="maintenance-main">
        <div class="maintenance-actions-title">
          <span>维护操作</span><small>08 OPERATIONS</small>
        </div>
        <div class="maintenance-actions" :aria-busy="!!busy">
          <article
            v-for="(action, index) in actions"
            :key="action.key"
            class="maintenance-action"
            :class="{ 'maintenance-action-selected': selected === action.key }"
            :data-tone="action.tone"
          >
            <div class="maintenance-action-symbol">
              <span class="maintenance-index">{{ String(index + 1).padStart(2, '0') }}</span
              ><Icon :name="action.icon" :size="21" />
            </div>
            <div class="maintenance-action-copy">
              <div class="maintenance-action-name">
                <h2>{{ action.label }}</h2>
                <span>{{ action.tag }}</span>
              </div>
              <p>{{ action.description }}</p>
              <p v-if="results[action.key]" class="maintenance-last-result">
                <Icon name="check" :size="12" />{{ time(results[action.key]!.at) }} ·
                {{ results[action.key]!.text }}
              </p>
            </div>
            <button
              v-if="action.key === 'image' || action.key === 'searchCompat'"
              type="button"
              class="maintenance-switch"
              role="switch"
              :aria-checked="action.key === 'image' && !!settings?.imageCompat"
              :aria-label="action.label"
              :disabled="locked || action.key === 'searchCompat' || !settings"
              @click="run(action.key)"
            >
              <span /><small>{{
                action.key === 'searchCompat'
                  ? '未开放'
                  : !settings
                    ? '未读取'
                    : settings.imageCompat
                      ? '已开启'
                      : '已关闭'
              }}</small>
            </button>
            <button
              v-else
              class="button maintenance-run"
              :class="{
                primary: action.key === 'diagnose',
                'maintenance-danger': action.key === 'forceKill',
              }"
              :disabled="locked || !snapshot"
              @click="run(action.key)"
            >
              <Icon
                v-if="busy === action.key"
                name="refresh"
                class="maintenance-spin"
                :size="14"
              />{{ busy === action.key ? '处理中…' : action.action
              }}<Icon v-if="busy !== action.key" name="arrow" :size="14" />
            </button>
            <p
              v-if="errors[action.key] || (action.key === 'image' && settingsError)"
              class="maintenance-action-error"
              role="alert"
            >
              <Icon name="alert" :size="14" />{{ errors[action.key] || settingsError }}
            </p>
          </article>
        </div>
        <details class="maintenance-advanced">
          <summary>其他维护操作 <span>账号注册表恢复</span></summary>
          <div>
            <p>从 Atelier 保存的账号档案重建列表，保留已有名称和分组。</p>
            <button class="button" :disabled="locked || !snapshot" @click="run('rebuild')">
              预览重建
            </button>
          </div>
          <p v-if="errors.rebuild" class="maintenance-error" role="alert">{{ errors.rebuild }}</p>
          <p v-if="results.rebuild" class="maintenance-last-result">{{ results.rebuild.text }}</p>
        </details>
        <section v-if="lastResult" class="maintenance-result" aria-live="polite">
          <div class="maintenance-result-heading">
            <h2>{{ actionName(lastResult.action) }} · 执行记录</h2>
            <span>{{ time(lastResult.completedAt) }}</span>
          </div>
          <p class="maintenance-result-summary">
            <Icon name="check" :size="16" />{{ lastResult.summary }}
          </p>
          <p v-for="warning in lastResult.warnings" :key="warning" class="maintenance-warning">
            {{ warning }}
          </p>
          <div v-if="lastResult.backupPath" class="maintenance-backup">
            <Icon name="box" :size="15" />
            <div>
              <span>操作备份</span><code>{{ lastResult.backupPath }}</code>
            </div>
          </div>
        </section>
      </div>
      <aside class="maintenance-aside" aria-label="当前维护环境">
        <div class="maintenance-environment">
          <div class="maintenance-environment-label">
            <span>当前环境</span><Icon name="chip" :size="17" />
          </div>
          <strong
            >{{ snapshot?.platform || '—' }}<small>{{ snapshot?.arch || '载入中' }}</small></strong
          >
          <dl>
            <div>
              <dt>应用版本</dt>
              <dd>{{ snapshot?.version || '—' }}</dd>
            </div>
            <div>
              <dt>已登记账号</dt>
              <dd>{{ snapshot?.accountCount ?? '—' }}<small> 个</small></dd>
            </div>
          </dl>
          <div class="maintenance-environment-footer">
            <i /><span>{{ snapshot?.mode === 'demo' ? 'DEMO WORKSPACE' : 'LOCAL WORKSPACE' }}</span>
          </div>
        </div>
        <div class="maintenance-scope">
          <h2>环境路径</h2>
          <div>
            <span>Atelier 管理目录</span><code>{{ snapshot?.managedRoot || '等待环境信息' }}</code>
          </div>
          <div>
            <span>Desktop</span
            ><code>{{
              snapshot?.desktopPath ||
              (snapshot?.mode === 'demo' ? '演示环境不检测客户端' : '未识别安装位置')
            }}</code>
          </div>
          <p>系统诊断按需读取环境。修复、清理和进程操作分别确认，不会随打开页面执行。</p>
        </div>
        <div v-if="diagnosticReport" class="maintenance-scope maintenance-latest-scan">
          <h2>
            最近诊断 <small>{{ time(diagnosticReport.checkedAt) }}</small>
          </h2>
          <strong>{{ diagnosticReport.items.length }}<span> 项检查</span></strong>
          <p>{{ results.diagnose?.text }}</p>
          <button class="button" :disabled="locked" @click="run('diagnose')">打开诊断</button>
        </div>
        <div class="maintenance-footnote">
          <Icon name="shield" :size="16" /><span>检查与修改分开<br />修改先预览，执行留结果</span>
        </div>
      </aside>
    </div>
    <DiagnosticDialog
      v-if="diagnosticOpen"
      @close="diagnosticOpen = false"
      @completed="diagnosticCompleted"
    />
    <Modal
      v-if="plan || imagePlan"
      :title="plan?.title || (imagePlan?.enabled ? '启用 Image 声明过滤' : '关闭 Image 声明过滤')"
      @close="closePreview"
      ><div class="maintenance-preview">
        <p class="maintenance-preview-subtitle">
          {{ snapshot?.mode === 'demo' ? '演示操作 · 不影响本机' : '请核对修改范围' }}
        </p>
        <ol class="maintenance-changes">
          <li v-for="(change, index) in plan?.changes || imagePlan?.changes" :key="index">
            <span>{{ String(index + 1).padStart(2, '0') }}</span
            >{{ change }}
          </li>
        </ol>
        <p
          v-for="warning in plan?.warnings || imagePlan?.warnings"
          :key="warning"
          class="maintenance-warning"
        >
          <Icon name="alert" :size="14" />{{ warning }}
        </p>
        <label class="maintenance-confirm"
          ><input v-model="accepted" type="checkbox" :disabled="isApplying" /><span>{{
            plan?.action === 'forceKill'
              ? '我已保存工作，确认强制结束上述 Desktop 进程。'
              : plan?.action === 'restart'
                ? '我已保存工作，确认重启上述 Desktop。'
                : '我已核对上述范围，确认执行。'
          }}</span></label
        >
        <p v-if="applyError" class="maintenance-error" role="alert">{{ applyError }}</p>
        <div class="maintenance-preview-actions">
          <button class="button" :disabled="isApplying" @click="closePreview">
            {{ applyError ? '关闭并重新预览' : '取消' }}</button
          ><button
            class="button primary"
            :disabled="!accepted || isApplying || !!applyError || !!(plan && !plan.count)"
            @click="apply"
          >
            <Icon v-if="isApplying" name="refresh" class="maintenance-spin" :size="14" />{{
              isApplying ? '执行中…' : '确认并执行'
            }}
          </button>
        </div>
      </div></Modal
    >
    <Modal v-if="bundle" title="导出诊断包" @close="bundle = null"
      ><div class="maintenance-preview">
        <p class="maintenance-preview-subtitle">{{ bundle.summary }}</p>
        <div class="maintenance-bundle-name">
          <Icon name="box" :size="24" />
          <div>
            <b>{{ bundle.fileName }}</b
            ><span>{{ (bundle.bytes.length / 1024).toFixed(1) }} KB</span>
          </div>
        </div>
        <h3 class="maintenance-form-label">包含内容</h3>
        <ul class="maintenance-bundle-list">
          <li v-for="item in bundle.included" :key="item">
            <Icon name="check" :size="13" />{{ item }}
          </li>
        </ul>
        <h3 class="maintenance-form-label">排除内容</h3>
        <ul class="maintenance-bundle-list">
          <li v-for="item in bundle.excluded" :key="item">
            <Icon name="shield" :size="13" />{{ item }}
          </li>
        </ul>
        <div class="maintenance-preview-actions">
          <button class="button" @click="bundle = null">取消</button
          ><button class="button primary" @click="downloadBundle">
            <Icon name="download" :size="15" />保存诊断包
          </button>
        </div>
      </div></Modal
    >
    <Modal v-if="sendOpen" title="发送诊断信息" @close="closeSend"
      ><form class="maintenance-support-form" @submit.prevent="send">
        <p class="maintenance-preview-subtitle">
          发送前请核对接收方。Atelier 不会自动上传，也没有预设的第三方接收地址。
        </p>
        <label class="maintenance-form-label" for="diagnostic-endpoint">接收地址</label
        ><input
          id="diagnostic-endpoint"
          v-model="endpoint"
          type="url"
          placeholder="https://your-support.example/diagnostics"
          :disabled="busy === 'send'"
          autocomplete="off"
          required
        /><small
          >使用你的 HTTPS 接收器；本机测试可使用回环 HTTP
          地址。不接受含账号、查询参数或片段的地址。</small
        ><label class="maintenance-form-label" for="diagnostic-note"
          >问题说明 <span>选填 · {{ userNote.length }}/2000</span></label
        ><textarea
          id="diagnostic-note"
          v-model="userNote"
          maxlength="2000"
          placeholder="描述问题、发生时间与复现步骤。请勿填写密码或 API Key。"
          :disabled="busy === 'send'"
        /><label class="maintenance-confirm"
          ><input v-model="sendAccepted" type="checkbox" :disabled="busy === 'send'" /><span
            >我确认将诊断资料与上述说明发送到此地址，已检查说明中没有敏感信息。</span
          ></label
        >
        <p v-if="sendError" class="maintenance-error" role="alert">{{ sendError }}</p>
        <div class="maintenance-preview-actions">
          <button type="button" class="button" :disabled="busy === 'send'" @click="closeSend">
            取消</button
          ><button
            class="button primary"
            type="submit"
            :disabled="!endpointValid || !sendAccepted || !!busy"
          >
            <Icon name="upload" :size="15" />{{
              busy === 'send' ? '发送中…' : snapshot?.mode === 'demo' ? '演示发送' : '确认发送'
            }}
          </button>
        </div>
      </form></Modal
    >
  </section>
</template>
