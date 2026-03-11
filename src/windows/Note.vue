<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue"
import { useDialog, useMessage } from "naive-ui"
import {emitTo, listen, type UnlistenFn} from "@tauri-apps/api/event"
import {useNoteTabsStore} from "../stroe/note-tabs.ts";
import {NoteResponseDto} from "../components/main/main-type.ts";
import TitleBar from "../components/TitleBar.vue";
import NoteTabsBar from "../components/note/NoteTabsBar.vue"
import VditorNote from "../components/note/VditorNote.vue"


const dialog = useDialog()
const message = useMessage()
const noteTabsStore = useNoteTabsStore()

const tabs = computed(() => noteTabsStore.tabs)
const activeTab = computed(() => noteTabsStore.activeTab)
const activeTabId = computed(() => noteTabsStore.activeTabId)

let unlistenOpenNote: UnlistenFn | null = null

function handleChangeTab(tabId: string) {
  noteTabsStore.setActiveTab(tabId)
}

function handleUpdateTitle(value: string) {
  noteTabsStore.updateActiveDraft({ title: value })
}

function handleUpdateContent(value: string) {
  noteTabsStore.updateActiveDraft({ content: value })
}

async function fakeSaveApi(note: NoteResponseDto): Promise<NoteResponseDto> {
  await new Promise(resolve => setTimeout(resolve, 300))
  return {
    ...note,
    updated_at: new Date().toISOString()
  }
}

async function handleSaveActiveTab() {
  const tab = noteTabsStore.activeTab
  if (!tab) return

  try {
    noteTabsStore.markSaving(tab.tabId, true)
    const saved = await fakeSaveApi(tab.draft)
    noteTabsStore.replaceNote(saved)
    message.success("保存成功")
  } catch (error) {
    console.error(error)
    message.error("保存失败")
  } finally {
    noteTabsStore.markSaving(tab.tabId, false)
  }
}

function handleCloseTab(tabId: string) {
  const tab = noteTabsStore.tabs.find(item => item.tabId === tabId)
  if (!tab) return

  if (!tab.dirty) {
    noteTabsStore.closeTab(tabId)
    return
  }

  dialog.warning({
    title: "提示",
    content: "当前笔记尚未保存，是否保存后关闭？",
    positiveText: "保存并关闭",
    negativeText: "直接关闭",
    onPositiveClick: async () => {
      try {
        noteTabsStore.markSaving(tab.tabId, true)
        const saved = await fakeSaveApi(tab.draft)
        noteTabsStore.replaceNote(saved)
        noteTabsStore.closeTab(tabId)
        message.success("已保存并关闭")
      } catch (error) {
        console.error(error)
        message.error("保存失败")
      } finally {
        noteTabsStore.markSaving(tab.tabId, false)
      }
    },
    onNegativeClick: () => {
      noteTabsStore.closeTab(tabId)
      message.warning("已放弃未保存修改")
    }
  })
}

onMounted(async () => {
  console.log("DragImport mounted")
  unlistenOpenNote = await listen<NoteResponseDto>("open-note", async (event) => {
    noteTabsStore.openNote(event.payload)
  })
  await emitTo("main", "note-window-ready")
})

onUnmounted(() => {
  unlistenOpenNote?.()
})
</script>

<template>
  <TitleBar title="笔记" />

  <div class="note-window-container">
    <NoteTabsBar
        :tabs="tabs"
        :active-tab-id="activeTabId"
        @change="handleChangeTab"
        @close="handleCloseTab"
    />

    <div class="editor-container">
      <VditorNote
          v-if="activeTab"
          :note="activeTab.draft"
          @update:title="handleUpdateTitle"
          @update:content="handleUpdateContent"
          @save="handleSaveActiveTab"
      />

      <div v-else class="empty-state">
        <div class="empty-title">暂无打开的笔记</div>
        <div class="empty-desc">从笔记列表中打开一篇笔记开始编辑</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.note-window-container {
  height: calc(100vh - 30px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-container {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.empty-state {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #666;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 8px;
}

.empty-desc {
  font-size: 14px;
}
</style>