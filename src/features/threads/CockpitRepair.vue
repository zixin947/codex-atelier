<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
const props = defineProps<{ total: number; selected: string[] }>()
const url = computed(() => '/cockpit.html?' + new URLSearchParams({ total: String(props.total), selected: JSON.stringify(props.selected) }))
const emit = defineEmits<{ close: []; repaired: [] }>()
function receive(event: MessageEvent) {
  if (event.origin !== location.origin) return
  if (event.data?.type === 'cockpit-repair-close') emit('close')
  if (event.data?.type === 'cockpit-repair-done') emit('repaired')
}
onMounted(() => window.addEventListener('message', receive))
onUnmounted(() => window.removeEventListener('message', receive))
</script>
<template><Teleport to="body"><iframe class="cockpit-original-frame" :src="url" title="线程修复" /></Teleport></template>
<style scoped>.cockpit-original-frame { position: fixed; inset: 0; width: 100%; height: 100%; border: 0; z-index: 10000; }</style>
