<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import Panel from '../../shared/components/Panel.vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import HookList from './HookList.vue'
import PluginModes from './PluginModes.vue'
import { useAtelier } from '../../app/store'
import { behaviorPluginService } from '../../services/plugins'
import type {
  BehaviorPlugin,
  BehaviorPluginService,
  PluginSnapshot,
  HookTrust,
} from '../../types/plugins'
import type { PluginModeService } from '../../types/pluginModes'
const props = defineProps<{ service?: BehaviorPluginService; modeService?: PluginModeService }>()
const adapter = props.service ?? behaviorPluginService
const store = useAtelier(),
  snapshot = ref<PluginSnapshot | null>(null),
  busy = ref(''),
  pendingAction = ref(''),
  error = ref(''),
  installing = ref<BehaviorPlugin | null>(null),
  receipts = ref<Record<string, { error: boolean; message: string }>>({})
function trustLabel(trust: HookTrust) {
  switch (trust.state) {
    case 'trusted':
      return '已信任'
    case 'reviewRequired':
      return trust.modified ? '定义已变化，需重新审阅' : '有 Hook 待信任'
    case 'notLoaded':
      return '未加载，无法判断信任'
    case 'notInstalled':
      return '尚未安装'
    case 'unavailable':
      return '无法读取'
  }
}
function hooksOff(plugin: BehaviorPlugin) {
  return plugin.installed === true && plugin.enabled === false
}
function modeBusy(id: string, value: boolean) {
  if (value) busy.value = `mode-${id}`
  else if (busy.value === `mode-${id}`) busy.value = ''
}
async function refresh(preserveReceipt = false) {
  if (busy.value) return
  busy.value = 'refresh'
  if (!preserveReceipt) {
    error.value = ''
    receipts.value = {}
  }
  try {
    snapshot.value = await adapter.status()
  } catch (e) {
    error.value = String(e)
  } finally {
    busy.value = ''
  }
}
async function mutate(plugin: BehaviorPlugin, operation: 'install' | 'toggle' | 'rollback') {
  if (!snapshot.value?.available || busy.value) return
  busy.value = plugin.id
  pendingAction.value =
    operation === 'toggle'
      ? plugin.enabled
        ? '正在停用并保存…'
        : '正在启用并保存…'
      : operation === 'rollback'
        ? '正在回退并读回…'
        : '正在安装…'
  error.value = ''
  const revision = snapshot.value.revision
  delete receipts.value[plugin.id]
  try {
    snapshot.value =
      operation === 'install'
        ? await adapter.install(plugin.id, revision)
        : operation === 'rollback'
          ? await adapter.rollback(plugin.id, revision)
          : await adapter.setEnabled(plugin.id, !plugin.enabled, revision)
    installing.value = null
    const updated = snapshot.value.plugins.find((p) => p.id === plugin.id)
    const message =
      operation === 'install'
        ? `${plugin.name} 已安装，默认停用 · Hook 状态见卡片`
        : `${plugin.name} ${updated?.enabled ? '已启用' : '已停用'} · 已保存并读回，${snapshot.value.runtimeReceipt ?? '请新建 Desktop 对话加载最新开关，无需重启应用'}`
    receipts.value[plugin.id] = { error: false, message }
    store.notify(message)
  } catch (e) {
    error.value = String(e)
    receipts.value[plugin.id] = { error: true, message: `操作未确认成功：${String(e)}` }
    try {
      snapshot.value = await adapter.status()
    } catch {
      /* Keep last known snapshot, with the error visible. */
    }
  } finally {
    busy.value = ''
    pendingAction.value = ''
  }
}
function resumeStatus() {
  if (!document.hidden && !installing.value && !busy.value) void refresh(true)
}
onMounted(() => {
  void refresh()
  window.addEventListener('focus', resumeStatus)
  document.addEventListener('visibilitychange', resumeStatus)
})
onUnmounted(() => {
  window.removeEventListener('focus', resumeStatus)
  document.removeEventListener('visibilitychange', resumeStatus)
})
</script>
<template>
  <div class="page behavior-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / BEHAVIOR PLUGINS</p>
        <h1>少一点折腾，多一点专注<span class="heading-dot">.</span></h1>
        <p class="subtitle">Ponytail 与 Stop That Shit · Codex Desktop 插件管理</p>
      </div>
      <button class="button" :disabled="!!busy" @click="refresh()">
        <Icon name="refresh" :size="16" :class="{ spinning: busy === 'refresh' }" />{{
          busy === 'refresh' ? '检测中…' : '刷新本机状态'
        }}
      </button>
    </div>
    <div class="plugin-scope inset">
      <Icon name="shield" :size="21" />
      <div>
        <b>真实插件配置，与演示数据分开</b>
        <p>
          开关在后台保存，新建 Desktop 对话时加载，无需重启应用。首次使用或定义变化时，按 Desktop
          提示审阅并信任 Hook；已有信任会保留。
        </p>
      </div>
      <span class="badge">USER SCOPE</span>
    </div>
    <div v-if="error" role="alert" class="error-state plugin-error">{{ error }}</div>
    <div v-if="snapshot && !snapshot.available" class="plugin-unavailable" role="status">
      <Icon name="terminal" :size="20" /><span>{{ snapshot.reason }}</span>
    </div>
    <div v-if="!snapshot" class="empty" role="status">正在读取 Desktop 内置引擎状态…</div>
    <div v-if="snapshot" class="behavior-grid">
      <Panel
        v-for="plugin in snapshot.plugins"
        :key="plugin.id"
        :title="plugin.name"
        :code="plugin.id === 'ponytail' ? 'SIMPLICITY' : 'TASK BOUNDARIES'"
      >
        <div class="behavior-card-heading">
          <span class="behavior-logo" :class="{ stop: plugin.id === 'stop-that-shit' }"
            ><Icon :name="plugin.id === 'ponytail' ? 'plug' : 'shield'" :size="30" /></span
          ><span
            class="badge"
            :class="{ neutral: !plugin.enabled, orange: plugin.installed === false }"
            >{{
              plugin.installed === null
                ? '状态未检测'
                : !plugin.installed
                  ? '尚未安装'
                  : plugin.enabled
                    ? '配置已启用'
                    : '配置已停用'
            }}</span
          >
        </div>
        <h2>{{ plugin.summary }}</h2>
        <p class="behavior-description">{{ plugin.description }}</p>
        <div class="setting-row real-setting">
          <span
            ><b>插件加载开关</b
            ><small
              :title="
                snapshot.desktopBridge.state === 'connected'
                  ? '影响该宿主的全部已加载会话；共用宿主的其他客户端也可能受到影响。'
                  : undefined
              "
              >{{
                busy === plugin.id
                  ? pendingAction
                  : plugin.installed
                    ? snapshot.desktopBridge.state === 'connected'
                      ? '当前会话控制 · ' + snapshot.desktopBridge.loadedThreads + ' 个已加载会话'
                      : '新对话生效 · 无需重启 Desktop'
                    : '先安装，再启用'
              }}</small
            ></span
          ><button
            class="switch"
            :class="{ on: plugin.enabled }"
            role="switch"
            :aria-checked="plugin.enabled === true"
            :aria-label="`${plugin.name} 插件加载`"
            :disabled="!snapshot.available || !plugin.installed || !!busy || plugin.duplicates"
            @click="mutate(plugin, 'toggle')"
          >
            <span />
          </button>
        </div>
        <div
          v-if="receipts[plugin.id]"
          class="plugin-operation-result"
          :class="{ failed: receipts[plugin.id]!.error }"
          :role="receipts[plugin.id]!.error ? 'alert' : 'status'"
        >
          <span>{{ receipts[plugin.id]!.message }}</span>
          <button
            v-if="receipts[plugin.id]!.error"
            class="button"
            :disabled="!!busy"
            @click="refresh()"
          >
            刷新状态
          </button>
        </div>
        <div class="behavior-actions">
          <button
            v-if="!plugin.installed"
            class="button primary"
            :disabled="!snapshot.available || !!busy"
            @click="installing = plugin"
          >
            <Icon name="box" :size="16" />安装固定版本</button
          ><button
            v-else
            class="button"
            :disabled="!snapshot.available || !plugin.canRollback || !!busy"
            @click="mutate(plugin, 'rollback')"
          >
            <Icon name="history" :size="16" />回退上次切换</button
          ><span class="tiny muted">{{ plugin.license }} / OFFICIAL SOURCE</span>
        </div>
        <PluginModes
          :plugin-id="plugin.id"
          :installed="plugin.installed"
          :enabled="plugin.enabled"
          :duplicates="plugin.duplicates"
          :locked="!!busy && busy !== `mode-${plugin.id}`"
          :service="modeService"
          @busy="modeBusy(plugin.id, $event)"
        />
        <div
          class="inset behavior-trust"
          :class="{
            'trust-ok': plugin.enabled === true && plugin.hookTrust.state === 'trusted',
            'trust-review': plugin.enabled === true && plugin.hookTrust.state === 'reviewRequired',
          }"
          role="status"
          :aria-label="`${plugin.name} Hook 信任状态`"
        >
          <Icon
            :name="
              hooksOff(plugin)
                ? 'plug'
                : plugin.hookTrust.state === 'trusted'
                  ? 'shield'
                  : plugin.hookTrust.state === 'reviewRequired'
                    ? 'alert'
                    : 'terminal'
            "
            :size="17"
          />
          <div>
            <b>{{
              hooksOff(plugin)
                ? 'Hooks · 随插件停用'
                : plugin.enabled === true
                  ? 'Hooks · 随插件加载'
                  : 'Hooks · 状态未检测'
            }}</b>
            <p v-if="hooksOff(plugin)">
              总开关已关闭，新会话不加载该插件的
              Hook。已连接宿主以切换后的热重载回执为准；关闭不会清除信任记录。
            </p>
            <template v-else>
              <p>信任状态 · {{ trustLabel(plugin.hookTrust) }}</p>
              <p v-if="plugin.enabled === true">各条 Hook 仍以自身启用与信任状态为准。</p>
              <p v-if="plugin.hookTrust.total">
                {{ plugin.hookTrust.trusted + plugin.hookTrust.managed }}/{{
                  plugin.hookTrust.total
                }}
                已批准
                <span v-if="plugin.hookTrust.managed">
                  · {{ plugin.hookTrust.managed }} 项由策略信任</span
                >
                <span v-if="plugin.hookTrust.untrusted">
                  · {{ plugin.hookTrust.untrusted }} 项待信任</span
                >
                <span v-if="plugin.hookTrust.modified">
                  · {{ plugin.hookTrust.modified }} 项定义变化</span
                >
                <span v-if="plugin.hookTrust.disabled">
                  · {{ plugin.hookTrust.disabled }} 项已单独停用</span
                >
              </p>
              <p v-if="plugin.hookTrust.reason">{{ plugin.hookTrust.reason }}</p>
              <p v-if="plugin.hookTrust.state === 'reviewRequired'">
                请新建 Desktop 对话，按宿主提示审阅当前定义并决定是否信任。已信任且定义未变化的 Hook
                不需要重复信任。
              </p>
            </template>
          </div>
        </div>
        <HookList
          :plugin-name="plugin.name"
          :plugin-id="plugin.pluginId"
          :plugin-enabled="plugin.enabled"
          :hooks="plugin.hookTrust.hooks"
        />
        <dl class="behavior-meta">
          <dt>固定版本</dt>
          <dd class="mono">{{ plugin.tag }}</dd>
          <dt>已安装版本</dt>
          <dd class="mono">{{ plugin.installedVersion ?? '—' }}</dd>
          <dt>开源许可证</dt>
          <dd>{{ plugin.license }}</dd>
          <dt>来源</dt>
          <dd>
            <a
              :href="`https://github.com/${plugin.repository}/tree/${plugin.tag}`"
              target="_blank"
              rel="noopener noreferrer"
              >{{ plugin.repository }} ↗</a
            >
          </dd>
        </dl>
        <div v-if="plugin.duplicates" role="alert" class="plugin-duplicate">
          发现同名插件来自两个市场。请在 Codex 中保留一个来源，再刷新。
        </div>
      </Panel>
    </div>
    <Modal
      v-if="installing"
      :title="`安装 ${installing.name}`"
      @close="!busy && (installing = null)"
      ><p class="eyebrow">INSTALL / {{ installing.tag }}</p>
      <h2>把它加入你的工作间。</h2>
      <p class="muted">
        将从官方 GitHub 仓库下载固定版本，并通过 Desktop 内置引擎安装到用户级插件中。需要网络、Git
        与受支持的 Codex Desktop，不要求另外安装 CLI。
      </p>
      <div class="inset install-preview">
        <b>{{ installing.repository }}</b
        ><code>{{ installing.tag }} → {{ installing.id }}@atelier-companions</code>
        <p>安装完成后默认停用。启用加载后，请在新的 Codex 会话中按宿主提示审阅 Hook。</p>
      </div>
      <p v-if="error" role="alert" class="error-state">{{ error }}</p>
      <div class="modal-actions">
        <button class="button" :disabled="!!busy" @click="installing = null">取消</button
        ><button class="button primary" :disabled="!!busy" @click="mutate(installing, 'install')">
          {{ busy ? '下载与安装中…' : '安装并保持停用' }}
        </button>
      </div></Modal
    >
  </div>
</template>
