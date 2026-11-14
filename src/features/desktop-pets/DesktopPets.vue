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
            ? '桌宠已停止。'
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
      <header class="pet-heading">
        <div class="pet-title">
          <h2>GPT小龙娘</h2>
          <span class="pet-version">v{{ state?.version || '0.3.0' }}</span>
        </div>
        <div class="pet-status">
          <span class="badge" :class="{ neutral: !state?.running }">{{
            !native
              ? '请在桌面应用中管理'
              : !state
                ? '正在读取状态'
                : busy === 'stop'
                  ? '正在关闭'
                  : state.running
                    ? '运行中'
                    : state.installed
                      ? '已停止'
                      : '未安装'
          }}</span>
          <button
            class="pet-power"
            type="button"
            role="switch"
            aria-label="桌宠运行"
            :aria-checked="!!state?.running"
            :aria-busy="busy === 'open' || busy === 'stop'"
            :title="state?.running ? '关闭桌宠' : '开启桌宠'"
            :disabled="!native || !!busy || !state?.installed"
            @click="run(state?.running ? 'stop' : 'open')"
          >
            <span class="pet-power-track" aria-hidden="true">
              <span class="pet-power-circuit" />
              <span class="pet-power-thumb" />
            </span>
          </button>
        </div>
      </header>
      <div class="pet-body">
        <div class="pet-portrait"><img src="/desktop-pets/gpt-dragon.png" alt="GPT小龙娘" /></div>
        <div class="pet-controls">
          <p class="pet-section-label">启动方式</p>
          <div class="pet-launch-actions">
            <button
              class="button primary"
              :disabled="!native || !!busy || !state?.installed"
              @click="run('desktop')"
            >
              <Icon name="grid" :size="20" />独立桌面
            </button>
            <button
              class="button"
              :disabled="!native || !!busy || !state?.installed"
              @click="run('follow')"
            >
              <Icon name="plug" :size="20" />跟随 Codex
            </button>
          </div>
          <button
            class="button pet-show"
            :disabled="!native || !!busy || !state?.installed"
            @click="run('open')"
          >
            显示桌宠<Icon name="arrow" :size="15" />
          </button>
        </div>
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
      <footer class="pet-footer">
        <div class="pet-feedback">
          <p v-if="error" class="error-state pet-message" role="alert">{{ error }}</p>
          <p v-else-if="busy" class="pet-message muted" role="status">
            {{
              busy === 'install' ? progress : busy === 'stop' ? '正在等待桌宠退出…' : '正在处理…'
            }}
          </p>
          <p v-else-if="notice" class="pet-message muted" role="status">{{ notice }}</p>
        </div>
        <button
          class="icon-button"
          aria-label="刷新状态与额度"
          title="刷新状态与额度"
          :disabled="!native || !!busy"
          @click="run('refresh-quota')"
        >
          <Icon name="refresh" :size="16" />
        </button>
        <button
          class="pet-install-toggle"
          :disabled="!native || !!busy"
          :aria-expanded="installOpen"
          @click="installOpen = !installOpen"
        >
          {{ state?.installed ? '重新安装' : '安装桌宠' }}
        </button>
      </footer>
    </section>
  </div>
</template>
<style scoped>
.pet-card {
  max-width: 780px;
  overflow: hidden;
  padding: 0;
}
.pet-heading,
.pet-title,
.pet-status,
.pet-footer {
  display: flex;
  align-items: center;
}
.pet-heading {
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-5) var(--space-6);
  border-bottom: 1px solid var(--hairline);
}
.pet-title {
  flex-wrap: wrap;
  gap: var(--space-3);
  padding-left: var(--space-3);
  border-left: 3px solid var(--signal);
}
.pet-title h2 {
  margin: 0;
  font-size: 21px;
  letter-spacing: -0.4px;
}
.pet-version {
  color: var(--muted);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
}
.pet-status {
  gap: var(--space-3);
}
.pet-status .badge {
  white-space: nowrap;
}
.pet-body {
  display: grid;
  grid-template-columns: 180px minmax(0, 1fr);
  align-items: center;
  gap: var(--space-8);
  padding: var(--space-6);
}
.pet-portrait {
  height: 184px;
  overflow: hidden;
  border: 1px solid var(--hairline);
  border-radius: var(--radius-control);
  background: color-mix(in srgb, var(--signal) 5%, var(--surface));
}
.pet-portrait img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}
.pet-controls {
  min-width: 0;
}
.pet-section-label {
  margin: 0 0 var(--space-3);
  color: var(--muted);
  font-size: var(--text-sm);
}
.pet-launch-actions {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
}
.pet-launch-actions .button {
  height: 68px;
  gap: var(--space-3);
  justify-content: flex-start;
  padding: var(--space-4);
}
.pet-show {
  justify-content: flex-start;
  gap: var(--space-2);
  margin-top: var(--space-4);
  padding: 0;
  border: 0;
  background: transparent;
  box-shadow: none;
}
.pet-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: var(--space-3);
}
.pet-footer {
  gap: var(--space-3);
  padding: var(--space-3) var(--space-6);
  border-top: 1px solid var(--hairline);
  background: color-mix(in srgb, var(--surface) 70%, var(--canvas));
}
.pet-feedback {
  flex: 1;
  min-width: 0;
}
.pet-power {
  --pet-power-blue: #239be9;
  display: inline-grid;
  place-items: center;
  width: 84px;
  height: 44px;
  padding: 2px;
  border: 0;
  border-radius: 999px;
  background: transparent;
  cursor: pointer;
}
.pet-power-track {
  position: relative;
  display: block;
  width: 76px;
  height: 38px;
  border-radius: inherit;
  background: linear-gradient(160deg, #cecfcb, #f5f5f0);
  box-shadow:
    inset 2px 3px 5px #00000026,
    inset -1px -2px 3px #ffffffdc,
    0 1px 1px #ffffffb3;
}
.pet-power-circuit {
  position: absolute;
  top: 10px;
  left: 10px;
  width: 56px;
  height: 18px;
  color: #9b9e9b;
  background: linear-gradient(currentColor, currentColor) center / 100% 3px no-repeat;
}
.pet-power-circuit::before,
.pet-power-circuit::after {
  content: '';
  position: absolute;
  top: 0;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: currentColor;
  box-shadow: inset 0 2px 3px #00000024;
}
.pet-power-circuit::before {
  left: 0;
}
.pet-power-circuit::after {
  right: 0;
}
.pet-power-thumb {
  position: absolute;
  inset: 2px auto auto 2px;
  width: 34px;
  height: 34px;
  border-radius: 50%;
  background: linear-gradient(145deg, #fffffc, #dadcd8);
  border: 1px solid #ffffff80;
  box-shadow:
    2px 3px 5px #00000038,
    inset 0 1px 2px #ffffffed,
    inset 0 -2px 3px #0000000d;
  transition: transform var(--motion) var(--ease);
}
.pet-power[aria-checked='true'] .pet-power-circuit {
  color: var(--pet-power-blue);
}
.pet-power[aria-checked='true'] .pet-power-thumb {
  transform: translateX(38px);
}
.pet-power:focus-visible {
  outline: 2px solid var(--ink);
  outline-offset: 3px;
}
.pet-power:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
.pet-power:active:not(:disabled) .pet-power-thumb {
  box-shadow:
    1px 1px 3px #00000038,
    inset 0 2px 4px #00000014;
}
@media (prefers-reduced-motion: reduce) {
  .pet-power-thumb {
    transition: none;
  }
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
  margin: 0;
  font-size: 13px;
}
.pet-install {
  border-top: 1px solid var(--hairline);
  padding: var(--space-4) var(--space-6);
}
.pet-install p {
  margin: 0;
}
@media (max-width: 1100px) {
  .pet-body {
    grid-template-columns: 140px minmax(0, 1fr);
    gap: var(--space-5);
  }
  .pet-portrait {
    height: 160px;
  }
  .pet-launch-actions .button {
    gap: var(--space-2);
    padding: var(--space-3);
  }
}
@media (max-width: 700px) {
  .pet-heading {
    flex-wrap: wrap;
  }
  .pet-status {
    margin-left: auto;
  }
  .pet-body {
    grid-template-columns: 100px minmax(0, 1fr);
    gap: var(--space-4);
  }
  .pet-portrait {
    height: 140px;
  }
  .pet-launch-actions {
    grid-template-columns: 1fr;
    gap: var(--space-2);
  }
  .pet-launch-actions .button {
    height: 40px;
  }
}
</style>
