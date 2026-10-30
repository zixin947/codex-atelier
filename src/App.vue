<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, nextTick } from 'vue'
import { isTauri } from '@tauri-apps/api/core'
import { useAccountRefresh } from './app/accountRefresh'
import { useRoute } from 'vue-router'
import { useAtelier } from './app/store'
import Icon from './shared/components/Icon.vue'
import Modal from './shared/components/Modal.vue'
import ThemePicker from './shared/components/ThemePicker.vue'
import ThemeSwitcher from './shared/components/ThemeSwitcher.vue'
import EndfieldBackground from './shared/background/EndfieldBackground.vue'
import { theme } from './app/theme'
import {
  startupAnimationActive,
  finishStartupInitialization,
  previewStartupAnimation,
} from './app/startup'
const workstation = ref<HTMLElement | null>(null)
const route = useRoute()
const realPlugins = computed(() => route.path === '/plugins')
const realManagement = computed(
  () => ['/plugins', '/connections'].includes(route.path) || route.path.startsWith('/accounts'),
)
const store = useAtelier(),
  settings = ref(false)
const accountRefresh = useAccountRefresh()
function syncAccountRefresh() {
  void accountRefresh.sync()
}
function skip() {
  document.getElementById('main-content')?.focus()
}
const nav = [
  { path: '/', label: '主工作台', icon: 'grid' },
  { path: '/accounts', label: '账号与额度', icon: 'user' },
  { path: '/connections', label: '模型与连接', icon: 'chip' },
  { path: '/extensions', label: '扩展管理', icon: 'box' },
  { path: '/plugins', label: '行为插件', icon: 'shield' },
  { path: '/planned/history', label: '会话与历史', icon: 'history' },
  { path: '/planned/companions', label: '人设与陪伴', icon: 'heart' },
  { path: '/diagnostics', label: '诊断与回滚', icon: 'pulse' },
]
onMounted(async () => {
  if (isTauri()) {
    void accountRefresh.initialize()
    window.addEventListener('focus', syncAccountRefresh)
  }
  try {
    await store.load()
  } finally {
    finishStartupInitialization()
  }
})
onUnmounted(() => window.removeEventListener('focus', syncAccountRefresh))
async function previewStartup() {
  settings.value = false
  await nextTick()
  previewStartupAnimation()
}
</script>
<template>
  <a class="skip-link" href="#main-content" @click.prevent="skip">跳转到主内容</a>
  <div ref="workstation" class="workstation">
    <EndfieldBackground
      v-if="theme === 'endfield' && workstation && !startupAnimationActive"
      :host="workstation"
    />
    <header class="topbar">
      <a class="brand" href="#/" aria-label="Codex Atelier 首页"
        ><span class="brand-mark"><i /><i /><i /></span
        ><span>CODEX <b>ATELIER</b><small>YOUR PERSONAL CONTROL ROOM</small></span></a
      >
      <div v-if="!realManagement" class="top-context">
        <span class="tiny">工作配置</span
        ><select
          v-model="store.profile"
          aria-label="工作配置"
          @change="store.notify('已切换示例工作配置')"
        >
          <option>日常开发</option>
          <option>专注创作</option>
          <option>沙盒实验</option>
        </select>
      </div>
      <div v-else class="top-context mono tiny">
        {{ realPlugins ? 'CODEX / USER PLUGINS' : 'ATELIER / LOCAL GATEWAY' }}
      </div>
      <div class="top-actions">
        <span class="badge orange"
          ><i class="dot" />{{
            realManagement ? (realPlugins ? '本机插件管理' : '本地账号与网关') : '演示数据'
          }}</span
        >
        <ThemeSwitcher
          @change="
            (label, saved) => store.notify('已切换 ' + label + (saved ? '' : ' · 当前无法保存偏好'))
          "
        />
        <button class="icon-button" aria-label="应用设置" @click="settings = true">
          <Icon name="settings" />
        </button>
      </div>
    </header>
    <div class="shell-body">
      <aside class="sidebar">
        <div class="sidebar-label">工作空间 <span>WORKSPACE</span></div>
        <nav aria-label="主导航">
          <RouterLink
            v-for="(item, i) in nav"
            :key="item.path"
            :to="item.path"
            :class="{
              active:
                $route.path === item.path ||
                (item.path === '/accounts' && $route.path.startsWith('/accounts/')),
            }"
            ><Icon :name="item.icon" /><span>{{ item.label }}</span
            ><small>0{{ i + 1 }}</small></RouterLink
          >
        </nav>
        <div class="sidebar-bottom">
          <div class="vent" />
          <div class="sandbox-stamp">
            <Icon name="shield" /><b>{{ realManagement ? '受限本机管理' : '安全沙盒' }}</b>
            <p v-if="realPlugins">仅管理两个指定插件<br />账号与代理不受影响</p>
            <p v-else-if="realManagement">独立账号 · 手动连接<br />网关默认关闭</p>
            <p v-else>所有操作仅作用于演示数据<br />未连接本机 Codex</p>
          </div>
          <span class="tiny mono">ATELIER / BUILD 0.1.0</span>
        </div>
      </aside>
      <main id="main-content" tabindex="-1">
        <div v-if="store.error && !realManagement" class="error-state" role="alert">
          {{ store.error }} <button @click="store.load">重新加载</button>
        </div>
        <RouterView v-else />
      </main>
    </div>
    <footer class="statusbar">
      <span
        ><i class="dot" /> {{ realManagement ? 'LOCAL ADAPTER' : 'DEMO ADAPTER' }}
        <b>{{
          realManagement ? '本地管理 · 运行状态见当前页面' : '本地演示 · 未连接 Codex'
        }}</b></span
      ><span
        >{{ accountRefresh.status?.intervalMs ? '额度后台刷新已启用' : '无独立后台服务' }}
        <span>/</span> 无遥测 <span>/</span> v0.1.0</span
      >
    </footer>
  </div>
  <div v-if="store.notice" class="toast" role="status">
    <Icon name="check" />{{ store.notice
    }}<button class="icon-button" aria-label="关闭通知" @click="store.notice = ''">×</button>
  </div>
  <Modal v-if="settings" title="工作站设置 / PREFERENCES" @close="settings = false"
    ><p class="eyebrow">APPEARANCE / PREFERENCES</p>
    <h2>让工作台，有你的风格。</h2>
    <p class="muted">选择你喜欢的界面风格，立即应用到整个工作台。</p>
    <ThemePicker
      @change="(label) => store.notify('已切换 ' + label)"
      @preview-startup="previewStartup"
    />
    <div class="setting-row">
      <span
        ><b>演示交互</b
        ><small>仅控制示例 MCP、Skills 与模拟确认；行为插件、账号与网关独立管理。</small></span
      ><button
        role="switch"
        :aria-checked="store.demo"
        class="switch"
        :class="{ on: store.demo }"
        aria-label="演示交互"
        @click="store.demo = !store.demo"
      >
        <span />
      </button>
    </div>
    <div class="inset safety-note">
      示例状态仅保留在本次运行中。行为插件页独立使用 Codex
      官方接口管理两个插件的启用字段。账号与网关只在对应页面由你手动配置，不自动接管现有 Codex。
    </div>
    <button class="button primary" @click="settings = false">完成</button></Modal
  >
</template>
