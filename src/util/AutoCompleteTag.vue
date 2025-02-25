<script setup lang="ts">
import {nextTick, ref, watch} from 'vue';
import {Tag} from "../components/main/main-type.ts";


interface Props {
  modelValue: Tag[]
  options?: Tag[]
  labelKey?: string
  placeholder?: string
  triggerOnFocus?: boolean
  fetchSuggestions?: (query: string) => Promise<Tag[]>
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => [],
  options: () => [],
  labelKey: 'value',
  triggerOnFocus: true
})

const emit = defineEmits(['update:modelValue'])

const inputValue = ref('')
const showMenu = ref(false)
const suggestions = ref<Tag[]>([])
const inputRef = ref<HTMLInputElement>()
const isEmpty = ref(false)

// const selectedIndex = ref(-1); // 添加一个变量来跟踪当前选中的匹配项索引

const getTagLabel = (tag: Tag) => {
  return tag[props.labelKey]
}

const handleSelect = (value: Tag) => {
  console.log('handleSelect', value)
  const newValue = [...props.modelValue, value]
  emit('update:modelValue', newValue)
  nextTick(() => {
    inputValue.value = ''
    showMenu.value = false
    // selectedIndex.value = 0; // 重置选中索引
    //如果使用鼠标选中匹配项完成后，后续不知道为什么再次输入不能再触发提示了，
    //只有输入框失焦后再重现点击获得焦点，才可以继续有提示
    //这里简单粗暴的解决了这个问题，其实不知道到底为什么。
    inputRef?.value?.blur()
    inputRef.value?.focus()
  })
}

const handleCloseTag = (index: number) => {
  const newValue = props.modelValue.filter((_, i) => i !== index)
  emit('update:modelValue', newValue)
}
// let keyArrow = false;
const handleKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Backspace' && !inputValue.value) {
    const newValue = [...props.modelValue];
    newValue.pop();
    emit('update:modelValue', newValue);
  }
};

const handleContainerClick = () => {
  inputRef.value?.focus()
}

watch(inputValue, (newValue) => {
  if (newValue) {
    showMenu.value = true
    handleSearch(newValue)
  } else {
    showMenu.value = false
    isEmpty.value = false
  }
})

const handleSearch = async (query: string) => {
  if (props.fetchSuggestions) {
    const result = await props.fetchSuggestions(query)
    suggestions.value = result
  } else {
    suggestions.value = props.options.filter(option =>
        getTagLabel(option).toLowerCase().includes(query.toLowerCase())
    )
  }
  isEmpty.value = suggestions.value.length === 0
  // selectedIndex.value = -1; // 重置选中索引
}
</script>

<template>
  <div class="tag-input" @click="handleContainerClick">
    <n-auto-complete
        v-model:value="inputValue"
        :options="suggestions.map(tag => ({ label: tag.value, value: tag }))"
        :show="showMenu"
        :trigger-on-focus="triggerOnFocus"
        @select="handleSelect"
        :show-empty="isEmpty"
        :clear-after-select="true"
        @update:show="(val) => (showMenu = val)"
    >
      <template #empty>
        <div>没有匹配的标签</div>
      </template>
      <template #default="{ handleBlur, handleFocus }">
        <div class="input-container">
          <n-tag
              v-for="(tag, index) in modelValue"
              :key="index"
              closable
              @close="handleCloseTag(index)"
              :style="{
                backgroundColor: tag.bg_color,
                color: tag.text_color
              }"
          >
            {{ getTagLabel(tag) }}
          </n-tag>
          <input
              ref="inputRef"
              v-model="inputValue"
              :placeholder="props.placeholder"
              class="input"
              type="text"
              @blur="handleBlur"
              @focus="handleFocus"
              @keydown="handleKeydown"
          />
        </div>
      </template>
    </n-auto-complete>
  </div>
</template>

<style scoped>
.tag-input {
  position: relative;
  border: 1px solid #ddd;
  border-radius: 3px;
  padding: 4px;
  min-height: 36px;
}

.input-container {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.input {
  flex: 1;
  min-width: 100px;
  border: none;
  outline: none;
  padding: 4px;
  background: transparent;
}
</style>