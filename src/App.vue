<script setup lang="ts">
import {onMounted, onUnmounted} from "vue";
import { invoke } from "@tauri-apps/api/core";

import { getCurrentWindow } from "@tauri-apps/api/window";

const store = useConfigStore()


let unlisten: () => void;
onMounted(async () => {
  unlisten = await getCurrentWindow().onCloseRequested(async (event) => {
    console.log("close requested");
    event.preventDefault();
    await invoke('save_config',{config: store.config}).then(_ => {}).catch(e => {
      message.error(`保存配置出错${e}`);
    })
    await invoke('exit_app', {})
  });

  await invoke<Config>('get_config',{}).then(data => {
    console.log(data);
    store.config = data;
  }).catch(e => {
    console.log(e);
  })
})
onUnmounted(async ()=>{
  unlisten();
})


import {message, showMessage} from './message.ts';
import {Config} from "./config-type.ts";
import useTagGroupsStore from "./stroe/tag.ts";
import useConfigStore from "./stroe/config.ts";
function mess(){
  showMessage('Hello World');
  message.error('Hello World，Hello World，Hello World，Hello World，Hello WorldHello WorldHello WorldHello WorldHello World');
}
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