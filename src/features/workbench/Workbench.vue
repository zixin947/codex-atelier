<script setup lang="ts">
import NativeModels from '../connections/NativeModels.vue'
withDefaults(defineProps<{ tab?: 'usage' | 'requests' | 'sessions' | 'context' }>(), {
  tab: 'usage',
})
const labels = { usage: '概览', requests: '请求', sessions: '会话', context: '上下文' }
</script>

<template>
  <section class="workbench">
    <NativeModels view="usage" :usage-tab="tab" :title="`ChatGPT · ${labels[tab]}`" />
  </section>
</template>
