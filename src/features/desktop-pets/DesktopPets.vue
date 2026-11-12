<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke, isTauri } from '@tauri-apps/api/core'
import Icon from '../../shared/components/Icon.vue'
const native = isTauri()
const state = ref<{ version: string; installed: boolean; running: boolean } | null>(null)
const busy = ref('')
const error = ref('')
const notice = ref('')
const installOpen = ref(false)
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
      installOpen.value = false
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
      </div>
    </div>
    <section class="panel pet-card" aria-label="GPT小龙娘控制">
      <div class="pet-portrait"><img src="/desktop-pets/gpt-dragon.png" alt="GPT小龙娘" /></div>
      <div class="pet-controls">
        <div class="pet-heading">
          <h2>
            GPT小龙娘 <small>v{{ state?.version || '0.3.0' }}</small>
          </h2>
          <button
            class="icon-button"
            aria-label="刷新状态与额度"
            title="刷新状态与额度"
            :disabled="!native || !!busy"
            @click="run('refresh-quota')"
          >
            <Icon name="refresh" :size="17" />
          </button>
        </div>
        <span class="badge" :class="{ neutral: !state?.running }">{{
          !native
            ? '请在桌面应用中管理'
            : !state
              ? '正在读取状态'
              : state.running
                ? '运行中'
                : state.installed
                  ? '已停止'
                  : '未安装'
        }}</span>
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
            显示桌宠
          </button>
          <button
            class="button"
            :disabled="!native || !!busy || !state?.running"
            @click="run('stop')"
          >
            停止
          </button>
        </div>
        <button
          class="pet-install-toggle"
          :disabled="!native || !!busy"
          :aria-expanded="installOpen"
          @click="installOpen = !installOpen"
        >
          {{ state?.installed ? '重新安装' : '安装桌宠' }}
        </button>
      </div>
      <div v-if="installOpen" class="pet-install">
        <p>将下载组件并启用后台服务，与小鲸鱼共用数据。</p>
        <div class="pet-actions">
          <button class="button primary" :disabled="!!busy" @click="run('install')">
            确认安装
          </button>
          <button class="button" :disabled="!!busy" @click="installOpen = false">取消</button>
        </div>
      </div>
      <p v-if="error" class="error-state pet-message" role="alert">{{ error }}</p>
      <p v-else-if="busy" class="pet-message muted" role="status">
        {{ busy === 'install' ? progress : '正在处理…' }}
      </p>
      <p v-else-if="notice" class="pet-message muted" role="status">{{ notice }}</p>
    </section>
  </div>
</template>
<style scoped>
.pet-card {
  max-width: 760px;
  display: grid;
  grid-template-columns: 140px minmax(0, 1fr);
  gap: 24px;
  padding: 24px;
  align-items: center;
}
.pet-portrait {
  height: 170px;
  overflow: hidden;
  background: color-mix(in srgb, var(--accent-text) 6%, var(--surface));
}
.pet-portrait img {
  width: 100%;
  height: 170px;
  object-fit: contain;
}
.pet-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 12px;
}
.pet-heading h2 {
  margin: 0;
  font-size: 21px;
}
.pet-heading small {
  font-size: 11px;
  font-weight: 400;
  color: var(--muted);
}
.pet-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin: 18px 0 10px;
}
.pet-install-toggle {
  padding: 0;
  border: 0;
  background: none;
  color: var(--muted);
  font-size: 12px;
  text-decoration: underline;
  text-underline-offset: 3px;
}
.pet-install,
.pet-message {
  grid-column: 1 / -1;
  margin: 0;
  font-size: 13px;
}
.pet-install {
  border-top: 1px solid var(--line);
  padding-top: 16px;
}
.pet-install p {
  margin: 0;
}
@media (max-width: 1100px) {
  .pet-card {
    grid-template-columns: 100px minmax(0, 1fr);
    gap: 18px;
    padding: 20px;
  }
  .pet-portrait,
  .pet-portrait img {
    height: 150px;
  }
}
</style>
