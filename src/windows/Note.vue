<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from "vue"
import { emitTo, listen, type UnlistenFn } from "@tauri-apps/api/event"
import { useNoteTabsStore } from "../stroe/note-tabs.ts"
import type { NoteResponseDto } from "../components/main/main-type.ts"
import TitleBar from "../components/note/NoteTitleBar.vue"
import NoteTabsBar from "../components/note/NoteTabsBar.vue"
import VditorNote from "../components/note/VditorNote.vue"
import { invoke } from "@tauri-apps/api/core"
import { message } from "../message.ts"
import CustomModal from "../util/CustomModal.vue"
import hotkeys from "hotkeys-js"
import useConfigStore from "../stroe/config.ts"

const noteTabsStore = useNoteTabsStore()
const configStore = useConfigStore()

const tabs = computed(() => noteTabsStore.tabs)
const activeTab = computed(() => noteTabsStore.activeTab)
const activeTabId = computed(() => noteTabsStore.activeTabId)

// 弹窗状态：记录“正在等待确认关闭”的 tabId
const showCloseConfirm = ref(false)
const pendingCloseTabId = ref<string | null>(null)

let unlistenOpenNote: UnlistenFn | null = null

function getTabById(tabId: string) {
  return noteTabsStore.tabs.find(item => item.tabId === tabId)
}

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
    }
    return await invoke<NoteResponseDto>("update_note", {
      dto: {
        id: note.id,
        document_id: note.document_id,
        title: note.title,
        content: note.content
      }
    })
  } catch (e) {
    message.error(String(e))
    throw e
  }
}

/**
 * 统一保存 tab
 * @param tabId 要保存的 tabId
 * @param options 保存后的附加行为
 */
async function saveTab(
    tabId: string,
    options?: {
      closeAfterSave?: boolean
      successMessage?: string
      emitSavedEvent?: boolean
    }
) {
  const tab = getTabById(tabId)
  if (!tab) return null

  const {
    closeAfterSave = false,
    successMessage = "保存成功",
    emitSavedEvent = false
  } = options || {}

  try {
    noteTabsStore.markSaving(tab.tabId, true)

    const oldNoteId = tab.noteId
    const saved = await saveNote(tab.draft)

    noteTabsStore.replaceNote(oldNoteId, saved)

    if (closeAfterSave) {
      noteTabsStore.closeTab(tabId)
    }

    if (emitSavedEvent) {
      await emitTo("main", "note-saved", saved)
    }

    message.success(successMessage)
    return saved
  } catch (error) {
    console.error(error)
    message.error("保存失败")
    throw error
  } finally {
    noteTabsStore.markSaving(tab.tabId, false)
  }
}

async function handleSaveActiveTab() {
  const tab = noteTabsStore.activeTab
  if (!tab) return

  await saveTab(tab.tabId, {
    successMessage: "保存成功",
    emitSavedEvent: true
  })
}

async function handleDeleteActiveTab() {
  const tab = noteTabsStore.activeTab
  if (!tab) return

  try {
    if (tab.noteId !== -1) {
      await invoke("delete_note", { id: tab.noteId })
      await emitTo("main", "note-deleted", {
        document_id: tab.draft.document_id
      })
    }
    noteTabsStore.closeTab(tab.tabId)
    message.success("删除成功")
  } catch (error) {
    console.error(error)
    message.error("删除失败")
  }
}

function handleCloseTab(tabId: string) {
  const tab = getTabById(tabId)
  if (!tab) return

  if (!tab.dirty) {
    noteTabsStore.closeTab(tabId)
    return
  }

  pendingCloseTabId.value = tabId
  showCloseConfirm.value = true
}

function resetCloseConfirmState() {
  showCloseConfirm.value = false
  pendingCloseTabId.value = null
}

async function handleConfirmSaveAndClose() {
  const tabId = pendingCloseTabId.value
  if (!tabId) return

  try {
    await saveTab(tabId, {
      closeAfterSave: true,
      successMessage: "已保存并关闭"
    })
  } finally {
    resetCloseConfirmState()
  }
}

function handleDirectClose() {
  const tabId = pendingCloseTabId.value
  if (!tabId) return

  noteTabsStore.closeTab(tabId)
  message.warning("已放弃未保存修改")
  resetCloseConfirmState()
}

onMounted(async () => {
  let currentActiveTabId: string | null = null
  
  unlistenOpenNote = await listen<NoteResponseDto>("open-note", async (event) => {
    const tab = noteTabsStore.openNote(event.payload)
    currentActiveTabId = tab.tabId
  })
  
  await emitTo("main", "note-window-ready")
  
  // 恢复上次打开的笔记
  const lastOpenedNoteIds = configStore.getLastOpenedNotes()
  if (lastOpenedNoteIds.length > 0) {
    console.log('恢复上次打开的笔记:', lastOpenedNoteIds)
    // 遍历每个笔记 ID，获取笔记并打开（不设置激活）
    for (const noteId of lastOpenedNoteIds) {
      try {
        const note = await invoke<NoteResponseDto>('get_note_by_id', { id: noteId })
        if (note) {
          noteTabsStore.openNote(note, { setActive: false })
        }
      } catch (e) {
        console.error(`恢复笔记 ${noteId} 失败:`, e)
      }
    }
  }
  
  // 如果有当前打开的笔记，确保焦点在它上面
  if (currentActiveTabId) {
    noteTabsStore.setActiveTab(currentActiveTabId)
  }
  
  // 监听保存笔记的快捷键
  watch(() => configStore.shortcuts, () => {
    const saveShortcut = configStore.getShortcutByName('保存笔记')
    if (!saveShortcut) return
    
    hotkeys.filter = function(_event){
      // 确保在输入框内也能触发快捷键
      return true;
    }
    
    hotkeys(saveShortcut, function (event) {
      event.preventDefault()
      console.log('保存笔记快捷键触发')
      handleSaveActiveTab()
    })
  }, { immediate: true, deep: true })
})

onUnmounted(() => {
  unlistenOpenNote?.()
  hotkeys.unbind()
})
</script>

<template>
  <TitleBar title="笔记" />

  <CustomModal
      v-model:show="showCloseConfirm"
      title="提示"
      :onConfirm="handleConfirmSaveAndClose"
  >
    <div>当前笔记尚未保存，是否保存后关闭？</div>

    <template #action>
      <n-flex justify="end">
        <n-button size="small" @click="handleDirectClose">
          直接关闭
        </n-button>
        <n-button
            size="small"
            type="success"
            class="text-black"
            @click="handleConfirmSaveAndClose"
        >
          保存并关闭
        </n-button>
      </n-flex>
    </template>
  </CustomModal>

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