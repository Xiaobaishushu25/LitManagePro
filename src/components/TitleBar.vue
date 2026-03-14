<script setup lang="ts">
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { saveWindowState, StateFlags } from "@tauri-apps/plugin-window-state";
import { ref, onMounted, onUnmounted } from "vue";

const { title, showMaximize, showMinimize } = defineProps({
  title: {
    type: String,
    required: true
  },
  showMaximize: {
    type: Boolean,
    default: true
  },
  showMinimize: {
    type: Boolean,
    default: true
  }
});

const appWindow = WebviewWindow.getCurrent();
const isMaximized = ref(false);

let unlistenResize: (() => void) | null = null;

// 更新窗口状态
async function updateWindowStatus() {
  isMaximized.value = await appWindow.isMaximized();
}

onMounted(async () => {
  await updateWindowStatus();
  // 监听窗口大小变化
  unlistenResize = await appWindow.onResized(async () => {
    await updateWindowStatus();
  });
});

onUnmounted(() => {
  if (unlistenResize) {
    unlistenResize();
  }
});

async function window_minimize() {
  await appWindow.minimize();
}

async function window_maximize() {
  if (isMaximized.value) {
    await appWindow.unmaximize();
  } else {
    await appWindow.maximize();
  }
  await updateWindowStatus();
}

async function window_close() {
  await saveWindowState(StateFlags.ALL);
  await appWindow.close();
}
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="w-5"></div>
    <img src="../assets/icon/app.ico" class="w-4 h-4" alt="ICO Icon">
    <label class="pl-1">{{ title }}</label>
    <div class="ml-auto flex gap-0">
      <inline-svg
          v-if="showMinimize"
          src="../assets/svg/minimize.svg"
          class="window-button min"
          @click.left="window_minimize"
      />
      <inline-svg
          v-if="showMaximize"
          :src="isMaximized
          ? '../assets/svg/restore.svg'
          : '../assets/svg/maximize.svg'"
          class="window-button restore maximize"
          @click.left="window_maximize"
      />
      <inline-svg
          src="../assets/svg/close.svg"
          class="window-button close"
          @click.left="window_close"
      />
    </div>
  </div>
</template>

<style>
.title-bar{
  align-items: center;
  display: flex;
  flex-direction: row;
  height: 30px;
  user-select: none;
  background-color: #e8e8e5;
}

.window-button {
  width: 40px;
  height: 30px;
  outline: none;
}

.min path,
.maximize path,
.restore path,
.close path {
  transform-origin: center;
}

.min path {
  transform: scale(0.5);
}

.maximize path {
  transform: scale(0.6);
  stroke: #0f0f0f;
  stroke-width: 0.1;
}

.restore path {
  transform: scale(0.5);
  stroke-width: 0.2;
}

.close path {
  transform: scale(0.6);
  stroke-width: 20;
  stroke: #0f0f0f;
}

.min:hover,
.maximize:hover,
.restore:hover {
  background-color: #33303020;
}

.close:hover {
  background-color: red;
}

.close:hover path {
  fill: white;
  stroke: white;
}
</style>