<script setup lang="ts">
import { ref } from 'vue'
import {
  theme,
  setTheme,
  backgroundOptions,
  updateBackground,
  type ThemeId,
  type BackgroundOptions,
} from '../../app/theme'
import Icon from './Icon.vue'
import { startupAnimationEnabled, setStartupAnimationEnabled } from '../../app/startup'
const emit = defineEmits<{ change: [label: string]; previewStartup: [] }>()
const saved = ref(true)
const choices: { id: ThemeId; name: string; english: string; description: string }[] = [
  {
    id: 'endfield',
    name: '终末地 · 野外终端',
    english: 'FIELD TERMINAL / 01',
    description: '纸灰、墨黑与信号黄，工业制图的秩序。',
  },
  {
    id: 'millennium',
    name: '千禧机壳',
    english: 'MILLENNIUM / 02',
    description: '奶油面板与薄荷绿，保留复古工作站质感。',
  },
]
function select(choice: (typeof choices)[number]) {
  saved.value = setTheme(choice.id)
  emit('change', choice.name)
}
function toggleBackground(key: keyof BackgroundOptions) {
  saved.value = updateBackground({ [key]: !backgroundOptions.value[key] })
}
function toggleStartup() {
  saved.value = setStartupAnimationEnabled(!startupAnimationEnabled.value)
}
</script>
<template>
  <fieldset class="theme-picker">
    <legend>界面皮肤 <span>APPEARANCE</span></legend>
    <div class="theme-choices">
      <button
        v-for="choice in choices"
        :key="choice.id"
        type="button"
        class="theme-choice"
        :class="{ selected: theme === choice.id }"
        :aria-pressed="theme === choice.id"
        @click="select(choice)"
      >
        <span class="theme-swatch" :class="choice.id" aria-hidden="true"
          ><span class="swatch-header" /><span class="swatch-rail"><i /><i /><i /></span
          ><span class="swatch-screen"><i /><i /></span><span class="swatch-panel"
        /></span>
        <span class="theme-choice-heading"
          ><b>{{ choice.name }}</b
          ><Icon v-if="theme === choice.id" name="check" :size="15"
        /></span>
        <span class="theme-choice-code">{{ choice.english }}</span>
        <span class="theme-choice-description">{{ choice.description }}</span>
      </button>
    </div>
    <div v-if="theme === 'endfield'" class="background-options">
      <span class="background-options-heading"
        >终末地背景 <small>上游原版等高线 · 24 FPS</small></span
      >
      <label
        ><input
          type="checkbox"
          :checked="backgroundOptions.enabled"
          @change="toggleBackground('enabled')"
        />完整背景</label
      >
      <label
        ><input
          type="checkbox"
          :checked="backgroundOptions.animated"
          :disabled="!backgroundOptions.enabled"
          @change="toggleBackground('animated')"
        />动态等高线</label
      >
      <label
        ><input
          type="checkbox"
          :checked="backgroundOptions.trail"
          :disabled="!backgroundOptions.enabled || !backgroundOptions.animated"
          @change="toggleBackground('trail')"
        />鼠标扰动</label
      >
      <label
        ><input
          type="checkbox"
          :checked="backgroundOptions.watermark"
          :disabled="!backgroundOptions.enabled"
          @change="toggleBackground('watermark')"
        />ENDFIELD 字标</label
      >
      <p>系统开启“减少动态效果”时，背景自动静止。</p>
    </div>
    <div v-if="theme === 'endfield'" class="startup-options">
      <label
        ><input
          type="checkbox"
          :checked="startupAnimationEnabled"
          @change="toggleStartup"
        />启动加载动画</label
      >
      <button type="button" class="button" @click="emit('previewStartup')">预览开屏</button>
      <p>每次启动播放一次；可按 Esc 跳过。系统开启“减少动态效果”时自动跳过。</p>
    </div>
    <p class="theme-preference-note" role="status">
      {{
        saved
          ? '即时切换，自动记住你的选择。仅改变 Atelier 外观。'
          : '外观已切换，当前环境无法保存偏好。'
      }}
    </p>
  </fieldset>
</template>
<style scoped>
.startup-options {
  display: grid;
  grid-template-columns: 1fr auto;
  align-items: center;
  gap: var(--space-2);
  margin-top: var(--space-3);
  padding: var(--space-3);
  border: 1px solid var(--edge);
  font-size: 11px;
}
.startup-options label {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
}
.startup-options .button {
  min-height: 28px;
  font-size: 11px;
}
.startup-options p {
  grid-column: 1 / -1;
  margin: 0;
  color: var(--muted);
  font-size: 10px;
}
.background-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-3);
  margin-top: var(--space-4);
  padding: var(--space-3);
  border: 1px solid var(--edge);
}
.background-options-heading {
  grid-column: 1 / -1;
  display: flex;
  justify-content: space-between;
  gap: var(--space-2);
  font-size: 11px;
  font-weight: 700;
}
.background-options-heading small {
  font-size: 9px;
  font-weight: 400;
  color: var(--muted);
}
.background-options label {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 11px;
}
.background-options input {
  accent-color: var(--ink);
}
.background-options p {
  grid-column: 1 / -1;
  margin: 0;
  color: var(--muted);
  font-size: 10px;
}
.theme-picker {
  border: 0;
  padding: 0;
  margin: var(--space-5) 0;
  min-width: 0;
}
.theme-picker legend {
  font-size: 12px;
  font-weight: 700;
  margin-bottom: var(--space-3);
}
.theme-picker legend span {
  font: 9px var(--font-mono);
  letter-spacing: 1.5px;
  color: var(--muted);
  margin-left: var(--space-2);
}
.theme-choices {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-3);
}
.theme-choice {
  background: var(--paper);
  border: 1px solid var(--edge);
  padding: var(--space-3);
  color: var(--ink);
  text-align: left;
  min-width: 0;
  transition:
    border-color var(--motion),
    transform var(--motion);
}
.theme-choice:hover {
  border-color: var(--line);
}
.theme-choice:active {
  transform: translateY(1px);
}
.theme-choice.selected {
  border-color: var(--line);
  box-shadow: inset 0 0 0 1px var(--line);
}
.theme-choice-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-1);
  margin-top: var(--space-3);
  font-size: 12px;
}
.theme-choice-code {
  display: block;
  font: 8px var(--font-mono);
  letter-spacing: 1px;
  color: var(--muted);
  margin: 6px 0;
}
.theme-choice-description {
  display: block;
  color: var(--muted);
  font-size: 10px;
  line-height: 1.7;
}
.theme-preference-note {
  margin: var(--space-3) 0 0;
  font-size: 10px;
  color: var(--muted);
}
.theme-swatch {
  --swatch-bg: #e8e8e2;
  --swatch-panel: #f5f5ef;
  --swatch-ink: #171a18;
  --swatch-accent: #fff500;
  display: block;
  position: relative;
  height: 93px;
  overflow: hidden;
  background: var(--swatch-bg);
  border: 1px solid var(--swatch-ink);
}
.theme-swatch.millennium {
  --swatch-bg: #ddd4bf;
  --swatch-panel: #ebe4d2;
  --swatch-ink: #403c37;
  --swatch-accent: #8fd6a0;
}
.swatch-header {
  position: absolute;
  inset: 0 0 auto;
  height: 14px;
  background: var(--swatch-panel);
  border-bottom: 1px solid var(--swatch-ink);
}
.swatch-header::after {
  content: '';
  display: block;
  background: var(--swatch-accent);
  width: 7px;
  height: 7px;
  margin: 3px 6px;
}
.swatch-rail {
  position: absolute;
  inset: 14px auto 0 0;
  width: 25%;
  border-right: 1px solid #171a1826;
  padding: 10px 5px;
}
.swatch-rail i {
  display: block;
  height: 6px;
  background: var(--swatch-ink);
  margin-bottom: 6px;
  opacity: 0.2;
}
.swatch-rail i:first-child {
  background: var(--swatch-accent);
  opacity: 1;
}
.swatch-screen {
  position: absolute;
  top: 24px;
  left: 32%;
  right: 8px;
  height: 30px;
  background: var(--swatch-ink);
  padding: 8px;
}
.swatch-screen i {
  display: block;
  height: 3px;
  background: var(--swatch-accent);
  width: 60%;
  margin-bottom: 4px;
}
.swatch-screen i + i {
  width: 35%;
  opacity: 0.4;
}
.swatch-panel {
  position: absolute;
  top: 61px;
  bottom: 8px;
  left: 32%;
  right: 8px;
  border: 1px solid #171a1833;
  background: var(--swatch-panel);
}
@media (max-width: 520px) {
  .theme-choices {
    grid-template-columns: 1fr;
  }
}
</style>
