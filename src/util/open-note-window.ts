import { WebviewWindow } from "@tauri-apps/api/webviewWindow"
import {emitTo, once} from "@tauri-apps/api/event"
import {NoteResponseDto} from "../components/main/main-type.ts";

async function showAndFocusWindow(label: string) {
    const existing = await WebviewWindow.getByLabel(label)
    if (existing) {
        await existing.show()
        await existing.unminimize()
        await existing.setFocus()
        return true
    }
    return false
}
export async function openNoteWindow(note: NoteResponseDto) {
    const exists = await showAndFocusWindow("note-workspace")
    if (exists) {
        await emitTo("note-workspace", "open-note", note)
        return
    }
    const webview = new WebviewWindow("note-workspace", {
        url: "/#/note",
        center: true,
        title: "笔记",
        width: 1100,
        height: 760,
        minWidth: 700,
        minHeight: 500,
        decorations: false,
        resizable: true,
        dragDropEnabled: false,
        visible: false
    })

    await webview.once("tauri://created", async () => {
        await webview.show()
        await webview.setFocus()
        // await emitTo("note-workspace", "open-note", note)
        // 等待子窗口前端明确发 ready
        const unlisten = await once("note-window-ready", async () => {
            await emitTo("note-workspace", "open-note", note)
            unlisten()
        })
    })
    await webview.once("tauri://error", (e) => {
        console.error("窗口创建失败:", e)
    })
}