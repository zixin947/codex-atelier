<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue'
import { backgroundOptions } from '../../app/theme'
import { mountAtelierBackground, type BackgroundRenderer } from './atelier-background'
const props = defineProps<{ host: HTMLElement }>()
let renderer: BackgroundRenderer | undefined
onMounted(() => {
  renderer = mountAtelierBackground(props.host, () => backgroundOptions.value)
})
watch(backgroundOptions, () => renderer?.refresh(), { deep: true })
onUnmounted(() => renderer?.destroy())
</script>
<template>
  <!-- Both the sheet and its mark are drawn by the renderer into one canvas so
       the shell's flex layout sees no extra child. -->
  <span hidden aria-hidden="true" />
</template>
