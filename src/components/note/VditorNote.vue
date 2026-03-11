<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue"
import Vditor from "vditor"
import "vditor/dist/index.css"
import "katex/dist/katex.min.css"
import { NButton, NInput } from "naive-ui"
import {NoteResponseDto} from "../main/main-type.ts";

const props = defineProps<{
  note: NoteResponseDto | null
}>()

const emit = defineEmits<{
  (e: "update:title", value: string): void
  (e: "update:content", value: string): void
  (e: "save"): void
}>()

const editorRef = ref<HTMLElement | null>(null)
const vditor = ref<Vditor | null>(null)
const content = ref("")
const localTitle = ref("")
const currentNoteId = ref<number | null>(null)

let isSettingValue = false
let mathInsertLock = false

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

onMounted(async () => {
  await nextTick()
  vditor.value = new Vditor(editorRef.value!, {
    height: "100%",
    minHeight: 500,
    mode: "ir",
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
      mermaid: {
        enable: true
      }
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
        tip: "插入 Mermaid",
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
      { name: "edit-mode", tipPosition: "s" },
      { name: "preview", tipPosition: "s" },
      { name: "export", tipPosition: "s" }
    ],

    upload: {
      url: "/api/upload",
      accept: "image/*",
      max: 5 * 1024 * 1024
    },

    input(value) {
      if (isSettingValue) return

      content.value = value
      emit("update:content", value)

      if (currentNoteId.value) {
        localStorage.setItem(buildCacheKey(currentNoteId.value), value)
      }
    },

    after() {
      syncEditorFromProps(props.note)
    }
  })
})

watch(
    () => props.note,
    (newNote) => {
      syncEditorFromProps(newNote)
    },
    {
      immediate: true,
      deep: true
    }
)

watch(
    () => props.note?.id,
    (newId, oldId) => {
      if (newId === oldId) return

      if (newId && props.note) {
        localTitle.value = props.note.title ?? ""
        content.value = props.note.content ?? ""
      }
    }
)

watch(localTitle, (value) => {
  emit("update:title", value)
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

      <!-- 标题 -->
      <div class="title-area">
        <NInput
            v-model:value="localTitle"
            placeholder="请输入标题"
            clearable
            size="small"
        />
      </div>

      <!-- 信息 -->
      <div class="meta-info">
        <span>创建：{{ formatTime(note?.created_at) }}</span>
        <span>修改：{{ formatTime(note?.updated_at) }}</span>
        <span class="word-count">字数：{{ content.length }}</span>
      </div>

      <!-- 按钮 -->
      <div class="actions">
        <NButton type="primary" size="small" @click="handleSave">
          保存
        </NButton>
      </div>

    </div>

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
  width: 260px;
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