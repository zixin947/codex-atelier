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
    description: '环形首页导航，搭配纸灰、墨黑与信号黄的功能页面。',
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
        >工作台背景 <small>原创测绘网格 · 低速重绘</small></span
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
        />背景动画</label
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
        />背景标记</label
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
.startup-options,
.background-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-2) var(--space-3);
  margin-top: var(--space-3);
  padding: var(--space-3);
  border: 1px solid var(--hairline);
  border-radius: var(--radius-control);
  background: var(--inset);
}
.startup-options {
  grid-template-columns: 1fr auto;
  align-items: center;
}
.background-options-heading {
  grid-column: 1 / -1;
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  gap: var(--space-2);
  padding-bottom: var(--space-2);
  border-bottom: 1px solid var(--hairline);
  font-size: 11px;
  font-weight: 700;
  color: var(--ink);
}
.background-options-heading small {
  font-size: 9px;
  font-weight: 400;
  letter-spacing: 0.04em;
  color: var(--faint);
}
.startup-options label,
.background-options label {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  font-size: 11px;
  color: var(--ink);
  cursor: pointer;
}
.background-options input,
.startup-options input {
  width: 14px;
  height: 14px;
  margin: 0;
  accent-color: var(--ink);
}
.startup-options .button {
  min-height: 28px;
  padding: 0 10px;
  font-size: 11px;
}
.startup-options p,
.background-options p {
  grid-column: 1 / -1;
  margin: 0;
  font-size: 10px;
  line-height: 1.6;
  color: var(--muted);
}

.theme-picker {
  min-width: 0;
  margin: var(--space-4) 0;
  padding: 0;
  border: 0;
}
.theme-picker legend {
  margin-bottom: var(--space-2);
  font-size: 11px;
  font-weight: 700;
  color: var(--ink);
}
.theme-picker legend span {
  margin-left: var(--space-2);
  font: 9px var(--font-mono);
  letter-spacing: 0.12em;
  color: var(--faint);
}
.theme-choices {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-2);
}
.theme-choice {
  min-width: 0;
  padding: var(--space-2);
  text-align: left;
  color: var(--ink);
  background: var(--elevated);
  border: 1px solid var(--control-line);
  border-radius: var(--radius-control);
  transition:
    border-color var(--motion-fast) var(--ease),
    background-color var(--motion-fast) var(--ease);
}
.theme-choice:hover {
  background: var(--inset);
}
.theme-choice.selected {
  border-color: var(--ink);
  box-shadow: inset 3px 0 0 var(--signal);
}
.theme-choice-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-1);
  margin-top: var(--space-2);
  font-size: 12px;
  font-weight: 700;
}
.theme-choice.selected .theme-choice-heading svg {
  color: var(--accent-text);
}
.theme-choice-code {
  display: block;
  margin: 3px 0 6px;
  font: 8px var(--font-mono);
  letter-spacing: 0.1em;
  color: var(--faint);
}
.theme-choice-description {
  display: block;
  font-size: 10px;
  line-height: 1.6;
  color: var(--muted);
}
.theme-preference-note {
  margin: var(--space-3) 0 0;
  font-size: 10px;
  color: var(--muted);
}

/* Swatches show the same structure in both themes: header, rail, one dark
   instrument, one reading panel. They are illustrative, not live previews. */
.theme-swatch {
  --swatch-bg: #eceee6;
  --swatch-panel: #f7f8f2;
  --swatch-ink: #20231f;
  --swatch-accent: #fffa00;
  --swatch-screen: #171c17;
  --swatch-line: #d0d4ca;
  display: block;
  position: relative;
  height: 84px;
  overflow: hidden;
  background: var(--swatch-bg);
  border: 1px solid var(--swatch-line);
  border-radius: 2px;
}
.theme-swatch.millennium {
  --swatch-bg: #ddd4bf;
  --swatch-panel: #ebe4d2;
  --swatch-ink: #393b34;
  --swatch-accent: #8fd6a0;
  --swatch-screen: #363d37;
  --swatch-line: #b9ae96;
}
.theme-swatch.hud {
  --swatch-bg: #202222;
  --swatch-panel: #d3d4cf;
  border-color: #6c6d68;
}
.theme-swatch.hud::after {
  content: '';
  position: absolute;
  inset: 4px 18%;
  border: 1px solid #fffa00;
  border-radius: 50%;
}
.swatch-header {
  position: absolute;
  inset: 0 0 auto;
  height: 13px;
  background: var(--swatch-panel);
  border-bottom: 1px solid var(--swatch-line);
}
.swatch-header::after {
  content: '';
  display: block;
  width: 6px;
  height: 6px;
  margin: 3px 5px;
  background: var(--swatch-ink);
}
.swatch-rail {
  position: absolute;
  inset: 13px auto 0 0;
  width: 22%;
  padding: 6px 5px;
  border-right: 1px solid var(--swatch-line);
}
.swatch-rail i {
  display: block;
  height: 4px;
  margin-bottom: 5px;
  background: var(--swatch-ink);
  opacity: 0.18;
}
.swatch-rail i:first-child {
  background: var(--swatch-ink);
  opacity: 1;
}
.swatch-screen {
  position: absolute;
  top: 19px;
  left: 26%;
  right: 6px;
  height: 30px;
  padding: 7px;
  background: var(--swatch-screen);
  border: 1px solid var(--swatch-ink);
  border-radius: 2px;
}
.swatch-screen::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  width: 3px;
  height: 14px;
  background: var(--swatch-accent);
}
.swatch-screen i {
  display: block;
  width: 62%;
  height: 3px;
  margin-bottom: 4px;
  background: var(--swatch-accent);
}
.swatch-screen i + i {
  width: 38%;
  opacity: 0.45;
}
.swatch-panel {
  position: absolute;
  top: 54px;
  right: 6px;
  bottom: 6px;
  left: 26%;
  background: var(--swatch-panel);
  border: 1px solid var(--swatch-line);
  border-radius: 2px;
}
.swatch-panel::before {
  content: '';
  position: absolute;
  left: 6px;
  top: 6px;
  width: 3px;
  height: 8px;
  background: var(--swatch-ink);
}
@media (max-width: 520px) {
  .theme-choices,
  .startup-options,
  .background-options {
    grid-template-columns: 1fr;
  }
}
</style>
