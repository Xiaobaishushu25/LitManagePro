<script setup lang="ts">
import {onMounted, onUnmounted, watch} from "vue";
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
    // await invoke('exit_app', {})
  });
  // watch(()=>configStore.config,async (newValue, _oldValue)=>{
  //   console.log("在这里更新会是什么结果")
  //   if (newValue!=undefined){
  //     console.log("进来调用update_config")
  //     invoke('update_config', { config: newValue }).then(() => {}).catch(() => {})
  //   }
  // },{deep:true})
  // watch(() => configStore.config.value, (newConfig) => {
  //   if (newConfig?.ui_config?.tag_group_state) {
  //     for (const [key, value] of Object.entries(newConfig.ui_config.tag_group_state)) {
  //       const numericKey = Number(key);
  //       if (!isNaN(numericKey)) {
  //         tagGroupStates[numericKey] = value;
  //       }
  //     }
  //   }
  //   console.log("配置变化了")
  //   if (newConfig!=undefined){
  //     console.log("进来调用update_config")
  //     invoke('update_config', { config: newConfig }).then(() => {}).catch(() => {})
  //   }
  // }, { immediate: true,deep : true });
})

// 暴露 store 实例
onUnmounted(async ()=>{
  unlisten();
})

</script>

<template>
  <router-view v-slot="{ Component }">
    <keep-alive>
      <component :is="Component" />
    </keep-alive>
  </router-view>
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