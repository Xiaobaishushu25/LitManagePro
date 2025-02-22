<script setup lang="ts">
import {onMounted, ref} from "vue";
import { invoke } from "@tauri-apps/api/core";
import {WindowTitlebar} from "@tauri-controls/vue";

// import { WindowTitlebar } from "tauri-controls"



const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
import { getCurrentWindow } from "@tauri-apps/api/window";
onMounted(async () => {
  // const unlisten = await getCurrentWindow().onCloseRequested(async (event) => {
  //   console.log("close requested");
  //   event.preventDefault();
  // });
})
//
// // you need to call unlisten if your handler goes out of scope e.g. the component is unmounted
// unlisten();

import {message, showMessage} from './message.ts';
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