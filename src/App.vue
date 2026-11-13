<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, nextTick } from 'vue'
import { isTauri } from '@tauri-apps/api/core'
import { useAccountRefresh } from './app/accountRefresh'
import { useRoute, useRouter } from 'vue-router'
import { useAtelier } from './app/store'
import Icon from './shared/components/Icon.vue'
import Modal from './shared/components/Modal.vue'
import ThemePicker from './shared/components/ThemePicker.vue'
import ThemeSwitcher from './shared/components/ThemeSwitcher.vue'
import HudNavigation from './shared/hud/HudNavigation.vue'
import { useNativeHud } from './shared/hud/native'
import EndfieldBackground from './shared/background/EndfieldBackground.vue'
import { theme } from './app/theme'
import { nav } from './app/navigation'
import {
  startupAnimationActive,
  finishStartupInitialization,
  previewStartupAnimation,
} from './app/startup'
const workstation = ref<HTMLElement | null>(null)
const hudSurface = ref<HTMLElement | null>(null)
const hudExpanded = ref(false)
const themeMenuOpen = ref(false)
const route = useRoute()
const router = useRouter()
const hudOverview = computed(
  () => theme.value === 'endfield' && route.path === '/' && !hudExpanded.value,
)
const workbenchActive = computed(() => route.path === '/' || route.path.startsWith('/workbench/'))
const workbenchNav = [
  { path: '/', label: '概览' },
  { path: '/workbench/requests', label: '请求' },
  { path: '/workbench/sessions', label: '会话' },
  { path: '/workbench/context', label: '上下文' },
]
/* 真实状态标签：只描述当前页真正在做什么，不再有演示/适配器之类的开发期字眼。 */
const pageContext = computed<{ code: string; badge: string }>(() => {
  if (hudOverview.value) return { code: 'ATELIER / ORBITAL HUD', badge: '环形概览' }
  if (workbenchActive.value) return { code: 'ATELIER / CHATGPT USAGE', badge: 'ChatGPT 用量' }
  const path = route.path
  if (path === '/plugins') return { code: 'CODEX / USER PLUGINS', badge: '本机插件' }
  if (path === '/desktop-pets') return { code: 'ATELIER / DESKTOP PETS', badge: '本机桌宠' }
  if (path === '/threads') return { code: 'ATELIER / THREAD ARCHIVE', badge: '本机线程' }
  if (path === '/maintenance') return { code: 'ATELIER / MAINTENANCE', badge: '维护工具' }
  if (path === '/routing') return { code: 'ATELIER / LIVE ROUTING', badge: '路由观测' }
  if (path === '/routing-groups') return { code: 'ATELIER / ROUTE GROUPS', badge: '路由组' }
  if (path === '/providers') return { code: 'ATELIER / UPSTREAM PROVIDERS', badge: '供应商' }
  if (path === '/gateway') return { code: 'ATELIER / LOCAL GATEWAY', badge: '本地网关' }
  if (path.startsWith('/accounts')) return { code: 'ATELIER / ACCOUNTS', badge: '账号与额度' }
  return { code: 'ATELIER / MODELS & CONNECTIONS', badge: '模型与连接' }
})
const store = useAtelier(),
  settings = ref(false)
const accountRefresh = useAccountRefresh()
const navClicks = ref<Record<string, number>>({})
function syncAccountRefresh() {
  void accountRefresh.sync()
}
function skip() {
  document.getElementById('main-content')?.focus()
}
const nativeHud = useNativeHud(
  hudSurface,
  () =>
    hudOverview.value && !settings.value && !themeMenuOpen.value && !startupAnimationActive.value,
  () => nav.map((item) => item.label),
  (index) => {
    hudExpanded.value = true
    void router.push(nav[index]!.path)
  },
)
onMounted(() => {
  if (isTauri()) {
    void accountRefresh.initialize()
    window.addEventListener('focus', syncAccountRefresh)
  }
  finishStartupInitialization()
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
  <div
    ref="workstation"
    class="workstation"
    :class="{
      'hud-home': hudOverview,
      'hud-native-ready': nativeHud.ready.value,
    }"
  >
    <EndfieldBackground
      v-if="theme === 'endfield' && workstation && !startupAnimationActive"
      :host="workstation"
    />
    <header class="topbar">
      <button
        v-if="theme === 'endfield' && !hudOverview"
        class="ring-back"
        aria-label="返回环形首页"
        title="返回环形首页"
        @click="hudExpanded = false; router.push('/')"
      ><span aria-hidden="true">←</span><span>环形首页</span></button>
      <a class="brand" href="#/" aria-label="Codex Atelier 首页" @click="hudExpanded = false"
        ><span class="brand-mark"><i /><i /><i /></span
        ><span>CODEX <b>ATELIER</b><small>YOUR PERSONAL CONTROL ROOM</small></span></a
      >
      <div class="top-context mono tiny">{{ pageContext.code }}</div>
      <div class="top-actions">
        <span class="badge orange"><i class="dot" />{{ pageContext.badge }}</span>
        <ThemeSwitcher
          @open-change="themeMenuOpen = $event"
          @change="
            (label, saved) => store.notify('已切换 ' + label + (saved ? '' : ' · 当前无法保存偏好'))
          "
        />
        <button class="icon-button" aria-label="应用设置" @click="settings = true">
          <Icon name="settings" />
        </button>
      </div>
    </header>
    <div ref="hudSurface" class="shell-body">
      <p v-if="theme === 'endfield' && nativeHud.error.value" class="hud-native-error" role="alert">
        原生 HUD 加载失败：{{ nativeHud.error.value }}
      </p>
      <HudNavigation
        v-if="hudOverview"
        :items="nav"
        :expanded="route.path !== '/' || hudExpanded"
        @open="hudExpanded = true"
        @overview="hudExpanded = false"
      />
      <aside v-else class="sidebar">
        <div class="sidebar-label">工作空间 <span>WORKSPACE</span></div>
        <nav aria-label="主导航">
          <template v-for="(item, i) in nav" :key="item.path">
            <RouterLink
              :to="item.path"
              @click="navClicks[item.path] = (navClicks[item.path] ?? 0) + 1"
              :class="{
                active:
                  (item.path === '/' && workbenchActive) ||
                  $route.path === item.path ||
                  (item.path === '/accounts' && $route.path.startsWith('/accounts/')),
              }"
              ><span class="nav-plate" aria-hidden="true" />
              <span
                v-if="navClicks[item.path]"
                :key="navClicks[item.path]"
                class="nav-impact"
                aria-hidden="true"
              />
              <Icon :name="item.icon" /><span class="nav-label">{{ item.label }}</span
              ><small>{{ String(i + 1).padStart(2, '0') }}</small></RouterLink
            >
            <div
              v-if="item.path === '/' && workbenchActive"
              class="workbench-subnav"
              aria-label="ChatGPT 用量目录"
            >
              <RouterLink
                v-for="child in workbenchNav"
                :key="child.path"
                :to="child.path"
                :class="{ selected: route.path === child.path }"
                :aria-current="route.path === child.path ? 'page' : undefined"
                >{{ child.label }}</RouterLink
              >
            </div>
          </template>
        </nav>
      </aside>
      <main v-show="!hudOverview" id="main-content" data-hud-panel tabindex="-1">
        <RouterView />
      </main>
    </div>
    <footer class="statusbar" v-if="accountRefresh.status?.intervalMs">
      <span>额度自动刷新已开启</span>
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
    <p class="muted">Codex Atelier · v0.1.0</p>
    <button class="button primary" @click="settings = false">完成</button></Modal
  >
</template>

<style scoped>
.ring-back {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
  height: 32px;
  padding: 0 11px;
  border: 1px solid var(--hairline);
  border-radius: 5px;
  background: var(--surface);
  color: var(--ink);
  font: inherit;
  font-size: 11px;
  cursor: pointer;
  transition: background 140ms, border-color 140ms;
}
.ring-back > span:first-child { font-size: 21px; line-height: 1; }
.ring-back:hover { background: #e9ebd7; border-color: #969d70; }
.ring-back:focus-visible { outline: 2px solid var(--focus); outline-offset: 3px; }
.ring-back:active { background: #dde1bc; }
@media (max-width: 1100px) {
  .ring-back > span:last-child { display: none; }
  .ring-back { width: 32px; padding: 0; justify-content: center; }
}

.workbench-subnav {
  margin: 0 12px 8px 25px;
  padding-left: 10px;
  border-left: 1px solid var(--border);
}
.sidebar nav .workbench-subnav a {
  min-height: 30px;
  padding: 5px 14px;
  font-size: 13px;
  color: var(--muted);
}
.sidebar nav .workbench-subnav a.selected {
  background: var(--surface-alt, #e2e5d9);
  color: var(--text);
  font-weight: 600;
}
</style>
