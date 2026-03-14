<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue"
import { emitTo, listen, type UnlistenFn } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"
import type { NoteResponseDto } from "../components/main/main-type.ts"
import TitleBar from "../components/TitleBar.vue"
import { message } from "../message.ts"
import { openNoteWindow } from "../util/open-note-window.ts"
import CustomModal from "../util/CustomModal.vue"

const notes = ref<NoteResponseDto[]>([])
const loading = ref(false)
const showDeleteConfirm = ref(false)
const pendingDeleteNote = ref<NoteResponseDto | null>(null)

let unlistenReady: UnlistenFn | null = null

// 加载所有笔记
async function loadAllNotes() {
  loading.value = true
  try {
    notes.value = await invoke<NoteResponseDto[]>("get_all_notes")
  } catch (error) {
    message.error(`加载笔记列表失败：${error}`)
    console.error(error)
  } finally {
    loading.value = false
  }
}

// 打开笔记
async function handleOpenNote(note: NoteResponseDto) {
  try {
    await openNoteWindow(note)
  } catch (error) {
    message.error(`打开笔记失败：${error}`)
    console.error(error)
  }
}

// 打开文档
async function handleOpenDocument(note: NoteResponseDto) {
  try {
    await invoke('open_note_document', { documentId: note.document_id })
  } catch (error) {
    message.error(`打开文档失败：${error}`)
    console.error(error)
  }
}

// 删除笔记
function handleDeleteNote(note: NoteResponseDto) {
  pendingDeleteNote.value = note
  showDeleteConfirm.value = true
}

async function confirmDelete() {
  if (!pendingDeleteNote.value) return
  
  try {
    await invoke('delete_note', { id: pendingDeleteNote.value.id })
    // 通知主窗口笔记已删除，需要刷新列表
    await emitTo('main', 'note-deleted', {
      document_id: pendingDeleteNote.value.document_id
    })
    // 重新加载笔记列表
    await loadAllNotes()
    message.success('删除成功')
  } catch (error) {
    message.error(`删除失败：${error}`)
    console.error(error)
  } finally {
    pendingDeleteNote.value = null
  }
}

function resetDeleteState() {
  pendingDeleteNote.value = null
}

onMounted(async () => {
  await loadAllNotes()
  
  // 监听主窗口发来的刷新事件
  unlistenReady = await listen("refresh-note-list", async () => {
    await loadAllNotes()
  })
  
  // 通知主窗口已准备就绪
  await emitTo("main", "note-list-ready")
})

onUnmounted(() => {
  unlistenReady?.()
})
</script>

<template>
  <div class="note-list-window">
    <TitleBar title="笔记列表" :show-maximize="true" :show-minimize="true" />
    
    <!-- 删除确认弹窗 -->
    <CustomModal
        v-model:show="showDeleteConfirm"
        title="提示"
        :on-confirm="confirmDelete"
    >
      <div class="delete-confirm-content">
        <p>确定要删除笔记：<span class="text-orange-600 text-base">{{ pendingDeleteNote?.title || '无标题笔记' }}</span>？</p>
        <p class="text-gray-500 text-sm mt-2">删除后不可恢复，请谨慎操作。</p>
      </div>

      <template #action>
        <n-flex justify="end">
          <n-button size="small" @click="resetDeleteState">取消</n-button>
          <n-button
              size="small"
              type="error"
              class="text-white"
              @click="confirmDelete"
          >
            删除
          </n-button>
        </n-flex>
      </template>
    </CustomModal>
    
    <div class="note-list-container">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-state">
        <n-spin size="large" description="加载中..." />
      </div>
      
      <!-- 空状态 -->
      <div v-else-if="notes.length === 0" class="empty-state">
        <div class="empty-title">暂无笔记</div>
        <div class="empty-desc">还没有创建任何笔记</div>
      </div>
      
      <!-- 笔记列表 -->
      <n-list v-else hoverable clickable class="note-list">
        <n-list-item
          v-for="note in notes"
          :key="note.id"
          @click="handleOpenNote(note)"
          class="note-item"
        >
          <template #prefix>
            <n-icon size="20" color="#18a058">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14z"/>
                <path d="M7 7h10v2H7zm0 4h10v2H7zm0 4h7v2H7z"/>
              </svg>
            </n-icon>
          </template>
          
          <div class="note-content">
            <div class="note-title">{{ note.title || "无标题" }}</div>
            <div class="note-meta">
              <span class="note-time">
                {{ new Date(note.updated_at).toLocaleString('zh-CN') }}
              </span>
            </div>
          </div>
          
          <template #suffix>
            <div class="note-actions">
              <n-button text type="primary" size="small" @click.stop="handleOpenDocument(note)">
                打开文档
              </n-button>
              <n-divider vertical />
              <n-button text type="primary" size="small">
                打开
              </n-button>
              <n-divider vertical />
              <n-button text type="error" size="small" @click.stop="handleDeleteNote(note)">
                删除
              </n-button>
            </div>
          </template>
        </n-list-item>
      </n-list>
    </div>
  </div>
</template>

<style scoped>
.note-list-window {
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.note-list-container {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  padding: 8px;
}

.loading-state {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
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

.note-list {
  height: 100%;
  overflow-y: auto;
}

.note-item {
  margin-bottom: 4px;
  border-radius: 4px;
  transition: all 0.2s;
}

.note-item:hover {
  background-color: #f5f5f5;
}

.note-content {
  flex: 1;
  min-width: 0;
}

.note-title {
  font-size: 14px;
  font-weight: 500;
  color: #333;
  margin-bottom: 4px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.note-meta {
  font-size: 12px;
  color: #999;
}

.note-time {
  margin-right: 8px;
}

.note-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.delete-confirm-content {
  text-align: center;
}

.delete-confirm-content p {
  margin: 0 0 16px 0;
  color: #666;
}

.note-title-highlight {
  background-color: #fef0f0;
  border: 1px solid #fde2e2;
  border-radius: 4px;
  padding: 12px 16px;
  font-size: 15px;
  font-weight: 500;
  color: #f56c6c;
  text-align: center;
  word-break: break-all;
}
</style>
