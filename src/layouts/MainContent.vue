<script setup lang="ts">

import Table from "../components/main/Table.vue";
import TagCard from "../components/main/TagCard.vue";
import {onMounted, ref} from "vue";
import useConfigStore from "../stroe/config.ts";

const configStore = useConfigStore()
console.log(configStore.config?.ui_config)
const splitSize = ref(configStore.config?.ui_config.split_size[0]||0.2);
const handleDragEnd = () => {
  console.log('拖动结束，新的分隔比例:');
  const formattedSize = parseFloat(splitSize.value.toFixed(2));
  console.log('拖动结束，新的分隔比例（保留两位小数）:', formattedSize);
};
onMounted(() => {
  console.log(configStore.config?.ui_config)
})
</script>

<template>
<div>
  <n-split direction="horizontal" class="h-full" :max="0.85" :min="0.1" :size="splitSize"   @update:size="(e:number) => splitSize = e" @drag-end="handleDragEnd">
    <template #1>
      <TagCard></TagCard>
    </template>
    <template #2>
      <Table/>
    </template>
  </n-split>
</div>
</template>

<style scoped>

</style>