<script setup lang="ts">
import { nextTick, onUnmounted, ref, useId } from 'vue'
defineOptions({ inheritAttrs: false })
defineProps<{ label: string }>()
const id = useId()
const trigger = ref<HTMLButtonElement>()
const panel = ref<HTMLElement>()
const open = ref(false)
const style = ref({ left: '0px', top: '0px', visibility: 'hidden' as 'hidden' | 'visible' })
let pinned = false
let timer: ReturnType<typeof setTimeout> | undefined
let tracking = false
function position() {
  if (!trigger.value || !panel.value) return
  const anchor = trigger.value.getBoundingClientRect()
  const box = panel.value.getBoundingClientRect()
  const top =
    anchor.bottom + 8 + box.height < window.innerHeight - 12
      ? anchor.bottom + 8
      : Math.max(12, anchor.top - box.height - 8)
  style.value = {
    left:
      Math.max(
        12,
        Math.min(
          window.innerWidth - box.width - 12,
          anchor.left + anchor.width / 2 - box.width / 2,
        ),
      ) + 'px',
    top: top + 'px',
    visibility: 'visible',
  }
}
function outside(event: PointerEvent) {
  if (
    !trigger.value?.contains(event.target as Node) &&
    !panel.value?.contains(event.target as Node)
  )
    close()
}
function key(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    event.stopPropagation()
    close()
  }
}
function track(value: boolean) {
  if (tracking === value) return
  tracking = value
  if (value) {
    window.addEventListener('resize', position)
    window.addEventListener('scroll', position, true)
    document.addEventListener('pointerdown', outside)
    document.addEventListener('keydown', key, true)
  } else {
    window.removeEventListener('resize', position)
    window.removeEventListener('scroll', position, true)
    document.removeEventListener('pointerdown', outside)
    document.removeEventListener('keydown', key, true)
  }
}
async function show() {
  clearTimeout(timer)
  open.value = true
  track(true)
  await nextTick()
  position()
}
function hover() {
  clearTimeout(timer)
  if (!open.value) timer = setTimeout(() => void show(), 120)
}
function keepOpen() {
  clearTimeout(timer)
}
function leave() {
  clearTimeout(timer)
  if (!pinned && document.activeElement !== trigger.value) timer = setTimeout(close, 160)
}
function close() {
  clearTimeout(timer)
  pinned = false
  open.value = false
  track(false)
}
function toggle() {
  if (pinned) close()
  else {
    pinned = true
    void show()
  }
}
onUnmounted(close)
</script>
<template>
  <button
    ref="trigger"
    v-bind="$attrs"
    type="button"
    class="hover-preview-trigger"
    :aria-label="label"
    :aria-describedby="open ? id : undefined"
    @pointerenter="hover"
    @pointerleave="leave"
    @focus="show"
    @blur="close"
    @click="toggle"
  >
    <slot />
  </button>
  <Teleport to="body">
    <aside
      v-if="open"
      :id="id"
      ref="panel"
      role="tooltip"
      class="hover-preview-panel"
      :style="style"
      @pointerenter="keepOpen"
      @pointerleave="leave"
    >
      <slot name="preview" />
    </aside>
  </Teleport>
</template>
<style>
.hover-preview-trigger {
  border: 0;
  padding: 0;
  background: none;
  color: inherit;
  font: inherit;
  cursor: pointer;
}
.hover-preview-panel {
  position: fixed;
  z-index: 100;
  width: min(276px, calc(100vw - 24px));
  max-height: calc(100vh - 24px);
  overflow: auto;
  padding: var(--space-4);
  border: var(--border);
  border-radius: var(--radius);
  background: var(--paper);
  color: var(--ink);
  box-shadow:
    inset 1px 1px 0 var(--highlight),
    3px 4px 0 var(--edge);
  font: 12px/1.6 var(--font-body);
}
</style>
