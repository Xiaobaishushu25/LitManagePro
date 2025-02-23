<script setup lang="ts">

import Table from "../components/main/Table.vue";
import TagCard from "../components/main/TagCard.vue";
</script>

<template>
<div>
  <n-split direction="horizontal" class="h-full" :max="0.85" :min="0.1" :default-size="0.2">
    <template #1>
<!--      <Tree/>-->
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