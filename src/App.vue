<script setup lang="ts">
import {onMounted, onUnmounted} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {message, } from './message.ts';
import {Config} from "./config-type.ts";
import useConfigStore from "./stroe/config.ts";
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';

const configStore = useConfigStore()

let unlisten: () => void;

invoke<Config>('get_config',{}).then(data => {
  configStore.config = data;
}).catch(e => {
  message.error(e);
})

onMounted(async () => {
  document.addEventListener('contextmenu', function(event) {
    event.preventDefault();
  });
  unlisten = await getCurrentWindow().onCloseRequested(async (_event) => {
    let window = getCurrentWindow();
    // event.preventDefault();
    await saveWindowState(StateFlags.ALL);
    if (window.label === 'main') {
      await invoke('save_config',{config: configStore.config}).then(_ => {}).catch(e => {
        message.error(`保存配置出错${e}`);
      })
    }
  });
})

// 暴露 store 实例
onUnmounted(async ()=>{
  unlisten();
})

</script>

<template>
  <div v-if="configStore.config">
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </div>
</template>

<style scoped>
</style>
<style>
:root {
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}
</style>