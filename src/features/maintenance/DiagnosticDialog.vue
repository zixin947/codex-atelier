<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import { maintenanceService } from '../../services/maintenance'
import type {
  DiagnosticFixResult,
  DiagnosticGroup,
  DiagnosticReport,
  DiagnosticRepairPlan,
  DiagnosticStatus,
} from '../../types/maintenance'

const emit = defineEmits<{ close: []; completed: [report: DiagnosticReport] }>()
const report = ref<DiagnosticReport | null>(null)
const result = ref<DiagnosticFixResult | null>(null)
const plan = ref<DiagnosticRepairPlan | null>(null)
const busy = ref<'scan' | 'preview' | 'apply' | null>(null)
const error = ref('')
const planError = ref('')
const accepted = ref(false)
const showAll = ref(false)
const expanded = ref(new Set<string>())
const content = ref<HTMLElement>()
const previewHeading = ref<HTMLElement>()
const resultHeading = ref<HTMLElement>()
const recheckButton = ref<HTMLButtonElement>()
let previewSource: string | null = null
let generation = 0
let alive = true
const groups: { id: DiagnosticGroup; name: string; icon: string }[] = [
  { id: 'config', name: '配置', icon: 'settings' },
  { id: 'auth', name: '认证', icon: 'shield' },
  { id: 'runtime', name: '运行时与线程', icon: 'pulse' },
  { id: 'env', name: '环境与磁盘', icon: 'box' },
]
const statusNames: Record<DiagnosticStatus, string> = {
  ok: '正常',
  warning: '留意',
  error: '异常',
  unknown: '未确认',
  skipped: '不适用',
}
const fixable = computed(
  () => report.value?.items.filter((i) => i.fixable && !['ok', 'skipped'].includes(i.status)) ?? [],
)
const summary = computed(() => {
  const items = report.value?.items ?? []
  return {
    ok: items.filter((i) => i.status === 'ok').length,
    issues: items.filter((i) => ['warning', 'error'].includes(i.status)).length,
    unknown: items.filter((i) => i.status === 'unknown').length,
    skipped: items.filter((i) => i.status === 'skipped').length,
  }
})
const message = (e: unknown) => (e instanceof Error ? e.message : String(e))
function close() {
  if (busy.value !== 'apply') emit('close')
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
  try {
    const value = await maintenanceService.runDiagnostics()
    if (!alive || token !== generation) return
    report.value = value
    emit('completed', value)
  } catch (e) {
    if (alive && token === generation) error.value = message(e)
  } finally {
    if (alive && token === generation) busy.value = null
  }
}
async function preview(itemId: string) {
  if (busy.value) return
  previewSource = itemId
  const token = ++generation
  busy.value = 'preview'
  error.value = ''
  planError.value = ''
  try {
    const value = await maintenanceService.previewRepair(itemId)
    if (!alive || token !== generation) return
    plan.value = value
    accepted.value = false
    await nextTick()
    if (alive && token === generation) previewHeading.value?.focus()
  } catch (e) {
    if (alive && token === generation) error.value = message(e)
  } finally {
    if (alive && token === generation) busy.value = null
  }
}
async function back() {
  if (busy.value !== 'apply') {
    plan.value = null
    planError.value = ''
    accepted.value = false
    await nextTick()
    if (!alive) return
    const source = Array.from(
      content.value?.querySelectorAll<HTMLButtonElement>('[data-repair-id]') ?? [],
    ).find((button) => button.dataset.repairId === previewSource && !button.disabled)
    ;(source ?? recheckButton.value)?.focus()
  }
}
async function apply() {
  if (!plan.value || !accepted.value || busy.value || planError.value) return
  const ticket = plan.value.ticket
  const token = ++generation
  busy.value = 'apply'
  try {
    const value = await maintenanceService.applyRepair(ticket)
    if (!alive || token !== generation) return
    result.value = value
    report.value = value.report
    plan.value = null
    emit('completed', value.report)
    await nextTick()
    if (alive && token === generation) (resultHeading.value ?? recheckButton.value)?.focus()
  } catch (e) {
    if (alive && token === generation) planError.value = message(e)
  } finally {
    if (alive && token === generation) busy.value = null
  }
}
onMounted(scan)
onBeforeUnmount(() => {
  alive = false
  generation++
})
</script>

<template>
  <Modal
    class="maintenance-diagnostic-modal"
    title="系统诊断"
    :close-disabled="busy === 'apply'"
    @close="close"
  >
    <div ref="content" class="maintenance-diagnostic" :aria-busy="!!busy">
      <template v-if="plan">
        <div class="diagnostic-intro">
          <span class="diagnostic-kicker">REPAIR PREVIEW</span>
          <h2 ref="previewHeading" tabindex="-1">{{ plan.title }}</h2>
          <p>核对将要修改的内容。执行前会检查外部变更并保留备份。</p>
        </div>
        <div class="diagnostic-plan-body">
          <ol class="maintenance-changes">
            <li v-for="(change, index) in plan.changes" :key="index">
              <span>{{ String(index + 1).padStart(2, '0') }}</span
              >{{ change }}
            </li>
          </ol>
          <p v-for="warning in plan.warnings" :key="warning" class="maintenance-warning">
            <Icon name="alert" :size="14" />{{ warning }}
          </p>
          <p v-if="plan.requiresDesktopClosed" class="maintenance-warning">
            <Icon name="alert" :size="14" />这项修复需要 Desktop
            已退出。请保存工作并手动退出后执行，Atelier 不会自动结束你的会话。
          </p>
          <label class="maintenance-confirm"
            ><input v-model="accepted" type="checkbox" :disabled="!!busy" /><span
              >我已核对修改范围，确认修复这 {{ plan.itemIds.length }} 项。</span
            ></label
          >
          <p v-if="planError" class="maintenance-error" role="alert">{{ planError }}</p>
        </div>
        <footer class="diagnostic-footer">
          <button class="button" :disabled="busy === 'apply'" @click="back">返回检查结果</button
          ><button
            class="button primary"
            :disabled="!accepted || !!busy || !!planError"
            @click="apply"
          >
            <Icon v-if="busy === 'apply'" name="refresh" class="maintenance-spin" :size="14" />{{
              busy === 'apply' ? '修复并复检…' : '确认修复'
            }}
          </button>
        </footer>
      </template>
      <template v-else>
        <div class="diagnostic-intro">
          <span class="diagnostic-kicker">SYSTEM / INTEGRITY</span>
          <p>检查配置、认证、运行时、线程与目录权限，查看问题详情并修复可处理的项目。</p>
          <span v-if="report?.mode === 'demo'" class="diagnostic-demo-tag"
            >演示数据 · 不影响本机</span
          >
        </div>
        <p v-if="error" class="maintenance-error" role="alert">{{ error }}</p>
        <div v-if="busy === 'scan' && !report" class="diagnostic-loading" role="status">
          <Icon name="refresh" :size="25" class="maintenance-spin" /><b>正在检查环境</b
          ><span>等待实际检查结果，不会修改配置与会话。</span>
        </div>
        <template v-if="report">
          <div class="diagnostic-summary" aria-label="诊断概览">
            <div>
              <b>{{ report.items.length }}</b
              ><span>检查项目</span>
            </div>
            <div data-status="ok">
              <b>{{ summary.ok }}</b
              ><span>正常</span>
            </div>
            <div :data-status="summary.issues ? 'warning' : ''">
              <b>{{ summary.issues }}</b
              ><span>需处理</span>
            </div>
            <div>
              <b>{{ summary.unknown + summary.skipped }}</b
              ><span>未确认 / 不适用</span>
            </div>
          </div>
          <p v-if="report.transitionInFlight" class="maintenance-warning">
            <Icon name="alert" :size="14" />连接状态正在变更，部分结果尚未稳定，请稍后重新检测。
          </p>
          <div v-if="!report.hasIssues && !showAll" class="diagnostic-all-clear">
            <Icon name="shield" :size="30" /><b>未发现需要处理的问题</b>
            <p v-if="summary.unknown">
              还有 {{ summary.unknown }} 项未能确认，请展开查看检查范围。
            </p>
            <button class="button" @click="showAll = true">查看全部检查</button>
          </div>
          <div v-else class="diagnostic-groups">
            <section v-for="group in groups" :key="group.id" class="diagnostic-group">
              <header>
                <Icon :name="group.icon" :size="13" />
                <h3>{{ group.name }}</h3>
                <span>{{ report.items.filter((i) => i.group === group.id).length }}</span>
              </header>
              <article
                v-for="item in report.items.filter((i) => i.group === group.id)"
                :key="item.id"
                class="diagnostic-item"
                :data-status="item.status"
              >
                <div class="diagnostic-item-line">
                  <span class="diagnostic-status-icon"
                    ><Icon
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
                  /></span>
                  <button
                    class="diagnostic-item-label"
                    :aria-expanded="expanded.has(item.id)"
                    :aria-controls="`diagnostic-detail-${item.id}`"
                    @click="toggle(item.id)"
                  >
                    {{ item.label
                    }}<Icon
                      name="chevron"
                      :size="11"
                      :class="{ expanded: expanded.has(item.id) }"
                    />
                  </button>
                  <span class="diagnostic-status-text">{{ statusNames[item.status] }}</span>
                  <button
                    v-if="item.fixable && !['ok', 'skipped'].includes(item.status)"
                    class="button diagnostic-fix-one"
                    :disabled="!!busy"
                    :aria-label="`修复 ${item.label}`"
                    :data-repair-id="item.id"
                    @click="preview(item.id)"
                  >
                    修复
                  </button>
                </div>
                <p
                  v-if="expanded.has(item.id)"
                  :id="`diagnostic-detail-${item.id}`"
                  class="diagnostic-detail"
                >
                  {{ item.detail }}
                </p>
              </article>
            </section>
          </div>
          <div v-if="result" class="diagnostic-fix-log" aria-live="polite">
            <h3 ref="resultHeading" tabindex="-1">
              修复记录
              <span
                >{{ result.fixedIds.length }} 项完成 · {{ result.failedIds.length }} 项未完成</span
              >
            </h3>
            <p v-for="(line, index) in result.details" :key="index">{{ line }}</p>
            <code v-if="result.backupPath">备份：{{ result.backupPath }}</code>
          </div>
        </template>
        <footer class="diagnostic-footer">
          <div>
            <button ref="recheckButton" class="button" :disabled="!!busy" @click="scan">
              <Icon name="refresh" :size="14" :class="{ 'maintenance-spin': busy === 'scan' }" />{{
                busy === 'scan' ? '检测中…' : '重新检测'
              }}</button
            ><small v-if="report">{{
              new Date(report.checkedAt).toLocaleTimeString('zh-CN')
            }}</small>
          </div>
          <button
            v-if="fixable.length"
            class="button primary"
            :disabled="!!busy || report?.transitionInFlight"
            data-repair-id="all"
            @click="preview('all')"
          >
            <Icon name="settings" :size="14" />{{
              busy === 'preview' ? '准备修复预览…' : `全部修复（${fixable.length} 项）`
            }}
          </button>
        </footer>
      </template>
    </div>
  </Modal>
</template>
