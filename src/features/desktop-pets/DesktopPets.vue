<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke, isTauri } from '@tauri-apps/api/core'
import Panel from '../../shared/components/Panel.vue'
import Icon from '../../shared/components/Icon.vue'
const native = isTauri()
const state = ref<{ version: string; installed: boolean; running: boolean } | null>(null)
const busy = ref('')
const error = ref('')
const notice = ref('')
const confirmInstall = ref(false)
const progress = ref('')
async function refresh() {
  state.value = await invoke('desktop_pet_action', { action: 'status' })
}
async function run(action: string) {
  busy.value = action
  error.value = ''
  notice.value = ''
  let unlisten: (() => void) | undefined
  try {
    if (action === 'install') {
      progress.value = '正在准备安装…'
      unlisten = await listen<string>('desktop-pet-install-progress', (event) => {
        progress.value = event.payload
      })
    }
    if (action === 'status' || action === 'refresh-quota') {
      state.value = await invoke('desktop_pet_action', { action })
    } else {
      await invoke('desktop_pet_action', { action })
      confirmInstall.value = false
      await refresh()
      notice.value =
        action === 'install'
          ? '桌宠组件已安装。'
          : action === 'stop'
            ? '已请求停止桌宠。'
            : '已打开桌宠，可从托盘或人物菜单进入设置。'
    }
  } catch (e) {
    error.value = String(e)
  } finally {
    unlisten?.()
    busy.value = ''
  }
}
onMounted(() => {
  if (native) void run('status')
})
</script>
<template>
  <div class="page desktop-pets">
    <div class="page-heading">
      <div>
        <p class="eyebrow">WORKSPACE / DESKTOP PETS</p>
        <h1>桌宠<span class="heading-dot">.</span></h1>
        <p class="subtitle">给工作间添一位小伙伴，独立浮窗，随你安排。</p>
      </div>
      <button class="button" :disabled="!native || !!busy" @click="run('refresh-quota')">
        <Icon name="refresh" :size="16" />刷新状态与额度
      </button>
    </div>
    <p v-if="!native" class="inset">浏览器仅预览界面；请在 Atelier 桌面应用中安装与管理桌宠。</p>
    <p v-if="error" class="error-state" role="alert">{{ error }}</p>
    <p v-if="notice" class="inset" role="status">{{ notice }}</p>
    <div class="pet-layout">
      <Panel title="GPT小龙娘" code="COMPANION / 01">
        <div class="pet-portrait">
          <img src="/desktop-pets/gpt-dragon.png" alt="白发白角的 GPT小龙娘" />
        </div>
        <div class="pet-intro">
          <span class="badge" :class="{ neutral: !state?.running }">{{
            !native
              ? '桌面应用可用'
              : !state
                ? '尚未取得状态'
                : state.running
                  ? '服务运行中'
                  : state.installed
                    ? '已安装 · 服务未运行'
                    : '待安装组件'
          }}</span>
          <h2>GPT小龙娘</h2>
          <p class="muted">余额与用量陪伴挂件，支持中英切换、角色互动、自定义气泡及音效。</p>
          <div class="pet-actions">
            <button
              class="button primary"
              :disabled="!native || !!busy || !state?.installed"
              @click="run('desktop')"
            >
              独立桌面
            </button>
            <button
              class="button"
              :disabled="!native || !!busy || !state?.installed"
              @click="run('follow')"
            >
              跟随 Codex
            </button>
            <button
              class="button"
              :disabled="!native || !!busy || !state?.installed"
              @click="run('open')"
            >
              显示桌宠 / 设置入口
            </button>
            <button
              class="button"
              :disabled="!native || !!busy || !state?.running"
              @click="run('stop')"
            >
              停止桌宠
            </button>
          </div>
          <p class="tiny muted">
            设置入口：显示桌宠后，使用托盘或人物菜单中的「设置」。跟随模式需要打开 Codex。
          </p>
        </div>
      </Panel>
      <div class="pet-details">
        <Panel title="组件与接入" code="LOCAL RUNTIME">
          <dl class="detail-meta">
            <dt>内置版本</dt>
            <dd>v0.3.0</dd>
            <dt>适用系统</dt>
            <dd>macOS</dd>
            <dt>运行依赖</dt>
            <dd>Node.js 24+、npm、Xcode Command Line Tools</dd>
            <dt>界面</dt>
            <dd>独立 Electron 透明浮窗</dd>
          </dl>
          <p class="muted">
            首次安装需下载 Electron 运行时，可能需要几分钟。安装会创建上游的 macOS
            自动跟随后台服务；停止按钮暂停桌宠，保留设置与账本。
          </p>
          <p class="inset">
            兼容提醒：与旧小鲸鱼共用 <code>api-balance-whale</code> 标识和
            <code>whale-widget</code>
            数据目录。安装将更新同一套组件，上游安装器会创建备份；两者不能并行作为两个独立插件使用。
          </p>
          <label class="pet-confirm"
            ><input
              v-model="confirmInstall"
              type="checkbox"
              :disabled="!native || !!busy"
            />我了解安装会更新共用组件并启用后台服务</label
          >
          <button
            class="button"
            :disabled="!native || !!busy || !confirmInstall"
            @click="run('install')"
          >
            {{
              busy === 'install'
                ? '正在安装组件…'
                : state?.installed
                  ? '重新安装组件'
                  : '安装桌宠组件'
            }}
          </button>
          <p v-if="busy" role="status" class="tiny">
            {{ busy === 'install' ? progress : '正在处理…' }}
          </p>
        </Panel>
        <Panel title="来源与数据" code="OPEN SOURCE">
          <p class="muted">
            订阅额度复用 Atelier
            当前账号的识别与查询结果；点击顶部按钮同步刷新。用量来自本机事件，API
            余额仍按桌宠配置查询。
          </p>
          <p class="muted">
            内置上游运行时代码与素材，保留 MIT 许可及原作者署名。Atelier 仅提供运行管理。
          </p>
          <a href="https://github.com/Yang-huai406/GPT-Dragon-Girl" target="_blank" rel="noreferrer"
            >项目仓库与安装说明 ↗</a
          >
        </Panel>
      </div>
    </div>
  </div>
</template>
<style scoped>
.pet-layout {
  display: grid;
  grid-template-columns: minmax(0, 1.15fr) minmax(320px, 1fr);
  gap: 20px;
  align-items: start;
}
.pet-portrait {
  height: 290px;
  display: grid;
  place-items: center;
  background: linear-gradient(
    135deg,
    var(--surface),
    color-mix(in srgb, var(--accent-text) 9%, transparent)
  );
  overflow: hidden;
}
.pet-portrait img {
  height: 290px;
  width: 100%;
  object-fit: contain;
}
.pet-intro {
  padding-top: 20px;
}
.pet-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin: 20px 0;
}
.pet-details {
  display: grid;
  gap: 20px;
}
.pet-confirm {
  display: flex;
  gap: 8px;
  align-items: start;
  margin: 18px 0;
  font-size: 13px;
}
.pet-details .inset {
  line-height: 1.7;
}
@media (max-width: 1100px) {
  .pet-layout {
    grid-template-columns: 1fr;
  }
  .pet-portrait,
  .pet-portrait img {
    height: 240px;
  }
}
</style>
