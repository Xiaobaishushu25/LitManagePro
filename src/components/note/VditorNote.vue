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
import "../../vditor-fix.css"

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
/*AI 复制出来的 Markdown/LaTeX 格式和 Vditor 支持的公式格式不完全一致。
粘贴处理里，再加一步：把 AI 常用的 \[、\]、\(、\) 自动转换成 Vditor 更容易识别的 $ / $$ 公式格式。
* */
function normalizeAiMarkdownPaste(text: string) {
  let md = text
      .replace(/\r\n?/g, "\n")
      .replace(/[\u2028\u2029]/g, "\n")
      .replace(/\u00a0/g, " ")
  // 块级公式：\[ ... \] => $$ ... $$
  md = md.replace(/\\\[\s*([\s\S]*?)\s*\\\]/g, (_all: string, formula: string) => {
    return `$$\n${formula.trim()}\n$$`
  })
  // 行内公式：\( ... \) => $...$
  md = md.replace(/\\\(([\s\S]*?)\\\)/g, (_all: string, formula: string) => {
    return `$${formula.trim()}$`
  })
  // 兜底：有些内容粘贴后可能已经变成单独一行的 [ 和 ]
  md = md.replace(
      /(^|\n)\s*\[\s*\n([\s\S]*?)\n\s*\]\s*(?=\n|$)/g,
      (all: string, prefix: string, formula: string) => {
        const body = formula.trim()

        const looksLikeFormula =
            /\\[a-zA-Z]+|[_^=]|[A-Za-z]\s*[+\-*/]\s*[A-Za-z0-9]/.test(body)

        return looksLikeFormula ? `${prefix}$$\n${body}\n$$` : all
      }
  )
  return md
}
/*
v3.2.4引入：
部分 AI 工具复制内容时会同时携带 HTML 和纯文本格式，Vditor 在处理 HTML 粘贴时可能导致 Markdown 换行被合并，进而出现标题、段落全部变成一行的问题。
现已优化粘贴逻辑，对 Markdown 内容优先使用纯文本格式，保证换行和结构正常保留。
* */
function bindPlainMarkdownPaste() {
  const root = editorRef.value
  const vd = vditor.value

  if (!root || !vd) return () => {}

  const handler = (e: ClipboardEvent) => {
    const cd = e.clipboardData
    if (!cd) return
    // 不拦截图片、文件粘贴，避免影响 Vditor 原本上传能力
    if (cd.files && cd.files.length > 0) return
    const plain = cd.getData("text/plain")
    const html = cd.getData("text/html")
    // 没有 HTML 时，通常是正常纯文本粘贴，不需要管
    if (!plain || !html) return
    // const md = plain
    //     .replace(/\r\n?/g, "\n")
    //     .replace(/[\u2028\u2029]/g, "\n")
    //     .replace(/\u00a0/g, " ")
    const md = normalizeAiMarkdownPaste(plain)
    // 只拦截比较像 Markdown 的内容
    const looksLikeMarkdown =
        /(^|\n)\s{0,3}(#{1,6}\s+|[-*+]\s+|\d+\.\s+|>\s+|```|~~~|\|.+\||\$\$)/.test(md) ||
        md.includes("\n\n")
    if (!looksLikeMarkdown) return
    e.preventDefault()
    e.stopPropagation()
    e.stopImmediatePropagation()

    vd.insertValue(md, true)

    scheduleSaveCurrentViewState(currentNoteId.value)
  }
  // capture=true 很关键：抢在 Vditor 自己处理 paste 前执行
  root.addEventListener("paste", handler, true)
  return () => {
    root.removeEventListener("paste", handler, true)
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
type EditorMode = "ir" | "sv" | "wysiwyg"

type SelectionSnapshot = {
  startPath: number[]
  startOffset: number
  endPath: number[]
  endOffset: number
}

type ModeViewState = {
  scrollTop: number
  scrollRatio: number
  hadFocus: boolean
  contentHash: number
  selection: SelectionSnapshot | null
  updatedAt: number
}

type PreviewViewState = {
  scrollTop: number
  scrollRatio: number
  visible: boolean
  previewOnly: boolean
  updatedAt: number
}

type NoteViewState = {
  lastMode: EditorMode
  modes: Partial<Record<EditorMode, ModeViewState>>
  preview: PreviewViewState | null
  updatedAt: number
}

let positionSaveTimer: number | null = null
let layoutRestoreTimer: number | null = null
let unbindPositionEvents: (() => void) | null = null
let unbindLayoutEvents: (() => void) | null = null
let unbindPlainMarkdownPaste: (() => void) | null = null
let restoreToken = 0

function clearPositionSaveTimer() {
  if (positionSaveTimer) {
    window.clearTimeout(positionSaveTimer)
    positionSaveTimer = null
  }
}

function clearLayoutRestoreTimer() {
  if (layoutRestoreTimer) {
    window.clearTimeout(layoutRestoreTimer)
    layoutRestoreTimer = null
  }
}

function buildViewStateKey(noteId: number | null) {
  return noteId ? `vditor-note-view-${noteId}` : "vditor-note-view-empty"
}

function getInternalVditor(): any {
  return (vditor.value as any)?.vditor ?? null
}

function getCurrentMode(): EditorMode {
  return (vditor.value?.getCurrentMode?.() ??
      (configStore.config?.note_config?.editor_mode || "ir")) as EditorMode
}

function getModeEditorElement(mode: EditorMode = getCurrentMode()): HTMLElement | null {
  return getInternalVditor()?.[mode]?.element ?? null
}

function getModeEditorShell(mode: EditorMode = getCurrentMode()): HTMLElement | null {
  const editor = getModeEditorElement(mode)
  if (!editor) return null
  return mode === "sv" ? editor : ((editor.parentElement as HTMLElement | null) ?? editor)
}

function getPreviewContainer(): HTMLElement | null {
  return getInternalVditor()?.preview?.element ?? null
}

function isVisibleElement(el: HTMLElement | null | undefined) {
  return !!el && getComputedStyle(el).display !== "none"
}

function isPreviewVisible() {
  return isVisibleElement(getPreviewContainer())
}

function isPreviewOnly() {
  const preview = getPreviewContainer()
  const shell = getModeEditorShell()
  return !!preview && isVisibleElement(preview) && (!shell || !isVisibleElement(shell))
}

function hasFocusWithin(editor: HTMLElement) {
  const active = document.activeElement
  return !!active && (active === editor || editor.contains(active))
}

function hashString(text: string) {
  let h = 0
  for (let i = 0; i < text.length; i++) {
    h = (h * 31 + text.charCodeAt(i)) | 0
  }
  return h
}

function getMarkdownHash() {
  return hashString(vditor.value?.getValue?.() ?? content.value ?? "")
}

function getScrollRatio(el: HTMLElement) {
  const max = el.scrollHeight - el.clientHeight
  return max > 0 ? el.scrollTop / max : 0
}

function setScrollByRatio(el: HTMLElement, ratio: number) {
  const max = el.scrollHeight - el.clientHeight
  el.scrollTop = max > 0 ? max * Math.min(1, Math.max(0, ratio)) : 0
}

function readNoteViewState(noteId: number | null): NoteViewState | null {
  if (!noteId) return null
  const raw = localStorage.getItem(buildViewStateKey(noteId))
  if (!raw) return null

  try {
    return JSON.parse(raw) as NoteViewState
  } catch {
    return null
  }
}

function writeNoteViewState(noteId: number | null, state: NoteViewState) {
  if (!noteId) return
  localStorage.setItem(buildViewStateKey(noteId), JSON.stringify(state))
}

function getNodePath(root: Node, node: Node): number[] | null {
  const path: number[] = []
  let current: Node | null = node

  while (current && current !== root) {
    const parent: Node | null = current.parentNode
    if (!parent) return null

    const index = Array.from(parent.childNodes).indexOf(<ChildNode>current)
    if (index < 0) return null

    path.unshift(index)
    current = parent
  }

  return current === root ? path : null
}

function resolveNodePath(root: Node, path: number[]) {
  let current: Node = root

  for (const index of path) {
    const next = current.childNodes[index]
    if (!next) return null
    current = next
  }

  return current
}

function clampOffset(node: Node, offset: number) {
  if (node.nodeType === Node.TEXT_NODE) {
    return Math.min(offset, node.textContent?.length ?? 0)
  }
  return Math.min(offset, node.childNodes.length)
}

function captureSelection(root: HTMLElement): SelectionSnapshot | null {
  const selection = window.getSelection()
  if (!selection || selection.rangeCount === 0) return null

  const range = selection.getRangeAt(0)
  const common = range.commonAncestorContainer

  if (!(common === root || root.contains(common))) {
    return null
  }

  const startPath = getNodePath(root, range.startContainer)
  const endPath = getNodePath(root, range.endContainer)

  if (!startPath || !endPath) return null

  return {
    startPath,
    startOffset: range.startOffset,
    endPath,
    endOffset: range.endOffset,
  }
}

function restoreSelection(root: HTMLElement, snapshot?: SelectionSnapshot | null) {
  if (!snapshot) return

  const startNode = resolveNodePath(root, snapshot.startPath)
  const endNode = resolveNodePath(root, snapshot.endPath)
  if (!startNode || !endNode) return

  const range = document.createRange()

  try {
    range.setStart(startNode, clampOffset(startNode, snapshot.startOffset))
    range.setEnd(endNode, clampOffset(endNode, snapshot.endOffset))
  } catch {
    return
  }

  const selection = window.getSelection()
  selection?.removeAllRanges()
  selection?.addRange(range)
}

function saveCurrentViewState(noteId: number | null = currentNoteId.value) {
  if (!noteId || !vditor.value) return

  const mode = getCurrentMode()
  const editor = getModeEditorElement(mode)
  const preview = getPreviewContainer()
  const now = Date.now()

  const state: NoteViewState =
      readNoteViewState(noteId) ?? {
        lastMode: mode,
        modes: {},
        preview: null,
        updatedAt: now,
      }

  state.lastMode = mode
  state.updatedAt = now

  if (editor) {
    state.modes[mode] = {
      scrollTop: editor.scrollTop,
      scrollRatio: getScrollRatio(editor),
      hadFocus: hasFocusWithin(editor),
      contentHash: getMarkdownHash(),
      selection: captureSelection(editor),
      updatedAt: now,
    }
  }

  state.preview = preview
      ? {
        scrollTop: preview.scrollTop,
        scrollRatio: getScrollRatio(preview),
        visible: isPreviewVisible(),
        previewOnly: isPreviewOnly(),
        updatedAt: now,
      }
      : null

  writeNoteViewState(noteId, state)
}

function scheduleSaveCurrentViewState(noteId: number | null = currentNoteId.value) {
  if (!noteId) return

  clearPositionSaveTimer()
  positionSaveTimer = window.setTimeout(() => {
    saveCurrentViewState(noteId)
  }, 120)
}

async function waitEditorStable() {
  await nextTick()
  await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()))
  await new Promise<void>((resolve) => requestAnimationFrame(() => resolve()))
}

async function restoreCurrentViewState(noteId: number | null = currentNoteId.value) {
  if (!noteId || !vditor.value) return

  const state = readNoteViewState(noteId)
  if (!state) return

  const mode = getCurrentMode()
  const editor = getModeEditorElement(mode)
  if (!editor) return

  const sameModeState = state.modes[mode]
  const fallbackState = state.modes[state.lastMode]

  // 当前 mode 有自己的状态：精确恢复
  if (sameModeState) {
    editor.scrollTop = sameModeState.scrollTop ?? 0

    if (sameModeState.hadFocus) {
      editor.focus()

      // 只有内容哈希一致时才恢复选区，避免内容变更后错位
      if (sameModeState.contentHash === getMarkdownHash()) {
        restoreSelection(editor, sameModeState.selection)
      }
    }

    requestAnimationFrame(() => {
      editor.scrollTop = sameModeState.scrollTop ?? 0
    })
  }
  // 当前 mode 以前没进过：按上次 mode 的滚动比例回一个“接近位置”
  else if (fallbackState) {
    setScrollByRatio(editor, fallbackState.scrollRatio ?? 0)
  }

  // 预览区滚动单独恢复
  const preview = getPreviewContainer()
  if (preview && isPreviewVisible() && state.preview) {
    requestAnimationFrame(() => {
      if (typeof state.preview?.scrollTop === "number") {
        preview.scrollTop = state.preview.scrollTop
      } else {
        setScrollByRatio(preview, state.preview?.scrollRatio ?? 0)
      }
    })
  }
}

function bindViewStateEvents() {
  unbindPositionEvents?.()

  const editor = getModeEditorElement()
  const preview = getPreviewContainer()
  const handler = () => scheduleSaveCurrentViewState()

  const cleanups: Array<() => void> = []

  if (editor) {
    editor.addEventListener("scroll", handler, { passive: true })
    editor.addEventListener("input", handler)
    editor.addEventListener("keyup", handler)
    editor.addEventListener("mouseup", handler)

    cleanups.push(() => editor.removeEventListener("scroll", handler))
    cleanups.push(() => editor.removeEventListener("input", handler))
    cleanups.push(() => editor.removeEventListener("keyup", handler))
    cleanups.push(() => editor.removeEventListener("mouseup", handler))
  }

  if (preview) {
    preview.addEventListener("scroll", handler, { passive: true })
    cleanups.push(() => preview.removeEventListener("scroll", handler))
  }

  unbindPositionEvents = () => {
    cleanups.forEach((fn) => fn())
  }
}

function isLayoutTriggerTarget(target: HTMLElement | null) {
  if (!target) return false
  return Boolean(
      target.closest('[data-type="edit-mode"]') ||
      target.closest('[data-type="preview"]') ||
      target.closest('[data-type="both"]') ||
      target.closest("[data-mode]")
  )
}

function isLayoutCommitTarget(target: HTMLElement | null) {
  if (!target) return false
  return Boolean(
      target.closest('[data-type="preview"]') ||
      target.closest('[data-type="both"]') ||
      target.closest("[data-mode]")
  )
}

function scheduleLayoutRestoreAfterSwitch(waitPreview = false) {
  clearLayoutRestoreTimer()
  isSwitchingMode = true

  const previewDelay = Number(getInternalVditor()?.options?.preview?.delay ?? 200)
  const delay = waitPreview ? previewDelay + 40 : 60

  layoutRestoreTimer = window.setTimeout(async () => {
    try {
      await waitEditorStable()
      bindViewStateEvents()
      await restoreCurrentViewState(currentNoteId.value)

      const currentMode = getCurrentMode()
      if (configStore.config?.note_config) {
        configStore.config.note_config.editor_mode = currentMode
      }
    } finally {
      isSwitchingMode = false
    }
  }, delay)
}

function bindLayoutSwitchEvents() {
  unbindLayoutEvents?.()

  // 用 mousedown/touchstart 在失焦前先保存，避免点击工具栏后选区丢了
  const preHandler = (e: Event) => {
    const target = e.target as HTMLElement | null
    if (!isLayoutTriggerTarget(target)) return
    saveCurrentViewState(currentNoteId.value)
  }

  // 用 click 在 Vditor 完成切换后做恢复
  const postHandler = (e: Event) => {
    const target = e.target as HTMLElement | null
    if (!isLayoutCommitTarget(target)) return

    const waitPreview = Boolean(
        target?.closest('[data-type="preview"]') || target?.closest('[data-type="both"]')
    )

    scheduleLayoutRestoreAfterSwitch(waitPreview)
  }

  document.addEventListener("mousedown", preHandler, true)
  document.addEventListener("touchstart", preHandler, true)
  document.addEventListener("click", postHandler, true)

  unbindLayoutEvents = () => {
    document.removeEventListener("mousedown", preHandler, true)
    document.removeEventListener("touchstart", preHandler, true)
    document.removeEventListener("click", postHandler, true)
  }
}

function handleVisibilityChange() {
  if (document.visibilityState === "hidden") {
    saveCurrentViewState()
  }
}
function handlePageHide() {
  saveCurrentViewState()
}
// 这段实现的要点是：
// 每个 note 每个 mode 各存一份
// 同mode：恢复精确滚动 + 精确选区
// 跨mode：恢复滚动比例，不强行复用别的mode的光标
// 预览区：单独存preview.element.scrollTop
// 这和 Vditor 现有结构是匹配的：它有三种编辑模式，getCurrentMode()/focus()/blur() 都是围绕 currentMode 工作；预览区也有单独的 preview.element/preview.previewElement 结构。工具栏 preview 按钮在 sv 和其他 mode 下隐藏编辑区的方式也不同，所以预览滚动最好单独记。

//监听大纲是否打开
let unbindOutlineObserver: (() => void) | null = null
function observeOutlineState() {
  const root = editorRef.value
  if (!root) {
    console.log("observeOutlineState: root 不存在")
    return () => {}
  }
  const observer = new MutationObserver((mutations) => {
    for (const m of mutations) {
      const target = m.target as HTMLElement
      if (
          m.type === "attributes" &&
          m.attributeName === "style" &&
          target.classList.contains("vditor-outline")
      ) {
        const visible = getComputedStyle(target).display !== "none"
        console.log("大纲面板:", visible ? "打开" : "关闭")
        
        // 保存大纲状态到配置
        if (configStore.config?.note_config) {
          configStore.config.note_config.outline_enabled = visible
        }
      }
    }
  })
  observer.observe(root, {
    subtree: true,
    attributes: true,
    attributeFilter: ["style"],
  })
  return () => {
    observer.disconnect()
  }
}
//setOutlineVisible可以控制大纲的打开或者关闭，但是我直接在配置里面可以初始化，就没用上
//@ts-ignore
function setOutlineVisible(visible: boolean) {
  const vd = (vditor.value as any)?.vditor ?? vditor.value
  if (!vd?.outline) return
  vd.options.outline.enable = visible
  vd.outline.toggle(vd, visible)

  const btn = vd.toolbar?.elements?.outline?.firstElementChild as HTMLElement | null
  if (btn) {
    btn.classList.toggle("vditor-menu--current", visible)
  }
}

onMounted(async () => {
  await nextTick()
  vditor.value = new Vditor(editorRef.value!, {
    cdn: '/vditor',// 使用本地资源（把 node_modules 中 vditor 的 dist 复制到本地保存），不然 dev 模式下没问题，但是打包后编辑器找不到资源
    height: "100%",
    minHeight: 500,
    mode: configStore.config?.note_config?.editor_mode || "ir",
    theme: "classic",
    icon: "material",
    placeholder: "开始写 Markdown / LaTeX / Mermaid ...",
    cache: {
      enable: false
    },
    outline: {
      enable: configStore.config?.note_config?.outline_enabled ?? false,
      position:"left"//目前大纲默认在左边并且不可更改位置，其实可以配置到右边
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
      scheduleSaveCurrentViewState(currentNoteId.value)
    },
    focus() {
      scheduleSaveCurrentViewState(currentNoteId.value)
    },
    blur() {
      scheduleSaveCurrentViewState(currentNoteId.value)
    },
    select() {
      scheduleSaveCurrentViewState(currentNoteId.value)
    },
    after: async () => {
      syncEditorFromProps(props.note)
      bindViewStateEvents()
      bindLayoutSwitchEvents()

      unbindPlainMarkdownPaste?.()
      unbindPlainMarkdownPaste = bindPlainMarkdownPaste()

      unbindOutlineObserver?.()
      unbindOutlineObserver = observeOutlineState()

      await waitEditorStable()
      await restoreCurrentViewState(props.note?.id ?? null)

      const currentMode = getCurrentMode()
      if (configStore.config?.note_config) {
        configStore.config.note_config.editor_mode = currentMode
      }
    },
  })
  console.log("Vditor 初始化完成")
  document.addEventListener("visibilitychange", handleVisibilityChange)
  observeOutlineState()
  window.addEventListener("pagehide", handlePageHide)
})

watch(
    () => props.note?.id,
    async (newId, oldId) => {
      clearPositionSaveTimer()
      clearLayoutRestoreTimer()

      // 切走旧 note 前先保存旧 note 当前 mode 的状态
      if (oldId) {
        saveCurrentViewState(oldId)
      }

      syncEditorFromProps(props.note)

      if (newId) {
        localStorage.setItem("note-last-open-id", String(newId))
      }
      if (!newId || !vditor.value) return
      const token = ++restoreToken
      await waitEditorStable()
      if (token !== restoreToken) return
      bindViewStateEvents()
      await restoreCurrentViewState(newId)
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
    async (value) => {
      if (!vditor.value) return

      const nextValue = value ?? ""
      const currentValue = vditor.value.getValue()

      if (currentValue !== nextValue) {
        const noteId = currentNoteId.value
        content.value = nextValue
        isSettingValue = true

        vditor.value.setValue(nextValue)

        await waitEditorStable()

        if (noteId && noteId === currentNoteId.value) {
          await restoreCurrentViewState(noteId)
        }

        requestAnimationFrame(() => {
          isSettingValue = false
        })
      }
    }
)

watch(localTitle, (value) => {
  emit("update:title", value)
})


onDeactivated(() => {
  clearPositionSaveTimer()
  clearLayoutRestoreTimer()
  saveCurrentViewState()

  unbindPositionEvents?.()
  unbindPositionEvents = null

  unbindPlainMarkdownPaste?.()
  unbindPlainMarkdownPaste = null

  unbindOutlineObserver?.()
  unbindOutlineObserver = null

  unbindLayoutEvents?.()
  unbindLayoutEvents = null
})

onActivated(async () => {
  if (!currentNoteId.value || !vditor.value) return

  bindViewStateEvents()
  bindLayoutSwitchEvents()

  unbindPlainMarkdownPaste?.()
  unbindPlainMarkdownPaste = bindPlainMarkdownPaste()

  await waitEditorStable()
  await restoreCurrentViewState(currentNoteId.value)
})

onUnmounted(() => {
  clearPositionSaveTimer()
  clearLayoutRestoreTimer()
  saveCurrentViewState()

  unbindPositionEvents?.()
  unbindPositionEvents = null

  unbindLayoutEvents?.()
  unbindLayoutEvents = null

  unbindPlainMarkdownPaste?.()
  unbindPlainMarkdownPaste = null

  unbindOutlineObserver?.()
  unbindOutlineObserver = null

  document.removeEventListener("visibilitychange", handleVisibilityChange)
  window.removeEventListener("pagehide", handlePageHide)

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