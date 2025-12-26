<script setup lang="ts">
import Appearance from "../components/setting/Appearance.vue";
import useConfigStore from "../stroe/config.ts";
import TitleBar from "../components/TitleBar.vue";
import Keymap from "../components/setting/Keymap.vue";

const configStore = useConfigStore()

</script>

<template>
  <TitleBar title="设置"></TitleBar>
  <div v-if="configStore.config">
    <n-tabs
        type="card"
        placement="left"
    >
      <n-tab-pane name="外观" tab="外观"><appearance></appearance></n-tab-pane>
      <n-tab-pane name="功能" tab="功能"></n-tab-pane>
      <n-tab-pane name="快捷键" tab="快捷键">
        <keymap></keymap>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>

</style>