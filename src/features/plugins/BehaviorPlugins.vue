<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Panel from '../../shared/components/Panel.vue'
import Modal from '../../shared/components/Modal.vue'
import Icon from '../../shared/components/Icon.vue'
import { useAtelier } from '../../app/store'
import { behaviorPluginService } from '../../services/plugins'
import type {
  BehaviorPlugin,
  BehaviorPluginService,
  PluginSnapshot,
  HookTrust,
} from '../../types/plugins'
const props = defineProps<{ service?: BehaviorPluginService }>()
const adapter = props.service ?? behaviorPluginService
const store = useAtelier(),
  snapshot = ref<PluginSnapshot | null>(null),
  busy = ref(''),
  error = ref(''),
  installing = ref<BehaviorPlugin | null>(null)
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
async function refresh() {
  busy.value = 'refresh'
  error.value = ''
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
  error.value = ''
  const revision = snapshot.value.revision
  try {
    snapshot.value =
      operation === 'install'
        ? await adapter.install(plugin.id, revision)
        : operation === 'rollback'
          ? await adapter.rollback(plugin.id, revision)
          : await adapter.setEnabled(plugin.id, !plugin.enabled, revision)
    installing.value = null
    store.notify(
      operation === 'install'
        ? `${plugin.name} 已安装，默认停用 · Hook 状态见卡片`
        : `${plugin.name} 配置已读回 · 请在新会话中确认效果`,
    )
  } catch (e) {
    error.value = String(e)
    try {
      snapshot.value = await adapter.status()
    } catch {
      /* Keep last known snapshot, with the error visible. */
    }
  } finally {
    busy.value = ''
  }
}
onMounted(refresh)
</script>
<template>
  <div class="page behavior-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / BEHAVIOR PLUGINS</p>
        <h1>少一点折腾，多一点专注<span class="heading-dot">.</span></h1>
        <p class="subtitle">Ponytail 与 Stop That Shit · 本机用户级插件管理</p>
      </div>
      <button class="button" :disabled="!!busy" @click="refresh">
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
          开关控制插件在新会话中的加载。Hook 信任状态由 CLI 只读查询，刷新即可重读；
          当前会话执行效果未验证，项目配置也可能覆盖用户设置。
        </p>
      </div>
      <span class="badge">USER SCOPE</span>
    </div>
    <div v-if="error" role="alert" class="error-state plugin-error">{{ error }}</div>
    <div v-if="snapshot && !snapshot.available" class="plugin-unavailable" role="status">
      <Icon name="terminal" :size="20" /><span>{{ snapshot.reason }}</span>
    </div>
    <div v-if="!snapshot" class="empty" role="status">正在连接本机 Codex CLI…</div>
    <div v-else class="behavior-grid">
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
            ><small>{{
              busy === plugin.id
                ? '正在操作并读回验证…'
                : plugin.installed
                  ? '作用于新会话 · 不切换会话模式'
                  : '先安装，再启用'
            }}</small></span
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
        <div
          class="inset behavior-trust"
          :class="{
            'trust-ok': plugin.hookTrust.state === 'trusted',
            'trust-review': plugin.hookTrust.state === 'reviewRequired',
          }"
          role="status"
          :aria-label="`${plugin.name} Hook 信任状态`"
        >
          <Icon
            :name="
              plugin.hookTrust.state === 'trusted'
                ? 'shield'
                : plugin.hookTrust.state === 'reviewRequired'
                  ? 'alert'
                  : 'terminal'
            "
            :size="17"
          />
          <div>
            <b>Hook 信任 · {{ trustLabel(plugin.hookTrust) }}</b>
            <p v-if="plugin.hookTrust.total">
              {{ plugin.hookTrust.trusted + plugin.hookTrust.managed }}/{{ plugin.hookTrust.total }}
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
              请在 Codex CLI 的 /hooks 中审阅当前定义并决定是否信任。
            </p>
          </div>
        </div>
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
    <Panel v-if="snapshot" title="连接与生效范围" code="CONTROL RECEIPT"
      ><div class="plugin-receipt">
        <div>
          <span class="eyebrow">HOST</span><b>{{ snapshot.hostVersion ?? '未连接桌面后端' }}</b>
        </div>
        <div>
          <span class="eyebrow">TARGET</span><b>{{ snapshot.configTarget }}</b>
        </div>
        <div><span class="eyebrow">SESSION EFFECT</span><b>未观测 · 新会话验证</b></div>
      </div>
      <p class="plugin-footnote">
        Hook 状态按 CLI 用户目录发现结果读取，不包含项目覆盖，也不代表 Codex Desktop 当前会话。
        安装使用 Atelier 独立本地市场，固定上游版本；不自动授予 Hook
        信任、不修改账号或代理、不向现有聊天发消息。开关先检测外部变更，保存原启用值，写入后从 Codex
        再读回。
      </p></Panel
    >
    <Modal
      v-if="installing"
      :title="`安装 ${installing.name}`"
      @close="!busy && (installing = null)"
      ><p class="eyebrow">INSTALL / {{ installing.tag }}</p>
      <h2>把它加入你的工作间。</h2>
      <p class="muted">
        将从官方 GitHub 仓库下载固定版本，并通过 Codex CLI 安装到本机用户级插件中。需要网络、Git 与
        Codex CLI。
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
