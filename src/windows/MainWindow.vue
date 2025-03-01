<script setup lang="ts">

import {WindowTitlebar} from "@tauri-controls/vue";
import RightBar from "../layouts/RightBar.vue";
import MainContent from "../layouts/MainContent.vue";
import Footer from "../layouts/Footer.vue";
import {onMounted, onUnmounted} from "vue";
import {listen} from "@tauri-apps/api/event";
import {message} from "../message.ts";
import {invoke} from "@tauri-apps/api/core";

let unlistenMsg: () => void;
onMounted(async ()=>{
  unlistenMsg = await listen('backend_message', (event:{payload:string}) => {
    message.error(event.payload);
  })

  invoke<string[]>('first_run',{}).then(data => {
    data.forEach(item => {
      message.error(item);
    })
  }).catch(e => {
    message.error(e);
  })
})

onUnmounted(async ()=>{
  unlistenMsg();
})
</script>

<template>
  <div class="h-screen w-full flex flex-col">
    <!-- 顶部标题栏 -->
    <WindowTitlebar class="flex-shrink-0  window-titlebar">LitManagePro</WindowTitlebar>
    <div class="flex-grow flex flex-col">
      <!-- 中间内容区域 -->
<!--      用h-[calc(100vh-10rem)]来强制设置主内容的高度，避免被撑开导致软件出现滚动条，注意，这里这个高度可以设小，
，即使很小内容不会被挤压的，应该是flex-grow 让中间区域填充剩余空间，如果设置的高度过大话，还是会出现滚动条-->
      <div class="flex flex-grow h-[calc(100vh-10rem)]"> <!-- flex-grow 让中间区域填充剩余空间 -->
        <!-- 主内容区 -->
        <MainContent class="flex-grow"/> <!-- flex-grow 让 MainContent 填充剩余空间 -->
        <!-- 右侧边栏（固定宽度且禁止收缩） -->
        <RightBar class="w-10 flex-shrink-0"/> <!-- w-10 = 40px, flex-shrink-0 禁止收缩 -->
      </div>
      <!-- 底部栏（强制显示在视口底部） -->
      <Footer class="h-7 flex-shrink-0 w-full"/> <!-- h-7 = 30px, flex-shrink-0 禁止收缩, w-full 宽度100% -->
    </div>
  </div>
</template>

<style scoped>
.window-titlebar :deep(button) {
  @apply  hover:border-transparent shadow-none active:border-transparent;
}
</style>