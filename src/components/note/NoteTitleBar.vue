<script setup lang="ts">
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useNoteTabsStore } from "../../stroe/note-tabs";
import CustomModal from "../../util/CustomModal.vue";
import useConfigStore from "../../stroe/config.ts";

// 这个 TitleBar 是笔记窗口专用的，关闭时会检查是否有未保存的笔记

// 定义 props，接收来自父组件的标题和配置
const { title, showMaximize, showMinimize }= defineProps({
  title: {
    type: String,
    required: true
  },
  // 是否显示最大化按钮，默认 true
  showMaximize: {
    type: Boolean,
    default: true
  },
  // 是否显示最小化按钮，默认 true
  showMinimize: {
    type: Boolean,
    default: true
  }
});

const noteTabsStore = useNoteTabsStore();
const configStore = useConfigStore()

// 检查是否有未保存的笔记
const hasUnsavedNotes = computed(() => {
  return noteTabsStore.tabs.some(tab => tab.dirty);
});

// 窗口状态
const isMaximized = ref(false);

// 关闭确认弹窗
const showCloseConfirm = ref(false);

// 更新窗口状态
async function updateWindowStatus() {
  const currentWindow = WebviewWindow.getCurrent();
  isMaximized.value = await currentWindow.isMaximized();
}

onMounted(async ()=>{
  // 初始化窗口状态
  await updateWindowStatus();

  // 监听窗口大小变化来改变窗口最大化状态，主要是用于鼠标点击标题栏拖拽时会改变最大化状态的监听
  window.addEventListener('resize', () => {
    const isCurrentlyMaximized = isWindowMaximized();
    if (isCurrentlyMaximized) {
      isMaximized.value = true;
    } else if (!isCurrentlyMaximized) {
      isMaximized.value = false;
    }
  });
})

onUnmounted(()=>{
  window.removeEventListener('resize', updateWindowStatusOnResize);
})

// 判断窗口是否处于最大化状态
function isWindowMaximized() {
  // 获取窗口的内宽度和屏幕的宽度
  const innerWidth = window.innerWidth;
  const screenWidth = window.screen.width;
  // 获取窗口的外宽度（包括边框）
  const outerWidth = window.outerWidth;
  // 如果窗口的外宽度等于屏幕宽度，并且内宽度等于外宽度减去浏览器边框宽度 则认为窗口处于最大化状态
  return outerWidth === screenWidth && innerWidth === outerWidth;
}

function updateWindowStatusOnResize() {
  updateWindowStatus();
}

async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
}

async function window_maximize(){
  const currentWindow = WebviewWindow.getCurrent();
  if (isMaximized.value) {
    await currentWindow.unmaximize();
  } else {
    await currentWindow.maximize();
  }
  await updateWindowStatus();
}

// 点击关闭按钮时，先检查是否有未保存的笔记
async function handleWindowClose(){
  if (hasUnsavedNotes.value) {
    // 有未保存的笔记，显示确认弹窗
    showCloseConfirm.value = true;
  } else {
    // 窗口关闭时，保存最后打开的笔记
    const openedNoteIds = noteTabsStore.openedNoteIds
    if (openedNoteIds && openedNoteIds.length > 0) {
      configStore.saveLastOpenedNotes(openedNoteIds)
      console.log('已保存最后打开的笔记:', openedNoteIds)
    } else {
      // 如果没有打开的笔记，清空保存的列表
      configStore.saveLastOpenedNotes([])
    }
    // 没有未保存的笔记，直接关闭
    await performClose();
  }
}

// 执行关闭窗口
async function performClose(){
  await saveWindowState(StateFlags.ALL);
  await WebviewWindow.getCurrent().close();
}

// 确认关闭（不保存）
async function confirmCloseWithoutSave(){
  showCloseConfirm.value = false;
  await performClose();
}

</script>


<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="w-5"></div> <!-- 占位符，不知道为什么给 app.ico 设置左边距不好使，直接用这个占空了 -->
    <img src="../../assets/icon/app.ico" class="w-4 h-4" alt="ICO Icon">
    <label class="pl-1">{{title}}</label>
    <div class="ml-auto flex gap-0">
      <!-- 最小化按钮 -->
      <inline-svg
          v-if="showMinimize"
          src="../assets/svg/minimize.svg"
          class="window-button min"
          @click.left="window_minimize"
      ></inline-svg>
      <!-- 最大化/还原按钮 -->
      <inline-svg
          v-if="showMaximize"
          :src="isMaximized ? '../assets/svg/restore.svg' : '../assets/svg/maximize.svg'"
          class="window-button restore maximize"
          @click.left="window_maximize"
      ></inline-svg>
      <!-- 关闭按钮 -->
      <inline-svg
          src="../assets/svg/close.svg"
          class="window-button close"
          @click.left="handleWindowClose"
      ></inline-svg>
    </div>
  </div>

  <!-- 关闭确认弹窗 -->
  <!-- 关闭确认弹窗 -->
  <CustomModal
      v-model:show="showCloseConfirm"
      title="提示"
      :onConfirm="confirmCloseWithoutSave"
  >
    <div>当前有未保存的笔记，确定要直接关闭吗？未保存的内容将会丢失。</div>
  </CustomModal>
</template>

<style>
.title-bar{
  align-items: center; /* 垂直居中 */
  display: flex;
  flex-direction:row;
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
  stroke: #0f0f0b;
  stroke-width: 0.1;
}

.restore path {
  transform: scale(0.5);
  stroke-width: 0.2;
}

.close path {
  transform: scale(0.6);
  stroke-width: 20;
  stroke: #0f0f0b;
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