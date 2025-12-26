<script setup lang="ts">
import {onMounted, onUnmounted} from "vue";
import { invoke } from "@tauri-apps/api/core";
import {message, } from './message.ts';
import {Config} from "./config-type.ts";
import useConfigStore from "./stroe/config.ts";
import {emitTo} from "@tauri-apps/api/event";

const configStore = useConfigStore()

let unlisten: () => void;

invoke<Config>('get_config',{}).then(data => {
  configStore.config = data;
}).catch(e => {
  message.error(e);
})
// ======================
// 工具函数：只识别文件拖拽
// ======================
const isFileDrag = (e: DragEvent) => {
  return e.dataTransfer?.types?.includes("Files");
};

let dragEntered = false;

// ======================
// dragenter
// ======================
const handleDragEnter = async (e: DragEvent) => {
  // ⛔ 非文件拖拽（vue-draggable / DOM 拖拽）直接忽略
  if (!isFileDrag(e)) return;

  e.preventDefault();
  e.stopPropagation();

  // 防止多次触发
  if (dragEntered) return;
  dragEntered = true;

  await emitTo("main", "拖拽上传");
  await emitTo("dragImport", "drag-enter", {
    x: e.screenX,
    y: e.screenY
  });
};

// ======================
// dragleave
// ======================
const handleDragLeave = (e: DragEvent) => {
  if (!isFileDrag(e)) return;

  e.preventDefault();
  e.stopPropagation();

  dragEntered = false;
};

// ======================
// dragover
// ======================
const handleDragOver = (e: DragEvent) => {
  if (!isFileDrag(e)) return;

  // ⚠️ 必须 preventDefault，否则 drop 不生效
  e.preventDefault();
  e.stopPropagation();

  // ❗不要在这里改 cursor / dropEffect
  // 会影响 dragImport 窗口
};

// ======================
// drop
// ======================
const handleDrop = (e: DragEvent) => {
  if (!isFileDrag(e)) return;

  e.preventDefault();
  e.stopPropagation();

  dragEntered = false;
};
// let dragEntered = false;
// // 阻止拖拽文件到窗口上浏览器默认打开的事件
// const handleDragEnter = async (e: DragEvent) => {
//   e.preventDefault();
//   e.stopPropagation();
//   // 防止多次触发
//   if (dragEntered) return;
//   dragEntered = true;
//
//   await emitTo("main","拖拽上传")
//   await emitTo("dragImport", "drag-enter", {
//     x: e.screenX,
//     y: e.screenY
//   });
// };
// const handleDragLeave = (e: DragEvent) => {
//   e.preventDefault();
//   e.stopPropagation();
//   dragEntered = false;
// };
// const handleDragOver = (e: DragEvent) => {
//   e.preventDefault();
//   e.stopPropagation();
//   // 阻止默认行为，并设置鼠标样式为 "not-allowed" (禁止) 后来发现这个会影响所有窗口，包括拖拽上传窗口
//   // e.dataTransfer.dropEffect = 'none'; // 设置拖放操作的效果为 "none"
//   // e.currentTarget.style.cursor = 'not-allowed'; // 设置鼠标样式
// };
// const handleDrop = (e: DragEvent) => {
//   e.preventDefault();
//   e.stopPropagation();
//   dragEntered = false;
// };
onMounted(async () => {
  document.addEventListener('contextmenu', function(event) {
    event.preventDefault();
  });
  window.addEventListener('dragenter', handleDragEnter);
  window.addEventListener('dragleave', handleDragLeave);
  window.addEventListener('dragover', handleDragOver);
  window.addEventListener('drop', handleDrop);
})

// 暴露 store 实例
onUnmounted(async ()=>{
  unlisten();
  window.removeEventListener('dragenter', handleDragEnter);
  window.removeEventListener('dragleave', handleDragLeave);
  window.removeEventListener('dragover', handleDragOver);
  window.removeEventListener('drop', handleDrop);
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