<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { theme, setTheme, type ThemeId } from '../../app/theme'
import Icon from './Icon.vue'
const emit = defineEmits<{ change: [label: string, saved: boolean]; openChange: [open: boolean] }>()
const details = ref<HTMLDetailsElement>()
const trigger = ref<HTMLElement>()
const label = computed(() => choices.find((choice) => choice.id === theme.value)!.label)
const choices: { id: ThemeId; label: string; detail: string }[] = [
  { id: 'endfield', label: '终末地', detail: '纸墨工业 · 信号色与测绘网格' },
  { id: 'millennium', label: '千禧机壳', detail: '奶油面板 · 薄荷绿' },
]
function close(restoreFocus = false) {
  if (!details.value?.open) return
  details.value.open = false
  if (restoreFocus) trigger.value?.focus()
}
function choose(choice: (typeof choices)[number]) {
  if (theme.value !== choice.id) emit('change', choice.label, setTheme(choice.id))
  close(true)
}
function outside(event: PointerEvent) {
  if (!details.value?.contains(event.target as Node)) close()
}
function escape(event: KeyboardEvent) {
  if (event.key === 'Escape' && details.value?.open) {
    event.preventDefault()
    event.stopPropagation()
    close(true)
  }
}
function focusChoice(event: KeyboardEvent) {
  if (event.key !== 'ArrowDown' && event.key !== 'ArrowUp') return
  event.preventDefault()
  if (!details.value) return
  details.value.open = true
  const buttons = [...details.value.querySelectorAll<HTMLButtonElement>('.theme-menu-choice')]
  const index = buttons.indexOf(document.activeElement as HTMLButtonElement)
  const next =
    index < 0
      ? event.key === 'ArrowDown'
        ? 0
        : buttons.length - 1
      : (index + (event.key === 'ArrowDown' ? 1 : -1) + buttons.length) % buttons.length
  buttons[next]?.focus()
}
onMounted(() => {
  document.addEventListener('pointerdown', outside)
  document.addEventListener('keydown', escape)
})
onUnmounted(() => {
  document.removeEventListener('pointerdown', outside)
  document.removeEventListener('keydown', escape)
})
</script>
<template>
  <details ref="details" class="theme-switcher" @keydown="focusChoice" @toggle="emit('openChange', !!details?.open)">
    <summary
      ref="trigger"
      class="theme-trigger"
      :aria-label="'切换皮肤，当前：' + label"
      title="切换皮肤"
    >
      <Icon name="palette" :size="16" /><span>皮肤</span><b>{{ label }}</b
      ><Icon name="chevron" :size="12" class="theme-trigger-arrow" />
    </summary>
    <div class="theme-menu" role="group" aria-label="选择皮肤">
      <span class="theme-menu-label">界面皮肤 <small>APPEARANCE</small></span>
      <button
        v-for="choice in choices"
        :key="choice.id"
        type="button"
        class="theme-menu-choice"
        :aria-pressed="theme === choice.id"
        :aria-label="'使用' + choice.label + '皮肤'"
        @click="choose(choice)"
      >
        <span class="theme-menu-swatch" :class="choice.id" aria-hidden="true"><i /></span>
        <span class="theme-menu-copy"
          ><b>{{ choice.label }}</b
          ><small>{{ choice.detail }}</small></span
        >
        <Icon v-if="theme === choice.id" name="check" :size="15" />
      </button>
    </div>
  </details>
</template>
<style scoped>
.theme-switcher {
  position: relative;
  flex-shrink: 0;
}
.theme-trigger {
  display: flex;
  align-items: center;
  gap: 7px;
  height: var(--control);
  padding: 0 10px;
  font-size: 11px;
  white-space: nowrap;
  color: var(--ink);
  background: var(--elevated);
  border: 1px solid var(--control-line);
  border-radius: var(--radius-control);
  cursor: pointer;
  list-style: none;
  transition: background-color var(--motion-fast) var(--ease);
}
.theme-trigger::-webkit-details-marker {
  display: none;
}
.theme-trigger > span {
  color: var(--muted);
}
.theme-trigger > b {
  font-weight: 700;
}
.theme-trigger-arrow {
  transform: rotate(90deg);
}
.theme-switcher[open] .theme-trigger-arrow {
  transform: rotate(-90deg);
}
.theme-trigger:hover {
  background: var(--inset);
}
.theme-trigger:focus-visible {
  outline: 2px solid var(--focus);
  outline-offset: 3px;
}

.theme-menu {
  position: absolute;
  right: 0;
  top: calc(100% + 8px);
  z-index: 20;
  width: 248px;
  padding: 6px;
  background: var(--elevated);
  border: 1px solid var(--ink);
  border-radius: var(--radius-panel);
  box-shadow: var(--shadow-overlay);
}
.theme-menu-label {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  padding: 4px 6px 8px;
  font-size: 10px;
  font-weight: 700;
  color: var(--ink);
}
.theme-menu-label small {
  font: 8px var(--font-mono);
  letter-spacing: 0.12em;
  color: var(--faint);
}
.theme-menu-choice {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 9px;
  padding: 8px 6px;
  text-align: left;
  color: var(--ink);
  background: transparent;
  border: 1px solid transparent;
  border-radius: var(--radius-control);
}
.theme-menu-choice:hover {
  background: var(--inset);
}
.theme-menu-choice[aria-pressed='true'] {
  border-color: var(--ink);
  background: var(--inset);
  box-shadow: inset 3px 0 0 var(--signal);
}
.theme-menu-choice[aria-pressed='true'] > svg {
  color: var(--accent-text);
}
.theme-menu-copy {
  flex: 1;
  min-width: 0;
}
.theme-menu-copy b {
  display: block;
  font-size: 12px;
  font-weight: 700;
}
.theme-menu-copy small {
  display: block;
  margin-top: 3px;
  font-size: 10px;
  color: var(--muted);
}

/* Swatch: the same four-part structure as the picker's larger preview. */
.theme-menu-swatch {
  --swatch-accent: #fffa00;
  --swatch-bg: #eceee6;
  --swatch-ink: #20231f;
  position: relative;
  flex-shrink: 0;
  width: 30px;
  height: 30px;
  background: var(--swatch-bg);
  border: 1px solid var(--swatch-ink);
  border-radius: 2px;
  overflow: hidden;
}
.theme-menu-swatch.millennium {
  --swatch-accent: #8fd6a0;
  --swatch-bg: #ddd4bf;
  --swatch-ink: #393b34;
}
.theme-menu-swatch.hud {
  --swatch-bg: #202222;
  border-color: #fffa00;
  border-radius: 50%;
}
.theme-menu-swatch.hud::before {
  inset: 4px;
  width: auto;
  border: 1px solid #fffa00;
  border-radius: 50%;
  background: transparent;
}
.theme-menu-swatch.hud i {
  background: #d3d4cf;
}
.theme-menu-swatch::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 6px;
  background: var(--swatch-accent);
  border-right: 1px solid var(--swatch-ink);
}
.theme-menu-swatch i {
  position: absolute;
  left: 10px;
  right: 4px;
  top: 5px;
  height: 10px;
  background: var(--swatch-ink);
}
.theme-menu-swatch i::after {
  content: '';
  position: absolute;
  left: 10px;
  right: 4px;
  top: 15px;
  height: 5px;
  background: var(--swatch-ink);
  opacity: 0.25;
}

@media (max-width: 1050px) {
  .theme-trigger {
    gap: 5px;
    padding: 0 8px;
  }
  .theme-trigger > span {
    display: none;
  }
}
</style>
