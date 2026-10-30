<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import Icon from '../../shared/components/Icon.vue'
import Modal from '../../shared/components/Modal.vue'
import { pluginModeService } from '../../services/pluginModes'
import type { BehaviorPluginId } from '../../types/plugins'
import type {
  PluginModeService,
  PonytailMode,
  PonytailModeState,
  StsTaskMode,
  StsControlLevel,
  StsSessionMode,
} from '../../types/pluginModes'

const props = defineProps<{
  pluginId: BehaviorPluginId
  installed: boolean | null
  enabled: boolean | null
  duplicates: boolean
  locked?: boolean
  service?: PluginModeService
}>()
const emit = defineEmits<{ busy: [value: boolean] }>()
const adapter = props.service ?? pluginModeService
const busy = ref(false),
  loading = ref(false),
  error = ref(''),
  receipt = ref('')
const pony = ref<PonytailModeState | null>(null),
  ponyDraft = ref<PonytailMode>('full')
const sessions = ref<StsSessionMode[]>([]),
  selectedId = ref(''),
  skipped = ref(0),
  manualId = ref('')
const task = ref<StsTaskMode>('unconfirmed'),
  level = ref<StsControlLevel>('watch'),
  paths = ref('')
const preview = ref<'save' | 'rollback' | null>(null)
const selected = computed(() => sessions.value.find((s) => s.id === selectedId.value) ?? null)
const blocked = computed(
  () => busy.value || loading.value || props.locked || props.duplicates || props.installed !== true,
)
const pathList = computed(() => [
  ...new Set(
    paths.value
      .split(/\r?\n/)
      .map((s) => s.trim())
      .filter(Boolean),
  ),
])
const changed = computed(() =>
  props.pluginId === 'ponytail'
    ? !!pony.value && pony.value.configMode !== ponyDraft.value
    : !!selected.value &&
      (selected.value.mode !== task.value ||
        selected.value.level !== level.value ||
        (level.value === 'lock' &&
          JSON.stringify(selected.value.allowedPaths ?? []) !== JSON.stringify(pathList.value))),
)
const invalid = computed(() =>
  props.pluginId === 'ponytail'
    ? !!pony.value?.environmentMode
    : !selected.value ||
      props.enabled !== true ||
      (['guard', 'lock'].includes(level.value) && task.value === 'unconfirmed') ||
      (level.value === 'lock' && pathList.value.length === 0),
)
const canRollback = computed(() =>
  props.pluginId === 'ponytail'
    ? !!pony.value?.canRollback && !pony.value.environmentMode
    : !!selected.value?.canRollback && props.enabled === true,
)
const ponyModes: { id: PonytailMode; label: string; description: string }[] = [
  { id: 'lite', label: 'Lite', description: '轻量 · 提示更简单的选择' },
  { id: 'full', label: 'Full', description: '标准 · 最小完整实现' },
  { id: 'ultra', label: 'Ultra', description: '深入 · 主动质疑额外复杂度' },
  { id: 'off', label: 'Off', description: '不注入 Ponytail 模式' },
]
const tasks: { id: StsTaskMode; label: string }[] = [
  { id: 'unconfirmed', label: '未确认任务' },
  { id: 'answer', label: 'Answer · 回答' },
  { id: 'review', label: 'Review · 审查' },
  { id: 'monitor', label: 'Monitor · 监测' },
  { id: 'change', label: 'Change · 修改' },
  { id: 'open', label: 'Open · 通用任务' },
]
const levels: { id: StsControlLevel; label: string }[] = [
  { id: 'watch', label: 'Watch · 观察' },
  { id: 'guard', label: 'Guard · 检查边界' },
  { id: 'lock', label: 'Lock · 文件边界' },
  { id: 'off', label: 'Off · 关闭 Guard' },
]
async function stepPony(event: KeyboardEvent, direction: number) {
  if (blocked.value || pony.value?.environmentMode) return
  const at = ponyModes.findIndex((mode) => mode.id === ponyDraft.value)
  ponyDraft.value = ponyModes[(at + direction + ponyModes.length) % ponyModes.length]!.id
  const group = event.currentTarget as HTMLElement
  await nextTick()
  group.querySelector<HTMLButtonElement>('[aria-checked="true"]')?.focus()
}
function resetSts() {
  task.value = selected.value?.mode ?? 'unconfirmed'
  level.value = selected.value?.level ?? 'watch'
  paths.value = selected.value?.allowedPaths?.join('\n') ?? ''
  receipt.value = ''
  error.value = ''
}
watch(selectedId, resetSts)
let loadGeneration = 0
async function refresh() {
  if (busy.value || props.locked || props.installed !== true || props.duplicates) return
  const generation = ++loadGeneration
  loading.value = true
  error.value = ''
  try {
    if (props.pluginId === 'ponytail') {
      const value = await adapter.ponytail()
      if (generation !== loadGeneration) return
      pony.value = value
      ponyDraft.value = value.configMode ?? value.mode
    } else {
      const value = await adapter.stsSessions()
      if (generation !== loadGeneration) return
      sessions.value = value.sessions
      skipped.value = value.skipped
      if (selectedId.value && !sessions.value.some((s) => s.id === selectedId.value))
        selectedId.value = ''
      resetSts()
    }
  } catch (e) {
    if (generation === loadGeneration) error.value = String(e)
  } finally {
    if (generation === loadGeneration) loading.value = false
  }
}
async function readById() {
  if (blocked.value || !manualId.value.trim()) return
  loading.value = true
  error.value = ''
  try {
    const value = await adapter.stsSession(manualId.value.trim())
    sessions.value = [value, ...sessions.value.filter((s) => s.id !== value.id)]
    selectedId.value = value.id
    resetSts()
  } catch (e) {
    error.value = String(e)
  } finally {
    loading.value = false
  }
}
async function commit() {
  if (blocked.value || !preview.value) return
  if (preview.value === 'save' && (!changed.value || invalid.value)) return
  if (preview.value === 'rollback' && !canRollback.value) return
  const operation = preview.value
  // Capture target and version before any asynchronous work; selection is locked
  // until readback. Never report a previous target's result under a new selection.
  const target = selected.value
  busy.value = true
  emit('busy', true)
  error.value = ''
  receipt.value = ''
  try {
    if (props.pluginId === 'ponytail' && pony.value) {
      const value =
        operation === 'save'
          ? await adapter.setPonytail(ponyDraft.value, pony.value.revision)
          : await adapter.rollbackPonytail(pony.value.revision)
      pony.value = value
      ponyDraft.value = value.configMode ?? value.mode
      receipt.value = value.receipt ?? '默认档位已保存并读回；请新建对话加载。'
    } else if (target) {
      const value =
        operation === 'save'
          ? await adapter.setSts({
              sessionId: target.id,
              expectedRevision: target.revision,
              mode: task.value,
              level: level.value,
              allowedPaths: level.value === 'lock' ? pathList.value : null,
            })
          : await adapter.rollbackSts(target.id, target.revision)
      if (value.id !== target.id) throw new Error('返回的对话与目标不一致，请刷新核对。')
      sessions.value = sessions.value.map((s) => (s.id === value.id ? value : s))
      task.value = value.mode
      level.value = value.level
      paths.value = value.allowedPaths?.join('\n') ?? ''
      receipt.value = value.receipt ?? '该对话模式已保存并读回；下一次消息重新注入说明。'
    }
    preview.value = null
  } catch (e) {
    error.value = String(e)
    preview.value = null
  } finally {
    busy.value = false
    emit('busy', false)
  }
}
onMounted(() => {
  void refresh()
})
watch(
  () => [props.installed, props.duplicates],
  () => {
    void refresh()
  },
)
onUnmounted(() => {
  ++loadGeneration
  if (busy.value) emit('busy', false)
})
</script>

<template>
  <section
    class="plugin-modes inset"
    :aria-label="pluginId === 'ponytail' ? 'Ponytail 模式控制' : 'STS 模式控制'"
  >
    <div class="mode-heading">
      <div>
        <p class="eyebrow">
          {{ pluginId === 'ponytail' ? 'DEFAULT INTENSITY' : 'SESSION CONTRACT' }}
        </p>
        <h3>{{ pluginId === 'ponytail' ? '新对话默认档位' : '指定对话的任务模式' }}</h3>
      </div>
      <button
        class="icon-button"
        :disabled="blocked"
        :aria-label="pluginId === 'ponytail' ? '刷新 Ponytail 模式' : '刷新 STS 对话'"
        @click="refresh"
      >
        <Icon name="refresh" :size="15" :class="{ spinning: loading }" />
      </button>
    </div>
    <p v-if="installed !== true" class="mode-note">安装插件后可读取与设置模式。</p>
    <p v-else-if="duplicates" class="mode-note">插件来源重复，保留一个来源后再设置。</p>
    <p v-if="loading" class="mode-note" role="status">正在读取上游模式状态…</p>
    <template v-if="pluginId === 'ponytail' && pony">
      <div
        class="mode-options"
        role="radiogroup"
        aria-label="Ponytail 默认档位"
        @keydown.left.prevent="stepPony($event, -1)"
        @keydown.right.prevent="stepPony($event, 1)"
      >
        <button
          v-for="option in ponyModes"
          :key="option.id"
          class="mode-option"
          :class="{ selected: ponyDraft === option.id }"
          role="radio"
          :aria-checked="ponyDraft === option.id"
          :tabindex="ponyDraft === option.id ? 0 : -1"
          :disabled="blocked || !!pony.environmentMode"
          :title="option.description"
          @click="ponyDraft = option.id"
        >
          <b>{{ option.label }}</b
          ><span>{{ option.description.split(' · ')[0] }}</span>
        </button>
      </div>
      <p class="mode-note">
        已保存：<strong>{{ pony.configMode ?? 'Full（上游默认）' }}</strong> · 仅供新对话的
        SessionStart 读取。与插件总开关独立。
      </p>
      <p v-if="pony.environmentMode" class="mode-warning" role="status">
        检测到当前进程环境覆盖：{{ pony.environmentMode }}。环境优先，暂不允许保存无效的文件设置。
      </p>
      <p v-if="enabled === false" class="mode-note">
        插件已停用，保存仅预设档位，不会自动启用插件。
      </p>
      <details class="mode-details">
        <summary>配置位置与生效范围</summary>
        <code>{{ pony.path }}</code>
        <p>
          上游默认配置可能被其他宿主共享。Desktop 自身的环境变量或 XDG
          路径不同会覆盖此处；这里不代表旧对话的运行档位。
        </p>
      </details>
    </template>
    <template v-else-if="pluginId === 'stop-that-shit' && installed">
      <label class="mode-field"
        ><span>目标对话</span
        ><select
          v-model="selectedId"
          :disabled="blocked || !sessions.length"
          aria-label="STS 目标对话"
        >
          <option value="" disabled>
            {{ sessions.length ? '请选择要修改的对话' : '暂无已运行 STS 的近期对话' }}
          </option>
          <option v-for="session in sessions" :key="session.id" :value="session.id">
            {{ session.title }} · {{ session.id.slice(-6) }}
          </option>
        </select></label
      >
      <p v-if="!sessions.length && !loading && !error" class="mode-note">
        启用 STS 后，在新对话发送一条普通消息，再刷新此处。只列出已有 STS 状态的对话。
      </p>
      <p v-if="skipped" class="mode-warning">
        {{ skipped }} 个状态因不可读取或超出最近数量上限未列出，可按 ID 单独核对。
      </p>
      <details class="mode-details">
        <summary>按对话 ID 读取</summary>
        <div class="mode-id-row">
          <input
            v-model="manualId"
            :disabled="blocked"
            aria-label="STS 对话 ID"
            placeholder="完整对话 UUID"
          /><button class="button" :disabled="blocked || !manualId.trim()" @click="readById">
            读取
          </button>
        </div>
      </details>
      <div v-if="selected" class="mode-session-meta">
        <code>{{ selected.id }}</code
        ><span :title="selected.cwd">{{ selected.cwd }}</span
        ><b>已保存：{{ selected.mode }} / {{ selected.level }}</b>
      </div>
      <div class="mode-field-grid">
        <label class="mode-field"
          ><span>任务模式</span
          ><select v-model="task" :disabled="blocked || !selected" aria-label="STS 任务模式">
            <option
              v-for="item in tasks"
              :key="item.id"
              :value="item.id"
              :disabled="item.id === 'unconfirmed'"
            >
              {{ item.label }}
            </option>
          </select></label
        ><label class="mode-field"
          ><span>控制等级</span
          ><select v-model="level" :disabled="blocked || !selected" aria-label="STS 控制等级">
            <option v-for="item in levels" :key="item.id" :value="item.id">{{ item.label }}</option>
          </select></label
        >
      </div>
      <label v-if="level === 'lock'" class="mode-field"
        ><span>文件边界 · 每行一个路径，可用目录/**</span
        ><textarea
          v-model="paths"
          :disabled="blocked || !selected"
          aria-label="STS 文件边界"
          rows="3"
          placeholder="src/config.ts&#10;tests/config.test.ts"
        />
      </label>
      <p v-if="['guard', 'lock'].includes(level) && task === 'unconfirmed'" class="mode-warning">
        先选择明确任务模式，再启用 Guard 或 Lock。
      </p>
      <p v-if="enabled !== true" class="mode-warning">
        STS 插件已停用。先启用并让目标新对话触发一次 Hook，才能应用会话契约。
      </p>
      <p class="mode-note">
        只修改所选对话。下一次消息重新注入说明，后续 Hook
        读取新契约；不会发送聊天消息、清除既有账本或自动信任。
      </p>
      <p v-if="selected?.allowedPaths?.length && level !== 'lock'" class="mode-note">
        已有文件边界会保留：{{ selected.allowedPaths.join('、') }}
      </p>
    </template>
    <p v-if="error" class="mode-error" role="alert">
      {{ error }}<span v-if="!loading"> 请刷新核对最新状态后重试。</span>
    </p>
    <p v-if="receipt" class="mode-receipt" role="status">{{ receipt }}</p>
    <div v-if="pony || selected" class="mode-actions">
      <button
        class="button primary"
        :disabled="blocked || invalid || !changed"
        @click="preview = 'save'"
      >
        {{ busy ? '正在保存并读回…' : '预览并保存模式' }}</button
      ><button class="button" :disabled="blocked || !canRollback" @click="preview = 'rollback'">
        回退上次模式
      </button>
    </div>
    <Modal
      v-if="preview"
      :title="preview === 'save' ? '确认模式变化' : '回退上次模式'"
      @close="!busy && (preview = null)"
    >
      <p class="eyebrow">
        {{
          pluginId === 'ponytail' ? 'PONYTAIL / NEW CONVERSATIONS' : 'STS / SELECTED CONVERSATION'
        }}
      </p>
      <template v-if="pluginId === 'ponytail'"
        ><h2>
          {{
            preview === 'save'
              ? `${pony?.configMode ?? 'Full（默认）'} → ${ponyDraft}`
              : '恢复上一份默认档位'
          }}
        </h2>
        <p>
          只修改 Ponytail 默认档位字段，新对话启动时读取。不会开启上方插件总开关或改变已有对话。
        </p></template
      >
      <template v-else
        ><h2>{{ selected?.title }}</h2>
        <code class="mode-preview-id">{{ selected?.id }}</code>
        <p>
          {{ selected?.mode }} / {{ selected?.level }} →
          {{ preview === 'save' ? `${task} / ${level}` : '上次契约' }}
        </p>
        <p v-if="level === 'lock' && preview === 'save'">文件边界：{{ pathList.join('、') }}</p>
        <p>
          通过 STS 上游会话锁更新该对话契约，保留委派账本与其他参数；下一次消息注入说明。实际
          Desktop 效果由你测试。
        </p></template
      >
      <p v-if="error" class="mode-error" role="alert">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="busy" @click="preview = null">取消</button
        ><button class="button primary" :disabled="busy" @click="commit">
          {{ busy ? '保存中…' : preview === 'save' ? '确认保存模式' : '确认回退模式' }}
        </button>
      </div>
    </Modal>
  </section>
</template>
