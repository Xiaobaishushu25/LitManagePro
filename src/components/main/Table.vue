<script setup lang="ts">
import AutoCompleteTag from "../../util/AutoCompleteTag.vue";
import {ref} from "vue";
import {Tag} from "./main-type.ts";
import useTagGroupsStore from "../../stroe/tag.ts";

const store = useTagGroupsStore()

const fetchSuggestions = async (query: string) => {
  if (query===" "){ //如果输入一个空格，则返回所有标签
    return store.allTags
  }
  return store.allTags.filter(option =>
      option.value.includes(query)
  )
}
</script>

<template>
  <div>
    <n-space vertical>
      {{store.currentSelectTags}}
      <auto-complete-tag
          v-model:modelValue="store.currentSelectTags"
          placeholder="按空格提示所有标签"
          :options="store.allTags"
          :fetchSuggestions="fetchSuggestions"
      />
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