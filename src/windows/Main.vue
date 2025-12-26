<script setup lang="ts">
import RightBar from "../layouts/RightBar.vue";
import MainContent from "../layouts/MainContent.vue";
import Footer from "../layouts/Footer.vue";
import {onMounted, onUnmounted,watch} from "vue";
import {emitTo, listen} from "@tauri-apps/api/event";
import {message} from "../message.ts";
import {invoke} from "@tauri-apps/api/core";
import TitleBar from "../components/main/TitleBar.vue";
import hotkeys from "hotkeys-js";
import useConfigStore from "../stroe/config.ts";

let configStore = useConfigStore()

let unlistenMsg: () => void;

onMounted(async ()=>{
  unlistenMsg = await listen('backend_message', (event:{payload:string}) => {
    message.error(event.payload);
  })
  invoke<string[]>('first_run',{}).then(data => {
    //一开始运行，拿一些ui初始化前后端的错误信息展示（如果有）
    data.forEach(item => {
      message.error(item);
    })
  }).catch(e => {
    message.error(e);
  })
})
watch(()=>configStore.shortcuts, () => {
  let shortcuts = configStore.shortcuts;
  if (shortcuts==undefined||shortcuts.length==0)return
  // 提取所有快捷键的 value 并去掉空字符串，形成一个字符串数组
  const hotkeysList = shortcuts.filter(item => item.value).map(item => item.value).join(',');
  hotkeys.unbind(); // 清除之前的监听
  hotkeys.filter = function(_event){
    //https://github.com/jaywcjlove/hotkeys-js/issues/113 确保在输入框内也能触发快捷键
    return true;
  }
  hotkeys(hotkeysList, function (event, handler) {
    event.preventDefault();// 阻止默认行为，比如Ctrl+p是浏览器打印
    // 找到对应的快捷键名称
    const matchedShortcut = shortcuts.find(item => item.value === handler.key);
    if (matchedShortcut) {
      console.log(`${matchedShortcut.key}快捷键触发`)
      emitTo('main',matchedShortcut.key,{})
    } else {
      message.error('Unknown shortcut triggered: ' + handler.key);
    }
  });
}, {immediate: true, deep: true });
onUnmounted(async ()=>{
  unlistenMsg();
  hotkeys.unbind();
})
</script>

<template>
  <div class="h-screen w-full flex flex-col">
    <!-- 顶部标题栏 -->
    <TitleBar></TitleBar>
<!--    <WindowTitlebar class="flex-shrink-0  window-titlebar"></WindowTitlebar>-->
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