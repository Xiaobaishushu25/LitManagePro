<script setup lang="ts">
/**
 * ------------------------------------------------------------
 * TitleBar Window Control & Safe Close Logic
 * ------------------------------------------------------------
 * 该组件负责：
 * 1. 自定义窗口 TitleBar（最小化 / 最大化 / 关闭）
 * 2. 保存窗口状态（位置、大小等）
 * 3. 保存最后打开的笔记
 * 4. 拦截所有窗口关闭行为，防止未保存内容丢失
 * ------------------------------------------------------------
 * 关闭拦截机制
 * ------------------------------------------------------------
 *
 * Tauri 中所有关闭操作都会触发 `onCloseRequested` 事件，例如：
 * - 点击系统关闭按钮
 * - Alt + F4
 * - Dock / 任务栏关闭
 * - 调用 `window.close()`
 *
 * 因此我们统一在 `onCloseRequested` 中进行拦截。
 * 关闭流程如下：
 * 用户关闭窗口
 *        │
 *        ▼
 * onCloseRequested 触发
 *        │
 *        ├── 如果 forceClose = true
 *        │      说明是程序主动关闭 → 允许关闭
 *        │
 *        └── 如果存在未保存笔记
 *               │
 *               ├─ event.preventDefault() 阻止关闭
 *               ├─ bringWindowToFront()
 *               │      (防止窗口最小化或在后台)
 *               └─ 显示确认弹窗
 *
 * 用户点击确认关闭
 *        │
 *        ▼
 * confirmCloseWithoutSave()
 *        │
 *        ├─ forceClose = true
 *        ├─ 保存窗口状态
 *        ├─ 保存最后打开笔记
 *        └─ currentWindow.close()
 *
 * 再次触发 onCloseRequested
 *        │
 *        ▼
 * 因为 forceClose = true
 *        │
 *        ▼
 * 允许窗口真正关闭
 * ------------------------------------------------------------
 * forceClose 作用
 * ------------------------------------------------------------
 * 用于区分：
 * - 用户触发关闭（需要拦截）
 * - 程序主动关闭（允许关闭）
 *
 * 因为 `currentWindow.close()` 也会再次触发 `onCloseRequested`，
 * 如果没有 forceClose 标志，会导致无限拦截循环。
 * ------------------------------------------------------------
 * bringWindowToFront()
 * ------------------------------------------------------------
 * 当用户从系统层关闭窗口时（例如 Alt+F4 / 任务栏关闭）：
 * 如果窗口当前：
 * - 被最小化
 * - 在后台
 *
 * 用户可能看不到弹窗。
 * 因此需要：
 * - unminimize()
 * - show()
 * - setFocus()
 * 强制将窗口恢复并置于前台。
 * ------------------------------------------------------------
 * 保存数据
 * ------------------------------------------------------------
 * 在真正关闭窗口前执行：
 * 1. saveOpenedNotes()
 *    保存当前打开的笔记 ID
 * 2. saveWindowState(StateFlags.ALL)
 *    保存窗口位置、大小、最大化状态
 * 这样下次启动可以恢复用户工作环境。
 *
 * ------------------------------------------------------------
 * 维护提示
 * ------------------------------------------------------------
 * 如果未来修改关闭逻辑，需要注意：
 * 1. 不要移除 forceClose 机制
 *    否则会导致 close 事件死循环
 * 2. 所有关闭路径必须统一走 closeWindow()
 * 3. 如果增加自动保存逻辑，需要在 closeWindow() 之前执行
 *
 * ------------------------------------------------------------
 */
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { saveWindowState, StateFlags } from "@tauri-apps/plugin-window-state";
import { ref, onMounted, onUnmounted, computed } from "vue";
import { useNoteTabsStore } from "../../stroe/note-tabs";
import useConfigStore from "../../stroe/config";
import CustomModal from "../../util/CustomModal.vue";

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

const noteTabsStore = useNoteTabsStore();
const configStore = useConfigStore();

const forceClose = ref(false);

const currentWindow = WebviewWindow.getCurrent();

let unlistenClose: (() => void) | null = null;

const isMaximized = ref(false);
const showCloseConfirm = ref(false);

const hasUnsavedNotes = computed(() => {
  return noteTabsStore.tabs.some(tab => tab.dirty);
});

/* ---------------- window state ---------------- */
//用于判断当前是不是最大化的窗口
async function updateWindowState() {
  isMaximized.value = await currentWindow.isMaximized();
}

function handleResize() {
  updateWindowState();
}

/* ---------------- window actions ---------------- */
async function windowMinimize() {
  await currentWindow.minimize();
}
async function windowMaximize() {
  if (isMaximized.value) {
    await currentWindow.unmaximize();
  } else {
    await currentWindow.maximize();
  }
  await updateWindowState();
}
/* ---------------- save opened notes ---------------- */
function saveOpenedNotes() {
  const openedNoteIds = noteTabsStore.openedNoteIds;
  if (openedNoteIds?.length) {
    configStore.saveLastOpenedNotes(openedNoteIds);
    console.log("保存最后打开的笔记:", openedNoteIds);
  } else {
    configStore.saveLastOpenedNotes([]);
  }
}

/* ---------------- close logic ---------------- */
async function closeWindow() {
  saveOpenedNotes();
  forceClose.value = true;
  await saveWindowState(StateFlags.ALL);
  await currentWindow.close();
}

async function handleWindowClose() {
  if (hasUnsavedNotes.value) {
    showCloseConfirm.value = true;
    return;
  }
  await closeWindow();
}
async function confirmCloseWithoutSave() {
  showCloseConfirm.value = false;
  forceClose.value = true;
  await closeWindow();
}

async function bringWindowToFront() {
  const minimized = await currentWindow.isMinimized();

  if (minimized) {
    await currentWindow.unminimize();
  }

  await currentWindow.show();
  await currentWindow.setFocus();
}

/* ---------------- lifecycle ---------------- */
onMounted(async () => {
  await updateWindowState();
  window.addEventListener("resize", handleResize);
  unlistenClose = await currentWindow.onCloseRequested(
      async (event) => {
        // 如果是程序主动关闭，不拦截
        if (forceClose.value) {
          return;
        }
        if (hasUnsavedNotes.value) {
          event.preventDefault();
          await bringWindowToFront();
          showCloseConfirm.value = true;
        } else {
          forceClose.value = true;
          await closeWindow();
        }
      }
  );
});

onUnmounted(() => {
  window.removeEventListener("resize", handleResize);
  if (unlistenClose) {
    unlistenClose();
  }
});
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="w-5"></div>
    <img src="../../assets/icon/app.ico" class="w-4 h-4" />
    <label class="pl-1">{{ title }}</label>
    <div class="ml-auto flex gap-0">
      <inline-svg
          v-if="showMinimize"
          src="../assets/svg/minimize.svg"
          class="window-button min"
          @click.left="windowMinimize"
      />
      <inline-svg
          v-if="showMaximize"
          :src="isMaximized
          ? '../assets/svg/restore.svg'
          : '../assets/svg/maximize.svg'"
          :class="[ 'window-button', isMaximized ? 'restore' : 'maximize' ]"
          @click.left="windowMaximize"
      />
      <inline-svg
          src="../assets/svg/close.svg"
          class="window-button close"
          @click.left="handleWindowClose"
      />
    </div>

  </div>

  <!-- 关闭确认弹窗 -->
  <CustomModal
      v-model:show="showCloseConfirm"
      title="提示"
      :onConfirm="confirmCloseWithoutSave"
  >
    <div>当前有未保存的笔记，确定要直接关闭吗？未保存的内容将会丢失。</div>
  </CustomModal>
</template>

<style scoped>

</style>