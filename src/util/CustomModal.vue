<script lang="ts" setup>
/// @author xbss
/// @description 自定义模态窗口，用于一些简单的提示和交互操作
/// @param {Boolean} show - 模态窗口是否显示
/// @param {String} title - 模态窗口标题，如果使用header插槽时，则默认渲染插槽内容
/// @param {Function} onConfirm - 确认按钮点击事件
/// @solt {#header} - 头部插槽
/// @solt {#default} - 内容插槽
/// @example
/// <CustomModal v-model:show="show" title="提示" @onConfirm="handleConfirm">
///   <template#header>my title</template#header>
///   <div>
///     <n-input v-model:value="inputValue" :placeholder="placeholder" ref="inputRef" />
///   </div>
/// </CustomModal>
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

const closeModal = () => {
  emit('update:show', false);
};

const confirmAction = () => {
  props.onConfirm();
  closeModal();
};

</script>

<template>
  <n-modal
      :show="show"
      preset="card"
      class="w-fit"
      :mask-closable="false"
      :draggable="true"
      transform-origin="center"
      @close="closeModal"
      :segmented="{
        content: true,
        footer: 'soft',
      }"
      action-class="action"
      action-style="padding: 0 !important"
  >
    <template #header>
      <div v-if="$slots.header">
        <slot name="header"></slot>
      </div>
      <div v-else>
        {{ title }}
      </div>
    </template>
      <slot></slot>
    <template #action>
      <n-flex justify="end">
        <n-button size="small" @click="closeModal">取消</n-button>
        <n-button size="small" type="success" class="text-black" @click="confirmAction">确认</n-button>
      </n-flex>
    </template>
  </n-modal>
</template>

<style>
.n-card-header{
  padding: 15px 20px 10px 20px !important;
}
.n-card__action{
  padding: 5px 20px 10px 0 !important;
}
</style>
<style scoped>
</style>