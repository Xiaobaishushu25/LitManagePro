<script setup lang="ts">
import Appearance from "../components/setting/Appearance.vue";
import useConfigStore from "../stroe/config.ts";

const configStore = useConfigStore()

</script>

<template>
  <div v-if="configStore.config">
    <n-tabs
        type="card"
        placement="left"
    >
      <n-tab-pane name="外观" tab="外观"><appearance></appearance></n-tab-pane>
      <n-tab-pane name="功能" tab="功能">
        <div>
        </div>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>

</style>