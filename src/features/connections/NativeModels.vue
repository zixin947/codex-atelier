<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { gatewayService } from '../../services/gateway'
import OriginalAgents from './OriginalAgents.vue'
defineProps<{ view: 'agents' | 'providers' | 'routing' | 'groups' | 'gateway'; title: string }>()
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
    <OriginalAgents :running="running" :view="view" :title="title" @changed="refresh" />
  </div>
</template>
