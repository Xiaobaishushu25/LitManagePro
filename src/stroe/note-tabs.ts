import { defineStore } from "pinia"
import {NoteResponseDto, NoteTab} from "../components/main/main-type.ts";

function cloneNote(note: NoteResponseDto): NoteResponseDto {
    return JSON.parse(JSON.stringify(note))
}

function createTabId(noteId: number) {
    return `note-${noteId}`
}

function isNoteDirty(original: NoteResponseDto, draft: NoteResponseDto) {
    return (
        (original.title ?? "") !== (draft.title ?? "") ||
        original.content !== draft.content
    )
}

interface NoteTabsState {
    tabs: NoteTab[]
    activeTabId: string | null
}

export const useNoteTabsStore = defineStore("note-tabs", {
    state: (): NoteTabsState => ({
        tabs: [],
        activeTabId: null
    }),

    getters: {
        activeTab(state): NoteTab | null {
            if (!state.activeTabId) return null
            return state.tabs.find(tab => tab.tabId === state.activeTabId) ?? null
        },

        openedNoteIds(state): number[] {
            return state.tabs.map(tab => tab.noteId)
        }
    },

    actions: {
        openNote(note: NoteResponseDto) {
            const existing = this.tabs.find(tab => tab.noteId === note.id)

            if (existing) {
                this.activeTabId = existing.tabId
                return existing
            }

            const newTab: NoteTab = {
                tabId: createTabId(note.id),
                noteId: note.id,
                original: cloneNote(note),
                draft: cloneNote(note),
                dirty: false,
                saving: false,
                loading: false,
                closable: true
            }

            this.tabs.push(newTab)
            this.activeTabId = newTab.tabId
            return newTab
        },

        openNotes(notes: NoteResponseDto[]) {
            for (const note of notes) {
                this.openNote(note)
            }
        },

        setActiveTab(tabId: string) {
            const exists = this.tabs.some(tab => tab.tabId === tabId)
            if (exists) {
                this.activeTabId = tabId
            }
        },

        updateDraftByTabId(
            tabId: string,
            patch: Partial<Pick<NoteResponseDto, "title" | "content">>
        ) {
            const tab = this.tabs.find(item => item.tabId === tabId)
            if (!tab) return

            tab.draft = {
                ...tab.draft,
                ...patch
            }

            tab.dirty = isNoteDirty(tab.original, tab.draft)
        },

        updateActiveDraft(patch: Partial<Pick<NoteResponseDto, "title" | "content">>) {
            if (!this.activeTabId) return
            this.updateDraftByTabId(this.activeTabId, patch)
        },

        replaceNote(note: NoteResponseDto) {
            const tab = this.tabs.find(item => item.noteId === note.id)
            if (!tab) return

            tab.original = cloneNote(note)
            tab.draft = cloneNote(note)
            tab.dirty = false
            tab.saving = false
            tab.loading = false
        },

        markSaving(tabId: string, saving: boolean) {
            const tab = this.tabs.find(item => item.tabId === tabId)
            if (!tab) return
            tab.saving = saving
        },

        closeTab(tabId: string) {
            const index = this.tabs.findIndex(tab => tab.tabId === tabId)
            if (index === -1) return

            const closing = this.tabs[index]
            this.tabs.splice(index, 1)

            if (this.activeTabId !== tabId) return

            if (this.tabs.length === 0) {
                this.activeTabId = null
                return
            }

            const nextTab = this.tabs[index] ?? this.tabs[index - 1]
            this.activeTabId = nextTab?.tabId ?? null
            return closing
        },

        closeActiveTab() {
            if (!this.activeTabId) return
            return this.closeTab(this.activeTabId)
        },

        resetAll() {
            this.tabs = []
            this.activeTabId = null
        }
    }
})