<script setup lang="ts">
defineProps<{ title: string; code?: string }>()
</script>
<template>
  <section class="panel">
    <div class="panel-head">
      <span>{{ title }}</span
      ><span class="panel-code">{{ code || '/////' }}</span>
    </div>
    <div class="panel-body"><slot /></div>
  </section>
</template>
