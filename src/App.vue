<script setup lang="ts">
import { onMounted, ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useAtelier } from './app/store'
import Icon from './shared/components/Icon.vue'
import Modal from './shared/components/Modal.vue'
const route = useRoute()
const realPlugins = computed(() => route.path === '/plugins')
const store = useAtelier(),
  settings = ref(false)
function skip() {
  document.getElementById('main-content')?.focus()
}
const nav = [
  { path: '/', label: '主工作台', icon: 'grid' },
  { path: '/planned/accounts', label: '账号与额度', icon: 'user' },
  { path: '/planned/models', label: '模型与连接', icon: 'chip' },
  { path: '/extensions', label: '扩展管理', icon: 'box' },
  { path: '/plugins', label: '行为插件', icon: 'shield' },
  { path: '/planned/history', label: '会话与历史', icon: 'history' },
  { path: '/planned/companions', label: '人设与陪伴', icon: 'heart' },
  { path: '/diagnostics', label: '诊断与回滚', icon: 'pulse' },
]
onMounted(() => store.load())
</script>
<template>
  <a class="skip-link" href="#main-content" @click.prevent="skip">跳转到主内容</a>
  <div class="workstation">
    <header class="topbar">
      <a class="brand" href="#/" aria-label="Codex Atelier 首页"
        ><span class="brand-mark"><i /><i /><i /></span
        ><span>CODEX <b>ATELIER</b><small>YOUR PERSONAL CONTROL ROOM</small></span></a
      >
      <div v-if="!realPlugins" class="top-context">
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
      <div v-else class="top-context mono tiny">CODEX / USER PLUGINS</div>
      <div class="top-actions">
        <span class="badge orange"
          ><i class="dot" />{{ realPlugins ? '本机插件管理' : '演示数据' }}</span
        ><button class="icon-button" aria-label="应用设置" @click="settings = true">
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
            :class="{ active: $route.path === item.path }"
            ><Icon :name="item.icon" /><span>{{ item.label }}</span
            ><small>0{{ i + 1 }}</small></RouterLink
          >
        </nav>
        <div class="sidebar-bottom">
          <div class="vent" />
          <div class="sandbox-stamp">
            <Icon name="shield" /><b>{{ realPlugins ? '受限本机管理' : '安全沙盒' }}</b>
            <p v-if="realPlugins">仅管理两个指定插件<br />账号与代理不受影响</p>
            <p v-else>所有操作仅作用于演示数据<br />未连接本机 Codex</p>
          </div>
          <span class="tiny mono">ATELIER / BUILD 0.1.0</span>
        </div>
      </aside>
      <main id="main-content" tabindex="-1">
        <div v-if="store.error && !realPlugins" class="error-state" role="alert">
          {{ store.error }} <button @click="store.load">重新加载</button>
        </div>
        <RouterView v-else />
      </main>
    </div>
    <footer class="statusbar">
      <span
        ><i class="dot" /> {{ realPlugins ? 'LOCAL PLUGIN ADAPTER' : 'DEMO ADAPTER' }}
        <b>{{ realPlugins ? '本机插件管理 · 新会话生效' : '本地演示 · 未连接 Codex' }}</b></span
      ><span>无后台服务 <span>/</span> 无遥测 <span>/</span> v0.1.0</span>
    </footer>
  </div>
  <div v-if="store.notice" class="toast" role="status">
    <Icon name="check" />{{ store.notice
    }}<button class="icon-button" aria-label="关闭通知" @click="store.notice = ''">×</button>
  </div>
  <Modal v-if="settings" title="工作站设置 / PREFERENCES" @close="settings = false"
    ><p class="eyebrow">PROTOTYPE SETTINGS</p>
    <h2>让工作台保持简单。</h2>
    <p class="muted">当前使用「千禧机壳」浅色主题。深色主题、真实连接和配置持久化仍在规划中。</p>
    <div class="setting-row">
      <span
        ><b>演示交互</b><small>仅控制示例 MCP、Skills 与模拟确认；行为插件独立管理。</small></span
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
      官方接口管理两个插件的启用字段，不接管账号、凭据或代理。
    </div>
    <button class="button primary" @click="settings = false">完成</button></Modal
  >
</template>
