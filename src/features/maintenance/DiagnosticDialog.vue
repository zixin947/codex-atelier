<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import diagnosticMeme from '../../assets/maintenance/diagnostic-ciallo.png'
import { maintenanceService } from '../../services/maintenance'
import type {
  DiagnosticFixResult,
  DiagnosticGroup,
  DiagnosticItem,
  DiagnosticReport,
} from '../../types/maintenance'

/* 结构与文案对齐 AiMaMi 1.2.7 系统诊断：弹窗标题、四组滚动列表、条目行、
   修复按钮、修复结果块。交互按原版执行一键修复，不再由用户逐步确认；
   备份与读回校验仍在后端完成。 */

const emit = defineEmits<{ close: []; completed: [report: DiagnosticReport] }>()

const report = ref<DiagnosticReport | null>(null)
const log = ref<string[]>([])
const error = ref('')
const busy = ref<'scan' | 'fix' | 'all' | null>(null)
const fixingId = ref<string | null>(null)
const expanded = ref(new Set<string>())
const content = ref<HTMLElement>()
let scrolled = false
let generation = 0
let alive = true

const groups: { id: DiagnosticGroup; name: string }[] = [
  { id: 'config', name: '配置' },
  { id: 'auth', name: '认证' },
  { id: 'runtime', name: '运行时与线程' },
  { id: 'env', name: '环境与磁盘' },
]

const message = (e: unknown) => (e instanceof Error ? e.message : String(e))

/* 过渡态：原版把每一行 label 换成「过渡说明 + 原始检测：<label>」 */
const transitionPrefix =
  '智能路由正在切换（线程收敛进行中，线程越多/磁盘越慢耗时越长）。' +
  '该项是过渡态，收敛完成后会自动消失——请勿在切换过程中点击修复或反复开关。原始检测：'

function labelOf(item: DiagnosticItem) {
  return report.value?.transitionInFlight ? transitionPrefix + item.label : item.label
}

function itemsOf(group: DiagnosticGroup) {
  return (report.value?.items ?? []).filter((i) => i.group === group)
}

const fixableCount = computed(() => {
  const value = report.value
  if (!value || value.transitionInFlight) return 0
  return value.items.filter((i) => i.fixable && i.status !== 'ok').length
})

const allClear = computed(
  () =>
    !busy.value &&
    report.value !== null &&
    report.value.items.length > 0 &&
    report.value.items.every((item) => item.status === 'ok') &&
    !report.value.transitionInFlight &&
    !error.value,
)

function isFixable(item: DiagnosticItem) {
  return item.fixable && item.status !== 'ok' && !report.value?.transitionInFlight
}

function toggle(id: string) {
  const next = new Set(expanded.value)
  next.has(id) ? next.delete(id) : next.add(id)
  expanded.value = next
}

async function scan() {
  if (busy.value) return
  const token = ++generation
  busy.value = 'scan'
  error.value = ''
  log.value = []
  try {
    const value = await maintenanceService.runDiagnostics()
    if (!alive || token !== generation) return
    report.value = value
    emit('completed', value)
    if (!scrolled) {
      scrolled = true
      await nextTick()
      content.value?.querySelector<HTMLElement>('.mnt-groups')?.scrollTo({ top: 0 })
    }
  } catch (e) {
    if (alive && token === generation) {
      error.value = message(e)
    }
  } finally {
    if (alive && token === generation) busy.value = null
  }
}

/* 原版「修复」是一键执行：这里把预览与写入串成一次调用，中途失败即终止。 */
async function fix(itemId: string) {
  if (busy.value || !alive) return
  const token = ++generation
  busy.value = 'fix'
  fixingId.value = itemId
  error.value = ''
  try {
    const applied =
      itemId === 'codex_home_writable'
        ? await maintenanceService
            .previewDirectoryProbe()
            .then((plan) => maintenanceService.applyDirectoryProbe(plan.ticket))
        : await maintenanceService
            .previewRepair(itemId)
            .then((plan) => maintenanceService.applyRepair(plan.ticket))
    if (!alive || token !== generation) return
    showLog(applied)
    await refresh(token)
  } catch (e) {
    if (alive && token === generation) error.value = message(e)
  } finally {
    if (alive && token === generation) {
      busy.value = null
      fixingId.value = null
    }
  }
}

async function fixAll() {
  if (busy.value || !alive || !fixableCount.value) return
  const token = ++generation
  busy.value = 'all'
  error.value = ''
  try {
    const plan = await maintenanceService.previewRepair('all')
    if (!alive || token !== generation) return
    const applied = await maintenanceService.applyRepair(plan.ticket)
    if (!alive || token !== generation) return
    showLog(applied)
    await refresh(token)
  } catch (e) {
    if (alive && token === generation) error.value = message(e)
  } finally {
    if (alive && token === generation) busy.value = null
  }
}

function showLog(result: DiagnosticFixResult) {
  log.value = result.details
}

async function refresh(token: number) {
  const value = await maintenanceService.runDiagnostics()
  if (!alive || token !== generation) return
  report.value = value
  emit('completed', value)
}

function close() {
  if (busy.value !== 'all' && busy.value !== 'fix') emit('close')
}

onMounted(scan)
onBeforeUnmount(() => {
  alive = false
  generation++
})
</script>

<template>
  <Modal
    class="mnt-dialog-modal"
    title="系统诊断"
    :close-disabled="busy === 'fix' || busy === 'all'"
    @close="close"
  >
    <div ref="content" class="mnt-dialog" :aria-busy="!!busy">
      <p v-if="!allClear" class="mnt-dialog-desc">
        全面检测 Codex
        运行环境：配置文件、认证状态、进程健康、线程数据、磁盘权限和密钥完整性，发现异常可一键修复。
      </p>

      <p v-if="error && !allClear" class="mnt-error" role="alert">{{ error }}</p>

      <div v-if="busy === 'scan'" class="mnt-loading" role="status">
        <Icon name="refresh" :size="18" class="mnt-spin" /><span>正在检测…</span>
      </div>

      <div v-else-if="allClear" class="mnt-clear" role="status">
        <div class="mnt-clear-copy">
          <span class="mnt-clear-check"><Icon name="check" :size="18" /></span>
          <p>当前诊断一切正常</p>
        </div>
        <img
          class="mnt-clear-meme"
          :src="diagnosticMeme"
          alt="Ciallo，开心比耶的表情包"
          width="128"
          height="96"
        />
      </div>

      <template v-else-if="report">
        <div class="mnt-groups">
          <section v-for="group in groups" :key="group.id" class="mnt-group">
            <template v-if="itemsOf(group.id).length">
              <p class="mnt-group-label">{{ group.name }}</p>
              <div
                v-for="item in itemsOf(group.id)"
                :key="item.id"
                class="mnt-item"
                :data-status="item.status"
              >
                <div class="mnt-item-row">
                  <Icon
                    :name="
                      item.status === 'ok'
                        ? 'check'
                        : item.status === 'error'
                          ? 'close'
                          : item.status === 'warning'
                            ? 'alert'
                            : 'list'
                    "
                    :size="14"
                    class="mnt-item-icon"
                  />
                  <button
                    type="button"
                    class="mnt-item-label"
                    :aria-expanded="expanded.has(item.id)"
                    @click="item.detail && toggle(item.id)"
                  >
                    {{ labelOf(item) }}
                  </button>
                  <button
                    v-if="isFixable(item)"
                    type="button"
                    class="mnt-fix"
                    :disabled="!!busy"
                    :aria-label="`修复 ${item.label}`"
                    :aria-busy="fixingId === item.id"
                    @click="fix(item.id)"
                  >
                    <Icon
                      v-if="fixingId === item.id"
                      name="refresh"
                      :size="11"
                      class="mnt-spin"
                    />修复
                  </button>
                </div>
                <p v-if="expanded.has(item.id) && item.detail" class="mnt-item-detail">
                  {{ item.detail }}
                </p>
              </div>
            </template>
          </section>
        </div>

        <div v-if="log.length && !allClear" class="mnt-log" aria-live="polite">
          <p class="mnt-log-title">修复结果</p>
          <p v-for="(line, index) in log" :key="index" class="mnt-log-line">{{ line }}</p>
        </div>
      </template>
      <div class="mnt-dialog-footer">
        <button
          type="button"
          class="mnt-text-button"
          :disabled="!!busy"
          :aria-busy="busy === 'scan'"
          @click="scan"
        >
          <Icon v-if="busy === 'scan'" name="refresh" :size="12" class="mnt-spin" />{{
            busy === 'scan' ? '正在检测…' : '重新检测'
          }}
        </button>
        <button
          v-if="fixableCount && !allClear"
          type="button"
          class="mnt-primary-button"
          :disabled="!!busy"
          :aria-busy="busy === 'all'"
          @click="fixAll"
        >
          <Icon v-if="busy === 'all'" name="refresh" :size="12" class="mnt-spin" />{{
            busy === 'all' ? '修复中…' : `全部修复（${fixableCount} 项）`
          }}
        </button>
      </div>
    </div>
  </Modal>
</template>
