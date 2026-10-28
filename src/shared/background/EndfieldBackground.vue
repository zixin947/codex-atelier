<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue'
import { backgroundOptions } from '../../app/theme'
import { mountEndfieldContour, type ContourRenderer } from './vendor/endfield-contour.js'
const props = defineProps<{ host: HTMLElement }>()
let renderer: ContourRenderer | undefined
onMounted(() => {
  renderer = mountEndfieldContour(props.host, () => backgroundOptions.value)
})
watch(backgroundOptions, () => renderer?.refresh(), { deep: true })
onUnmounted(() => renderer?.destroy())
</script>
<template>
  <div
    v-if="backgroundOptions.enabled && backgroundOptions.watermark"
    class="field-background-wordmark"
    data-endfield-watermark="persist"
    aria-hidden="true"
  />
</template>
