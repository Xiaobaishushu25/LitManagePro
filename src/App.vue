<script setup lang="ts">
import {onMounted, onUnmounted} from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {message, } from './message.ts';
import {Config, ExeConfig} from "./config-type.ts";
import useConfigStore from "./stroe/config.ts";
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state';
import {listen} from "@tauri-apps/api/event";

const configStore = useConfigStore()

let unlisten: () => void;
let unExeListen: () => void;

onMounted(async () => {
  document.addEventListener('contextmenu', function(event) {
    event.preventDefault();
  });
  unlisten = await listen('update_exe_config',async (event:{ payload:ExeConfig[]}) => {
    configStore.updateExeConfig(event.payload)
  });
  unExeListen = await getCurrentWindow().onCloseRequested(async (event) => {
    // event.preventDefault();
    await saveWindowState(StateFlags.ALL);
    await invoke('save_config',{config: configStore.config}).then(_ => {}).catch(e => {
      message.error(`保存配置出错${e}`);
    })
    // await invoke('exit_app', {})
  });

  await invoke<Config>('get_config',{}).then(data => {
    configStore.config = data;
  }).catch(e => {
    message.error(e);
  })
})
onUnmounted(async ()=>{
  unlisten();
  unExeListen();
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