<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";

//这个TitleBar只有最小化和关闭按钮，给一些工具窗口使用的

// 定义 props，接收来自父组件的标题
defineProps({
  title: {
    type: String,
    required: true
  }
});

async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
}
async function window_close(){
  await saveWindowState(StateFlags.ALL);
  // let label = WebviewWindow.getCurrent().label;
  // console.log(`关闭${label}窗口`)
  await WebviewWindow.getCurrent().close();
}
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="w-5"></div> <!-- 占位符，不知道为什么给app.ico设置左边距不好使，直接用这个占空了 -->
    <img src="../assets/icon/app.ico" class="w-4 h-4" alt="ICO Icon">
    <label class="pl-1">{{title}}</label>
    <div class="ml-auto flex gap-0">
      <inline-svg
          src="../assets/svg/minimize.svg"
          class="window-button min"
          @click.left="window_minimize"
      ></inline-svg>
      <inline-svg
          src="../assets/svg/close.svg"
          class="window-button close"
          @click.left="window_close"
      ></inline-svg>
    </div>
  </div>
</template>

<style>
</style>