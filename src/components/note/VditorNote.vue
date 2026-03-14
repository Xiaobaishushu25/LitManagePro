<script setup lang="ts">
import {ref, onMounted, onUnmounted, watch, nextTick, onActivated, onDeactivated,
} from "vue"
import Vditor from "vditor"
import "vditor/dist/index.css"
import "katex/dist/katex.min.css"
import { NButton, NInput } from "naive-ui"
import {NoteResponseDto} from "../main/main-type.ts"
import CustomModal from "../../util/CustomModal.vue"
import { invoke } from "@tauri-apps/api/core"
import { message } from "../../message"
import useConfigStore from "../../stroe/config.ts";

const props = defineProps<{
  note: NoteResponseDto | null
}>()

const emit = defineEmits<{
  (e: "update:title", value: string): void
  (e: "update:content", value: string): void
  (e: "save"): void
  (e: "delete"): void
}>()

const configStore = useConfigStore()

const editorRef = ref<HTMLElement | null>(null)
const vditor = ref<Vditor | null>(null)
const content = ref("")
const localTitle = ref("")
const currentNoteId = ref<number | null>(null)
const showDeleteModal = ref(false)

let isSettingValue = false
let mathInsertLock = false
let isSwitchingMode = false // 防止模式切换时触发 input 事件

function buildCacheKey(noteId: number | null) {
  return noteId ? `vditor-note-cache-${noteId}` : "vditor-note-cache-empty"
}
function formatTime(time?: string) {
  if (!time) return "-"
  return new Date(time).toLocaleString()
}
function syncEditorFromProps(note: NoteResponseDto | null) {
  if (!note) {
    localTitle.value = ""
    content.value = ""
    currentNoteId.value = null
    vditor.value?.setValue("")
    return
  }

  currentNoteId.value = note.id
  localTitle.value = note.title ?? ""
  content.value = note.content ?? ""

  if (vditor.value) {
    const current = vditor.value.getValue()
    if (current !== note.content) {
      isSettingValue = true
      vditor.value.setValue(note.content ?? "")
      setTimeout(() => {
        isSettingValue = false
      }, 0)
    }
  }
}

function insertMath() {
  if (!vditor.value || mathInsertLock) return
  mathInsertLock = true
  setTimeout(() => {
    mathInsertLock = false
  }, 120)
  const selected = vditor.value.getSelection()
  if (selected && selected.trim()) {
    vditor.value.deleteValue()
    vditor.value.insertValue(`$${selected}$`)
  } else {
    vditor.value.insertValue("$$\n\n$$")
  }
}

function handleSave() {
  emit("save")
}
function handleDelete() {
  showDeleteModal.value = true
}
function handleOpenDocument() {
  if (!props.note?.document_id) {
    message.error("笔记未关联文档")
    return
  }
  invoke('open_note_document', { documentId: props.note.document_id })
      .then(() => {
        message.success("已打开文档")
      })
      .catch((err) => {
        message.error(err)
      })
}

function confirmDelete() {
  emit("delete")
}
//
type EditorPositionState = {
  scrollTop: number
  selectionStart: number
  selectionEnd: number
  hadFocus: boolean
  updatedAt: number
}

let positionSaveTimer: number | null = null
let unbindPositionEvents: (() => void) | null = null
let restoreToken = 0

function buildPositionKey(noteId: number | null) {
  return noteId ? `vditor-note-position-${noteId}` : "vditor-note-position-empty"
}

// 你当前固定 mode: "ir"，所以直接拿 IR 的编辑区
function getIrEditorElement(): HTMLPreElement | null {
  return (
      editorRef.value?.querySelector(
          ".vditor-ir > pre.vditor-reset[contenteditable='true']"
      ) ?? null
  )
}

// 计算当前选区在整篇内容里的 start/end 偏移
function getSelectionOffsets(editor: HTMLElement) {
  const selection = window.getSelection()
  if (!selection || selection.rangeCount === 0) {
    return { start: 0, end: 0 }
  }

  const range = selection.getRangeAt(0)
  const container = range.commonAncestorContainer

  if (!(container === editor || editor.contains(container))) {
    return { start: 0, end: 0 }
  }

  const preSelectionRange = range.cloneRange()

  if (editor.childNodes[0] && editor.childNodes[0].childNodes[0]) {
    preSelectionRange.setStart(editor.childNodes[0].childNodes[0], 0)
  } else {
    preSelectionRange.selectNodeContents(editor)
  }

  preSelectionRange.setEnd(range.startContainer, range.startOffset)

  const start = preSelectionRange.toString().length
  return {
    start,
    end: start + range.toString().length,
  }
}

// 按字符偏移恢复选区/光标
function setSelectionOffsets(editor: HTMLElement, start: number, end: number = start) {
  start = Math.max(0, start)
  end = Math.max(0, end)

  if (!editor.childNodes.length) {
    const emptyRange = document.createRange()
    emptyRange.selectNodeContents(editor)
    emptyRange.collapse(false)

    const emptySelection = window.getSelection()
    emptySelection?.removeAllRanges()
    emptySelection?.addRange(emptyRange)
    return
  }

  let charIndex = 0
  let line = 0
  let pNode: ChildNode | null = editor.childNodes[line] ?? null
  let foundStart = false
  let stop = false

  const range = document.createRange()
  range.setStart(pNode || editor, 0)
  range.collapse(true)

  while (!stop && pNode) {
    const textLength = pNode.textContent?.length ?? 0
    const nextCharIndex = charIndex + textLength

    if (!foundStart && start >= charIndex && start <= nextCharIndex) {
      if (start === 0) {
        range.setStart(pNode, 0)
      } else {
        const firstChild = pNode.childNodes[0]
        if (firstChild && firstChild.nodeType === Node.TEXT_NODE) {
          range.setStart(firstChild, start - charIndex)
        } else if (pNode.nextSibling) {
          range.setStartBefore(pNode.nextSibling)
        } else {
          range.setStartAfter(pNode)
        }
      }

      foundStart = true

      if (start === end) {
        stop = true
        break
      }
    }

    if (foundStart && end >= charIndex && end <= nextCharIndex) {
      if (end === 0) {
        range.setEnd(pNode, 0)
      } else {
        const firstChild = pNode.childNodes[0]
        if (firstChild && firstChild.nodeType === Node.TEXT_NODE) {
          range.setEnd(firstChild, end - charIndex)
        } else if (pNode.nextSibling) {
          range.setEndBefore(pNode.nextSibling)
        } else {
          range.setEndAfter(pNode)
        }
      }
      stop = true
    }

    charIndex = nextCharIndex
    line += 1
    pNode = editor.childNodes[line] ?? null
  }

  if (!stop) {
    range.selectNodeContents(editor)
    range.collapse(false)
  }

  const selection = window.getSelection()
  selection?.removeAllRanges()
  selection?.addRange(range)
}

function saveEditorPosition(noteId: number | null = currentNoteId.value) {
  if (!noteId) return

  const editor = getIrEditorElement()
  if (!editor) return

  const { start, end } = getSelectionOffsets(editor)

  const state: EditorPositionState = {
    scrollTop: editor.scrollTop,
    selectionStart: start,
    selectionEnd: end,
    hadFocus: document.activeElement === editor,
    updatedAt: Date.now(),
  }

  localStorage.setItem(buildPositionKey(noteId), JSON.stringify(state))
}

function scheduleSaveEditorPosition(noteId: number | null = currentNoteId.value) {
  if (!noteId) return

  if (positionSaveTimer) {
    window.clearTimeout(positionSaveTimer)
  }

  positionSaveTimer = window.setTimeout(() => {
    saveEditorPosition(noteId)
  }, 120)
}

async function waitEditorStable() {
  await nextTick()
  await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()))
  await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()))
}

async function restoreEditorPosition(noteId: number | null = currentNoteId.value) {
  if (!noteId) return
  const editor = getIrEditorElement()
  if (!editor) return
  const raw = localStorage.getItem(buildPositionKey(noteId))
  if (!raw) return
  let state: EditorPositionState
  try {
    state = JSON.parse(raw) as EditorPositionState
  } catch {
    return
  }
  // 先恢复阅读位置
  editor.scrollTop = state.scrollTop ?? 0
  // 上次离开时在编辑，才恢复光标/选区
  if (state.hadFocus) {
    editor.focus()
    setSelectionOffsets(
        editor,
        state.selectionStart ?? 0,
        state.selectionEnd ?? state.selectionStart ?? 0
    )
  }
  // focus / selection 有可能把滚动条再带跑一次，所以最后再写回一次
  requestAnimationFrame(() => {
    editor.scrollTop = state.scrollTop ?? 0
  })
}

function bindEditorPositionEvents() {
  const editor = getIrEditorElement()
  if (!editor) return
  const handler = () => scheduleSaveEditorPosition()
  editor.addEventListener("scroll", handler, { passive: true })
  editor.addEventListener("input", handler)
  editor.addEventListener("keyup", handler)
  editor.addEventListener("mouseup", handler)
  unbindPositionEvents = () => {
    editor.removeEventListener("scroll", handler)
    editor.removeEventListener("input", handler)
    editor.removeEventListener("keyup", handler)
    editor.removeEventListener("mouseup", handler)
  }
}
function handleVisibilityChange() {
  if (document.visibilityState === "hidden") {
    saveEditorPosition()
  }
}
function handlePageHide() {
  saveEditorPosition()
}

onMounted(async () => {
  await nextTick()
  vditor.value = new Vditor(editorRef.value!, {
    cdn: '/vditor',// 使用本地资源（把 node_modules 中 vditor 的 dist 复制到本地保存），不然 dev 模式下没问题，但是打包后编辑器找不到资源
    height: "100%",
    minHeight: 500,
    mode: configStore.config?.ui_config.editor_mode || "ir",
    theme: "classic",
    icon: "material",
    placeholder: "开始写 Markdown / LaTeX / Mermaid ...",
    cache: {
      enable: false
    },
    preview: {
      delay: 200,
      actions: [],
      markdown: {
        toc: true
      },
      hljs: {
        style: "github"
      },
      math: {
        engine: "KaTeX"
      },
      // mermaid: {
      //   enable: true
      // }
    },
    toolbarConfig: {
      pin: true,
    },
    toolbar: [
      { name: "emoji", tipPosition: "s" },
      { name: "headings", tipPosition: "s" },
      { name: "bold", tipPosition: "s" },
      { name: "italic", tipPosition: "s" },
      { name: "strike", tipPosition: "s" },
      "|",
      { name: "list", tipPosition: "s" },
      { name: "ordered-list", tipPosition: "s" },
      { name: "check", tipPosition: "s" },
      "|",
      { name: "quote", tipPosition: "s" },
      { name: "code", tipPosition: "s" },
      { name: "inline-code", tipPosition: "s" },
      "|",
      {
        name: "math",
        tip: "插入公式",
        icon: `<svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
      <path d="M18 5H7l6 7-6 7h11v-2H11.5l4.8-5-4.8-5H18z"/>
    </svg>`,
        click() {
          insertMath()
        },
        tipPosition: "s"
      },
      {
        name: "mermaid",
        tip: "插入Mermaid",
        icon: `<svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <rect x="3" y="4" width="7" height="5" rx="1"></rect>
      <rect x="14" y="4" width="7" height="5" rx="1"></rect>
      <rect x="8.5" y="15" width="7" height="5" rx="1"></rect>
      <path d="M10 6.5h4"></path>
      <path d="M12 9v6"></path>
    </svg>`,
        click() {
          vditor.value?.insertValue(
              ` \`\`\`mermaid
graph TD
A[开始] --> B{判断}
B -->|Yes| C[执行]
B -->|No| D[结束]
\`\`\`
`
          )
        },
        tipPosition: "s"
      },
      "|",
      { name: "link", tipPosition: "s" },
      { name: "table", tipPosition: "s" },
      { name: "upload", tipPosition: "s" },
      "|",
      { name: "undo", tipPosition: "s" },
      { name: "redo", tipPosition: "s" },
      "|",
      { name: "outline", tipPosition: "s" },
      {
        name: "edit-mode",
        tipPosition: "s",
      },
      { name: "preview", tipPosition: "s" },
      { name: "export", tipPosition: "s" }
    ],
    customWysiwygToolbar: ()=>{},//https://www.bing.com/ck/a?!&&p=ea37f182de46ffd6d9e879d4eb1c3ddfbb15a2face024554e1a76a200843afb5JmltdHM9MTc3MzQ0NjQwMA&ptn=3&ver=2&hsh=4&fclid=2c76c421-e1b1-68fa-1426-d6e7e0d769b9&psq=Uncaught+TypeError%3a+vditor.options.customWysiwygToolbar+is+not+a+function&u=a1aHR0cHM6Ly9naXRodWIuY29tL1ZhbmVzc2EyMTkvdmRpdG9yL2lzc3Vlcy8xODY1
    upload: {
      url: "/api/upload",
      accept: "image/*",
      max: 5 * 1024 * 1024
    },
    input(value) {
      if (isSettingValue || isSwitchingMode) return
      content.value = value
      emit("update:content", value)
      if (currentNoteId.value) {
        localStorage.setItem(buildCacheKey(currentNoteId.value), value)
      }
      scheduleSaveEditorPosition(currentNoteId.value)
    },
    focus() {
      scheduleSaveEditorPosition(currentNoteId.value)
    },
    blur() {
      scheduleSaveEditorPosition(currentNoteId.value)
    },
    after: async () => {
      syncEditorFromProps(props.note)
      bindEditorPositionEvents()
      await waitEditorStable()
      await restoreEditorPosition(props.note?.id ?? null)
      //监听模式切换
      const handler = (e: MouseEvent) => {
        const target = e.target as HTMLElement | null
        if (!target) return
        // 这里只能尽量模糊匹配，具体 class/data-attr 得看实际 DOM，应该是[data-type="edit-mode"]
        const clickedModeEntry =
            target.closest('[data-type="edit-mode"]') ||
            target.closest(".vditor-panel") ||
            target.closest(".vditor-hint")
        if (!clickedModeEntry) return
        setTimeout(() => {
          const currentMode = vditor.value?.getCurrentMode()
          if (!currentMode) return
          configStore.config!.ui_config.editor_mode = currentMode
        }, 0)
      }
      document.addEventListener("click", handler, true)
    },
  })
  console.log("Vditor 初始化完成")

})

watch(
    () => props.note?.id,
    async (newId, oldId) => {
      // 切走旧 note 前先保存旧 note 的位置
      if (oldId) {
        saveEditorPosition(oldId)
      }
      syncEditorFromProps(props.note)
      // 顺手记住“最后打开的是哪篇”
      if (newId) {
        localStorage.setItem("note-last-open-id", String(newId))
      }
      if (!newId || !vditor.value) return

      const token = ++restoreToken
      await waitEditorStable()

      // 防止快速切 note 产生旧 restore 覆盖新 restore
      if (token !== restoreToken) return

      await restoreEditorPosition(newId)
    },
    { immediate: true }
)

watch(
    () => props.note?.title,
    (value) => {
      localTitle.value = value ?? ""
    }
)

watch(
    () => props.note?.content,
    (value) => {
      if (!vditor.value) return

      const nextValue = value ?? ""
      const currentValue = vditor.value.getValue()

      if (currentValue !== nextValue) {
        isSettingValue = true
        vditor.value.setValue(nextValue)

        requestAnimationFrame(() => {
          isSettingValue = false
        })
      }
    }
)

watch(localTitle, (value) => {
  emit("update:title", value)
})

watch(localTitle, (value) => {
  emit("update:title", value)
})

onDeactivated(() => {
  saveEditorPosition()
})

onActivated(async () => {
  if (!currentNoteId.value) return
  await waitEditorStable()
  await restoreEditorPosition(currentNoteId.value)
})

onUnmounted(() => {
  saveEditorPosition()

  if (positionSaveTimer) {
    window.clearTimeout(positionSaveTimer)
  }

  unbindPositionEvents?.()
  document.removeEventListener("visibilitychange", handleVisibilityChange)
  window.removeEventListener("pagehide", handlePageHide)

  vditor.value?.destroy()
})

onUnmounted(() => {
  vditor.value?.destroy()
})
</script>

<template>
  <div class="editor-wrapper">

    <!-- 编辑器 -->
    <div ref="editorRef" class="editor-host"></div>

    <!-- 底部工具栏 -->
    <div class="editor-toolbar">
      <div class="title-area">
        <NInput
            v-model:value="localTitle"
            placeholder="请输入标题"
            clearable
            size="small"
        />
      </div>
      <div class="meta-info">
        <span>创建：{{ formatTime(note?.created_at) }}</span>
        <span>修改：{{ formatTime(note?.updated_at) }}</span>
        <span class="word-count">字数：{{ content.length }}</span>
      </div>

      <!-- 按钮 -->
      <div class="actions">
        <NButton type="info" size="small" @click="handleOpenDocument">
          打开文档
        </NButton>
        <NButton type="error" size="small" @click="handleDelete">
          删除
        </NButton>
        <NButton type="primary" size="small" @click="handleSave">
          保存
        </NButton>
      </div>

    </div>

    <!-- 删除确认模态框 -->
    <CustomModal
        v-model:show="showDeleteModal"
        title="删除笔记"
        :onConfirm="confirmDelete"
    >
      <div>
        <p>确定要删除笔记：<span class="text-orange-600 text-base">{{ localTitle || '无标题笔记' }}</span>？</p>
        <p class="text-gray-500 text-sm mt-2">删除后不可恢复，请谨慎操作。</p>
      </div>
    </CustomModal>

  </div>
</template>

<style scoped>
.editor-wrapper {
  height: calc(100vh - 72px);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.title-area {
  flex: 1;
  min-width: 180px;
}

.actions {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-shrink: 0;
}

.word-count {
  color: #666;
  font-size: 13px;
}

.editor-host {
  flex: 1;
  min-height: 0;
}

.meta-info {
  display: flex;
  gap: 16px;
  font-size: 12px;
  color: #888;
}
.actions {
  display: flex;
  gap: 8px;
}
.editor-host {
  min-height: 500px;
}
.editor-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 4px;
  padding: 4px 10px;
  border-top: 1px solid #eee;
}

.title-area {
  width: 200px;
}
.meta-info {
  display: flex;
  gap: 14px;
  font-size: 12px;
  color: #888;
}

.actions {
  margin-left: auto;
  display: flex;
  gap: 8px;
}

.editor-host {
  min-height: 500px;
}
</style>