<script setup lang="ts">
import { ref, PropType, nextTick } from 'vue'
import { NInput } from 'naive-ui'

const props = defineProps({
  value: {
    type: String as PropType<string | null>,
    default: null,
  },
  onSave: {
    type: Function as PropType<(newValue: string | null) => void>,
    required: true
  },
  // 新增样式配置属性
  contentClass: {
    type: [String, Array, Object] as PropType<string | any[] | object>,
    default: 'text-gray-700'
  },
  contentStyle: {
    type: [Object, String] as PropType<object | string>,
    default: () => ({})
  }
})

const emit = defineEmits(['update:value'])

const inputRef = ref<InstanceType<typeof NInput> | null>(null)
const isEditing = ref(false)

const originalValue = ref<string | null>(null) // 用于保存原始值
function toggleEdit() {
  originalValue.value = props.value // 保存当前值为原始值
  isEditing.value = true
  nextTick(() => {
    inputRef.value?.focus()
  })
}

function handleSave() {
  isEditing.value = false
  const finalValue = props.value?.trim() === '' ? null : props.value?.trim()
  if (finalValue === undefined) {
    return
  }
  // 比较新值和原始值是否相同
  if (finalValue !== originalValue.value) {
    if (typeof props.onSave === 'function') {
      props.onSave(finalValue)
    } else {
      emit('update:value', finalValue)
    }
  }
  // if (typeof props.onSave === 'function') {
  //   props.onSave(finalValue)
  // } else {
  //   emit('update:value', finalValue)
  // }
}
</script>

<template>
  <div
      class="flex items-center justify-between rounded-md"
      @click="toggleEdit"
  >
    <!-- 本来给这个输入支持回车提交的，但是发现会和blur一起触发，再加上想支持换行，索性去掉回车提交了 -->
    <n-input
        v-if="isEditing"
        ref="inputRef"
        class="w-full p-0 bg-transparent border-none focus:ring-0"
        :autofocus="true"
        :value="props.value"
        type="textarea"
        :autosize="true"
        :placeholder="props.value ? '' : '点击编辑...'"
        @update:value="(newValue) => emit('update:value', newValue)"
        @blur="handleSave"
    />
    <!-- 暴露样式的显示区域 -->
<!--    <div-->
<!--        v-else-->
<!--        style="white-space: pre-wrap; /* 保留空白和换行 */word-wrap: break-word;"-->
<!--        :class="contentClass"-->
<!--        :style="contentStyle"-->
<!--    >-->
<!--      {{ props.value || '点击编辑...' }}-->
<!--    </div>-->
    <div
        v-else
        :class="contentClass"
        :style="contentStyle"
        v-html="props.value || '点击编辑...'"
    ></div>
  </div>
</template>

<style scoped>

</style>