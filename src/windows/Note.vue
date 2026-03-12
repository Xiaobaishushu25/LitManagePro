<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue"
import { useDialog } from "naive-ui"
import {emitTo, listen, type UnlistenFn} from "@tauri-apps/api/event"
import {useNoteTabsStore} from "../stroe/note-tabs.ts";
import {NoteResponseDto} from "../components/main/main-type.ts";
import TitleBar from "../components/TitleBar.vue";
import NoteTabsBar from "../components/note/NoteTabsBar.vue"
import VditorNote from "../components/note/VditorNote.vue"
import {invoke} from "@tauri-apps/api/core";
import {message} from "../message.ts";


const dialog = useDialog()
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

async function saveNote(note: NoteResponseDto): Promise<NoteResponseDto> {
  try {
    if (note.id === -1) {
      return await invoke<NoteResponseDto>("create_note", {
        dto: {
          document_id: note.document_id,
          title: note.title,
          content: note.content
        }
      })
    } else {
      return await invoke<NoteResponseDto>("update_note", {
        dto: {
          id: note.id,
          document_id: note.document_id,
          title: note.title,
          content: note.content
        }
      })
    }
  } catch (e) {
    message.error(String(e))
    throw e
  }
}

async function handleSaveActiveTab() {
  const tab = noteTabsStore.activeTab
  if (!tab) return
  try {
    const oldNoteId = tab.noteId
    noteTabsStore.markSaving(tab.tabId, true)
    const saved = await saveNote(tab.draft)
    noteTabsStore.replaceNote(oldNoteId, saved)
    message.success("保存成功")
    // 发送事件通知主窗口刷新笔记列表
    await emitTo("main", "note-saved", saved)
  } catch (error) {
    console.error(error)
    message.error("保存失败")
  } finally {
    noteTabsStore.markSaving(tab.tabId, false)
  }
}

async function handleDeleteActiveTab() {
  const tab = noteTabsStore.activeTab
  if (!tab) return
  try {
    // 如果是已存在的笔记（id !== -1），则调用后端删除
    if (tab.noteId !== -1) {
      await invoke('delete_note', { id: tab.noteId })
      // 发送事件通知主窗口刷新笔记列表（传递 document_id）
      await emitTo("main", "note-deleted", { document_id: tab.draft.document_id })
    }
    // 关闭标签页
    noteTabsStore.closeTab(tab.tabId)
    message.success("删除成功")
  } catch (error) {
    console.error(error)
    message.error("删除失败")
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
        const oldNoteId = tab.noteId
        const saved = await saveNote(tab.draft)
        noteTabsStore.replaceNote(oldNoteId, saved)
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
          @delete="handleDeleteActiveTab"
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