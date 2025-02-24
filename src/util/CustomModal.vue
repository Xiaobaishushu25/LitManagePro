<script lang="ts" setup>
import { ref, nextTick, watch } from 'vue';

const props = defineProps({
  show: {
    type: Boolean,
    required: true
  },
  title: {
    type: String,
    required: true
  },
  onConfirm: {
    type: Function,
    required: true
  },
});

const emit = defineEmits(['update:show']);

// const inputRef = ref<HTMLInputElement | null>(null);

const closeModal = () => {
  emit('update:show', false);
};

const confirmAction = () => {
  props.onConfirm();
  closeModal();
};

// const focusInput = () => {
//   nextTick(() => {
//     inputRef.value?.focus();
//   });
// };

// watch(() => props.show, (newVal) => {
//   if (newVal) {
//     focusInput();
//   }
// }, { immediate: true });
</script>

<template>
  <n-modal
      :show="show"
      preset="card"
      class="w-80"
      :mask-closable="false"
      :draggable="true"
      @close="closeModal"
  >
    <template #header>
      <div v-if="$slots.header">
        <slot name="header"></slot>
      </div>
      <div v-else>
        {{ title }}
      </div>
    </template>
<!--    <n-flex vertical>-->
<!--      <n-input v-model:value="inputValue" :placeholder="placeholder" ref="inputRef" />-->
      <slot></slot>
<!--    </n-flex>-->
    <template #action>
      <n-space>
        <n-button @click="closeModal">取消</n-button>
        <n-button @click="confirmAction">确认</n-button>
      </n-space>
    </template>
  </n-modal>
</template>

<style scoped>

</style>