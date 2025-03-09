<script setup lang="ts">

import Table from "../components/main/Table.vue";
import TagCard from "../components/main/TagCard.vue";
import {ref} from "vue";
import useConfigStore from "../stroe/config.ts";

const configStore = useConfigStore()
// watch(()=>configStore.config,async (_newValue, oldValue)=>{
//   if (oldValue==undefined){
//     splitSize.value = configStore.config?.ui_config.split_size[0]||0.2;
//   }
// })
const splitSize = ref(configStore.config?.ui_config.split_size[0]||0.2);
const handleDragEnd = () => {
  //注意，这个配置改变config那边没监听到，但是最后保存时是没问题的。
  configStore.config!.ui_config.split_size[0] = parseFloat(splitSize.value.toFixed(2));
};
</script>

<template>
<div>
  <n-split direction="horizontal" class="h-full" :max="0.85" :min="0.1" :size="splitSize"  @update:size="(e:number) => splitSize = e" @drag-end="handleDragEnd">
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