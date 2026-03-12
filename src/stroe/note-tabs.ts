// import { defineStore } from "pinia"
// import {NoteResponseDto, NoteTab} from "../components/main/main-type.ts";
//
// function cloneNote(note: NoteResponseDto): NoteResponseDto {
//     return JSON.parse(JSON.stringify(note))
// }
//
// function createTabId(noteId: number) {
//     return `note-${noteId}`
// }
//
// function isNoteDirty(original: NoteResponseDto, draft: NoteResponseDto) {
//     return (
//         (original.title ?? "") !== (draft.title ?? "") ||
//         original.content !== draft.content
//     )
// }
//
// interface NoteTabsState {
//     tabs: NoteTab[]
//     activeTabId: string | null
// }
//
// export const useNoteTabsStore = defineStore("note-tabs", {
//     state: (): NoteTabsState => ({
//         tabs: [],
//         activeTabId: null
//     }),
//
//     getters: {
//         activeTab(state): NoteTab | null {
//             if (!state.activeTabId) return null
//             return state.tabs.find(tab => tab.tabId === state.activeTabId) ?? null
//         },
//
//         openedNoteIds(state): number[] {
//             return state.tabs.map(tab => tab.noteId)
//         }
//     },
//
//     actions: {
//         openNote(note: NoteResponseDto) {
//             const existing = this.tabs.find(tab => tab.noteId === note.id)
//
//             if (existing) {
//                 this.activeTabId = existing.tabId
//                 return existing
//             }
//
//             const newTab: NoteTab = {
//                 tabId: createTabId(note.id),
//                 noteId: note.id,
//                 original: cloneNote(note),
//                 draft: cloneNote(note),
//                 dirty: false,
//                 saving: false,
//                 loading: false,
//                 closable: true
//             }
//
//             this.tabs.push(newTab)
//             this.activeTabId = newTab.tabId
//             return newTab
//         },
//
//         openNotes(notes: NoteResponseDto[]) {
//             for (const note of notes) {
//                 this.openNote(note)
//             }
//         },
//
//         setActiveTab(tabId: string) {
//             const exists = this.tabs.some(tab => tab.tabId === tabId)
//             if (exists) {
//                 this.activeTabId = tabId
//             }
//         },
//
//         updateDraftByTabId(
//             tabId: string,
//             patch: Partial<Pick<NoteResponseDto, "title" | "content">>
//         ) {
//             const tab = this.tabs.find(item => item.tabId === tabId)
//             if (!tab) return
//
//             tab.draft = {
//                 ...tab.draft,
//                 ...patch
//             }
//
//             tab.dirty = isNoteDirty(tab.original, tab.draft)
//         },
//
//         updateActiveDraft(patch: Partial<Pick<NoteResponseDto, "title" | "content">>) {
//             if (!this.activeTabId) return
//             this.updateDraftByTabId(this.activeTabId, patch)
//         },
//
//         replaceNote(note: NoteResponseDto) {
//             const tab = this.tabs.find(item => item.noteId === note.id)
//             if (!tab) return
//
//             tab.original = cloneNote(note)
//             tab.draft = cloneNote(note)
//             tab.dirty = false
//             tab.saving = false
//             tab.loading = false
//         },
//
//         markSaving(tabId: string, saving: boolean) {
//             const tab = this.tabs.find(item => item.tabId === tabId)
//             if (!tab) return
//             tab.saving = saving
//         },
//
//         closeTab(tabId: string) {
//             const index = this.tabs.findIndex(tab => tab.tabId === tabId)
//             if (index === -1) return
//
//             const closing = this.tabs[index]
//             this.tabs.splice(index, 1)
//
//             if (this.activeTabId !== tabId) return
//
//             if (this.tabs.length === 0) {
//                 this.activeTabId = null
//                 return
//             }
//
//             const nextTab = this.tabs[index] ?? this.tabs[index - 1]
//             this.activeTabId = nextTab?.tabId ?? null
//             return closing
//         },
//
//         closeActiveTab() {
//             if (!this.activeTabId) return
//             return this.closeTab(this.activeTabId)
//         },
//
//         resetAll() {
//             this.tabs = []
//             this.activeTabId = null
//         }
//     }
// })
import { defineStore } from "pinia"
import { ref, computed } from "vue"
import { NoteResponseDto, NoteTab } from "../components/main/main-type.ts"

const cloneNote = (note: NoteResponseDto): NoteResponseDto =>
    JSON.parse(JSON.stringify(note))
const createTabId = (noteId: number) => `note-${noteId}`
const isNoteDirty = (original: NoteResponseDto, draft: NoteResponseDto) =>
    (original.title ?? "") !== (draft.title ?? "") || original.content !== draft.content
export const useNoteTabsStore = defineStore("note-tabs", () => {
    const tabs = ref<NoteTab[]>([])
    const activeTabId = ref<string | null>(null)
    const activeTab = computed<NoteTab | null>(() => {
        if (!activeTabId.value) return null
        return tabs.value.find(tab => tab.tabId === activeTabId.value) ?? null
    })
    const openedNoteIds = computed<number[]>(() =>
        tabs.value.map(tab => tab.noteId)
    )
    const openNote = (note: NoteResponseDto) => {
        const existing = tabs.value.find(tab => tab.noteId === note.id &&tab.docId===note.document_id)
        if (existing) {
            activeTabId.value = existing.tabId
            return existing
        }
        const tab: NoteTab = {
            tabId: createTabId(note.id),
            noteId: note.id,
            docId:note.document_id,
            original: cloneNote(note),
            draft: cloneNote(note),
            dirty: false,
            saving: false,
            loading: false,
            closable: true
        }
        tabs.value.push(tab)
        activeTabId.value = tab.tabId
        return tab
    }
    const openNotes = (notes: NoteResponseDto[]) => {
        notes.forEach(openNote)
    }
    const setActiveTab = (tabId: string) => {
        const exists = tabs.value.some(tab => tab.tabId === tabId)
        if (exists) {
            activeTabId.value = tabId
        }
    }
    const updateDraftByTabId = (
        tabId: string,
        patch: Partial<Pick<NoteResponseDto, "title" | "content">>
    ) => {
        const tab = tabs.value.find(t => t.tabId === tabId)
        if (!tab) return
        tab.draft = {
            ...tab.draft,
            ...patch
        }
        tab.dirty = isNoteDirty(tab.original, tab.draft)
    }

    const updateActiveDraft = (
        patch: Partial<Pick<NoteResponseDto, "title" | "content">>
    ) => {
        if (!activeTabId.value) return
        updateActiveDraftById(activeTabId.value, patch)

    }
    const updateActiveDraftById = (
        tabId: string,
        patch: Partial<Pick<NoteResponseDto, "title" | "content">>
    ) => {
        updateDraftByTabId(tabId, patch)
    }
    //参数oldNoteId是为了判断是否是新建的笔记，新建的笔记就不能用t.noteId === note.id来查找了
    const replaceNote = (oldNoteId:number, note: NoteResponseDto) => {
        let tab
        if (oldNoteId===-1){//新建的笔记要根据原来的-1id和文档id来查找
            tab = tabs.value.find(t => t.noteId === -1&&t.docId===note.document_id)
        }else {//更新的笔记直接按id查找
            tab = tabs.value.find(t => t.noteId === note.id)
        }
        // const tab = tabs.value.find(t => t.noteId === note.id)
        if (!tab) return

        tab.original = cloneNote(note)
        tab.draft = cloneNote(note)

        tab.dirty = false
        tab.saving = false
        tab.loading = false

    }

    const markSaving = (tabId: string, saving: boolean) => {
        const tab = tabs.value.find(t => t.tabId === tabId)
        if (!tab) return
        tab.saving = saving
    }
    const closeTab = (tabId: string) => {
        const index = tabs.value.findIndex(tab => tab.tabId === tabId)
        if (index === -1) return
        const closing = tabs.value[index]
        tabs.value.splice(index, 1)
        if (activeTabId.value !== tabId) {
            return closing
        }
        if (tabs.value.length === 0) {
            activeTabId.value = null
            return closing
        }
        const nextTab = tabs.value[index] ?? tabs.value[index - 1]

        activeTabId.value = nextTab?.tabId ?? null

        return closing
    }

    const closeActiveTab = () => {

        if (!activeTabId.value) return

        return closeTab(activeTabId.value)

    }
    const resetAll = () => {

        tabs.value = []
        activeTabId.value = null

    }

    return {
        tabs,
        activeTabId,
        activeTab,
        openedNoteIds,
        openNote,
        openNotes,
        setActiveTab,
        updateDraftByTabId,
        updateActiveDraft,
        replaceNote,
        markSaving,
        closeTab,
        closeActiveTab,
        resetAll
    }

})