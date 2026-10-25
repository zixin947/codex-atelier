<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import Icon from './Icon.vue'
const props = defineProps<{ title: string; returnFocus?: HTMLElement | null }>()
const emit = defineEmits<{ close: [] }>()
const dialog = ref<HTMLDialogElement>()
let previous: HTMLElement | null = null
onMounted(() => {
  previous = props.returnFocus ?? (document.activeElement as HTMLElement)
  dialog.value?.showModal()
})
onUnmounted(() => {
  void nextTick(() => previous?.focus())
})
</script>
<template>
  <dialog ref="dialog" class="modal" aria-labelledby="modal-title" @cancel.prevent="emit('close')">
    <div class="panel-head">
      <span id="modal-title">{{ title }}</span
      ><button class="icon-button" aria-label="关闭弹窗" @click="emit('close')">
        <Icon name="close" />
      </button>
    </div>
    <div class="modal-body"><slot /></div>
  </dialog>
</template>
