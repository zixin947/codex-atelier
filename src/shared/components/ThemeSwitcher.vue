<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { theme, setTheme, type ThemeId } from '../../app/theme'
import Icon from './Icon.vue'
const emit = defineEmits<{ change: [label: string, saved: boolean] }>()
const details = ref<HTMLDetailsElement>()
const trigger = ref<HTMLElement>()
const label = computed(() => (theme.value === 'endfield' ? '终末地' : '千禧机壳'))
const choices: { id: ThemeId; label: string; detail: string }[] = [
  { id: 'endfield', label: '终末地', detail: '纸墨工业 · 动态等高线' },
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
  <details ref="details" class="theme-switcher" @keydown="focusChoice">
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
  height: 34px;
  padding: 0 10px;
  border: var(--border);
  border-radius: var(--radius);
  background: var(--panel);
  box-shadow: var(--key);
  cursor: pointer;
  list-style: none;
  font-size: 11px;
  white-space: nowrap;
}
.theme-trigger::-webkit-details-marker {
  display: none;
}
.theme-trigger > span {
  color: var(--muted);
}
.theme-trigger > b {
  font-weight: 600;
}
.theme-trigger-arrow {
  transform: rotate(90deg);
}
.theme-switcher[open] .theme-trigger-arrow {
  transform: rotate(-90deg);
}
.theme-trigger:hover {
  background: var(--paper);
}
.theme-trigger:focus-visible {
  outline: 2px solid var(--ink);
  outline-offset: 3px;
}
.theme-menu {
  position: absolute;
  right: 0;
  top: calc(100% + 9px);
  z-index: 20;
  width: 244px;
  padding: 7px;
  background: var(--paper);
  border: var(--border);
  box-shadow: 3px 3px 0 var(--edge);
}
.theme-menu-label {
  display: flex;
  justify-content: space-between;
  padding: 6px 7px 10px;
  color: var(--muted);
  font-size: 10px;
}
.theme-menu-label small {
  font: 8px var(--font-mono);
  letter-spacing: 1px;
}
.theme-menu-choice {
  display: flex;
  align-items: center;
  width: 100%;
  gap: 10px;
  padding: 10px 7px;
  border: 1px solid transparent;
  border-radius: var(--radius);
  background: transparent;
  color: var(--ink);
  text-align: left;
}
.theme-menu-choice:hover {
  background: var(--panel);
  border-color: var(--edge);
}
.theme-menu-choice[aria-pressed='true'] {
  border-color: var(--line);
  background: var(--panel);
}
.theme-menu-copy {
  flex: 1;
}
.theme-menu-copy b {
  display: block;
  font-size: 12px;
}
.theme-menu-copy small {
  display: block;
  color: var(--muted);
  font-size: 10px;
  margin-top: 5px;
}
.theme-menu-swatch {
  --swatch-accent: #fff500;
  --swatch-bg: #e8e8e2;
  --swatch-ink: #171a18;
  position: relative;
  width: 31px;
  height: 32px;
  border: 1px solid var(--swatch-ink);
  background: var(--swatch-bg);
  flex-shrink: 0;
}
.theme-menu-swatch.millennium {
  --swatch-accent: #8fd6a0;
  --swatch-bg: #ddd4bf;
  --swatch-ink: #403c37;
}
.theme-menu-swatch::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 7px;
  background: var(--swatch-accent);
  border-right: 1px solid var(--swatch-ink);
}
.theme-menu-swatch i {
  display: block;
  position: absolute;
  left: 11px;
  right: 3px;
  top: 5px;
  height: 11px;
  background: var(--swatch-ink);
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
