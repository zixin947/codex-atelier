<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { gatewayService } from '../../services/gateway'
import OriginalAgents from './OriginalAgents.vue'
defineProps<{
  view: 'agents' | 'providers' | 'routing' | 'groups' | 'gateway' | 'usage'
  title: string
  usageTab?: 'usage' | 'requests' | 'sessions' | 'context'
}>()
const running = ref(false),
  error = ref('')
async function refresh() {
  try {
    running.value = (await gatewayService.snapshot()).runtime.state === 'running'
  } catch (e) {
    error.value = String(e)
  }
}
onMounted(refresh)
</script>

<template>
  <div class="page native-models-page">
    <p v-if="error" role="alert">{{ error }}</p>
    <OriginalAgents
      :running="running"
      :view="view"
      :title="title"
      :usage-tab="usageTab"
      @changed="refresh"
    />
  </div>
</template>
