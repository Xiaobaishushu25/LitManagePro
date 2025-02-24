<script setup lang="ts">
import AutoCompleteTag from "../../util/AutoCompleteTag.vue";
import {ref} from "vue";
import {Tag} from "./MainType.ts";

// const selectedTags = ref([]);
// const options = ref([
//   { label: '标签1', value: 'tag1' },
//   { label: '标签2', value: 'tag2' },
//   { label: '标签3', value: 'tag3' }
// ]);
//
// const fetchSuggestions = async (query) => {
//   return options.value.filter(option =>
//       option.label.includes(query)
//   );
// };
const selectedTags = ref<Tag[]>([])
const options = ref<Tag[]>([
  { groupId: 1, id: 1, value: '标签1', bg_color: '#f00', text_color: '#fff' },
  { groupId: 1, id: 2, value: '标签2', bg_color: '#0f0', text_color: '#000' },
  { groupId: 1, id: 3, value: '标签3', bg_color: '#00f', text_color: '#fff' }
])

const fetchSuggestions = async (query: string) => {
  if (query===" "){ //如果输入一个空格，则返回所有标签
    return options.value
  }
  return options.value.filter(option =>
      option.value.includes(query)
  )
}
</script>

<template>
  <div>
    <n-space vertical>
      {{selectedTags}}
      <auto-complete-tag
          v-model:modelValue="selectedTags"
          placeholder="请输入标签"
          :options="options"
          :fetchSuggestions="fetchSuggestions"
      />
      <n-tag
          class="text-xs tag"
          type="success"
          :closable="true"
          :color="{color: '#d2666650',textColor: '#101010'}"
          :bordered="false"
      >
        扩散模型
      </n-tag>
    </n-space>
  </div>

</template>

<style scoped>
.tag :deep(g) {
  @apply  fill-black;
}
.tag :deep(button) {
  @apply  hover:border-red-600 shadow-none active:border-transparent;
}
</style>