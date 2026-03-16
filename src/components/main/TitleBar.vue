<script setup lang="ts">
import {h, onMounted, ref, watch, computed, onUnmounted} from "vue";
import {WebviewWindow} from "@tauri-apps/api/webviewWindow";
import {saveWindowState, StateFlags} from "@tauri-apps/plugin-window-state";
import {invoke} from "@tauri-apps/api/core";
import {message} from "../../message.ts";
import useConfigStore from "../../stroe/config.ts";
import InlineSvg from "vue-inline-svg";
import {open} from "@tauri-apps/plugin-dialog";
import useTagGroupsStore from "../../stroe/tag.ts";
import {emitTo, listen} from "@tauri-apps/api/event";

const configStore = useConfigStore()
const tagStore = useTagGroupsStore()

/* ---------------- window state ---------------- */

const appWindow = WebviewWindow.getCurrent()

const isMaximize = ref<boolean | undefined>()
const max_state_name = ref('maximize')

let unlisten1: () => void;
let unlisten2: () => void;
let unlisten3: () => void;
let unlisten4: () => void;
let unlistenResize: () => void;

/* ---------------- mounted ---------------- */

onMounted(async ()=>{
  await openDragImport(false)
  isMaximize.value = await appWindow.isMaximized()
  /* 监听窗口大小变化（替代 resize hack） */
  unlistenResize = await appWindow.onResized(async ()=>{
    const state = await appWindow.isMaximized()
    if(state !== isMaximize.value){
      isMaximize.value = state
    }
  })
  /* 全局快捷键事件 */
  unlisten1 = await listen('导入文件', async () => {
    await openFileSelect()
  })
  unlisten2 = await listen('打开设置', async () => {
    await open_setting()
  })
  unlisten3 = await listen('拖拽上传', async () => {
    await openDragImport()
  })
  unlisten4 = await listen('打开笔记列表', async () => {
    await open_note_list()
  })

})

/* ---------------- unmount ---------------- */

onUnmounted(()=>{
  unlisten1?.()
  unlisten2?.()
  unlisten3?.()
  unlisten4?.()
  unlistenResize?.()

})

/* ---------------- window state sync ---------------- */

watch(isMaximize, async (newValue) => {
  if (newValue === undefined) return
  if(newValue){
    max_state_name.value = 'restore'
    await appWindow.maximize()
  }else{
    max_state_name.value = 'maximize'
    await appWindow.unmaximize()
  }

},{immediate:true})

/* ---------------- window control ---------------- */

async function window_minimize(){
  await appWindow.minimize()
}

function window_maximize(){
  isMaximize.value = !isMaximize.value
}

async function window_close(){
  await saveWindowState(StateFlags.ALL)
  configStore.saveLastUseTags()
  await invoke('save_config',{config: configStore.config}).catch(e => {
    message.error(`保存配置出错${e}`);
  })
  let label = appWindow.label
  if (label=='main') {
    await emitTo("dragImport", 'close',{})
    const windows = await WebviewWindow.getAll()
    for (const w of windows) {
      if (w.label !== 'main') {
        await w.close()
      }
    }
    await appWindow.close()
    await invoke('exit_app', {})
  }
}

/* ---------------- dropdown menu ---------------- */

const options = computed(() => [
  {
    label: '设置',
    key: 'setting',
    shortKey: configStore.getShortcutByName("打开设置"),
    iconPath: '../assets/svg/setting.svg',
    props: {
      onClick: () => {
        open_setting();
      }
    }
  },
  {
    type: 'divider',
    key: 'd1'
  },
  {
    label: '导入',
    key: 'import',
    shortKey: configStore.getShortcutByName("导入文件"),
    iconPath: '../assets/svg/Import32.svg',
    props: {
      onClick: () => {
        openFileSelect();
      }
    }
  },
  {
    label: '拖拽上传',
    key: 'dragImport',
    shortKey: configStore.getShortcutByName("拖拽上传"),
    iconPath: '../assets/svg/DragFile32.svg',
    props: {
      onClick: () => {
        openDragImport();
      }
    }
  },
  {
    label: '打开笔记列表',
    key: 'openNoteList',
    shortKey: configStore.getShortcutByName("打开笔记列表"),
    iconPath: '../assets/svg/note.svg',
    props: {
      onClick: () => {
        open_note_list();
      }
    }
  },
]);

const renderLabel = (option:{ label: string, value: string,shortKey:string, iconPath: string}) => {
  return h('div', { class: 'flex items-center w-52' }, [
    h(InlineSvg, { src: option.iconPath, class: 'w-4 h-4' }),
    h('span', { style: { marginLeft: '8px' } }, option.label),
    option.shortKey?h('span', { class: 'ml-auto bg-gray-100 text-gray-600 px-2 py-1 rounded-md text-xs font-mono' }, option.shortKey):null,
  ]);
};

function handleSelect(value: string) {
  console.log(value)
}

/* ---------------- windows ---------------- */

async function open_setting(){
  let flag = await showAndFocusWindow('setting')
  if (flag) return
  const webview = new WebviewWindow('setting', {
    url: '/#/setting',
    center: true,
    title: '设置',
    width: 800,
    height: 600,
    minWidth: 700,
    minHeight: 500,
    decorations: false,
    resizable: true,
    dragDropEnabled: false,
    visible: false,
  });
  await webview.once('tauri://created', async () => {
    await webview.show()
  });
}

async function openFileSelect(){
  try {
    const paths = await open({
      multiple: true,
      directory: false,
    });
    if (paths==null)return
    let selectTagId = tagStore.currentSelectTags.map(tag => tag.id)
    await invoke('insert_docs', {paths: paths, tagsId: selectTagId})
  } catch (error) {
    message.error(`打开文件选择器时出错:${error}`)
  }
}

async function openDragImport(visible:boolean=true){
  let flag = await showAndFocusWindow('dragImport')
  if (flag) return
  const webview = new WebviewWindow('dragImport', {
    url: '/#/dragImport',
    center: true,
    title: '拖拽上传',
    width: 600,
    height: 400,
    minWidth: 200,
    minHeight: 200,
    decorations: false,
    resizable: true,
    dragDropEnabled: true,
    visible: false,
    alwaysOnTop: true
  });
  await webview.once('tauri://created', async () => {
    if (visible) {
      await webview.show()
    }
  });

}

async function showAndFocusWindow(label:string){
  const window = await WebviewWindow.getByLabel(label)
  if (window!=null) {
    await window.show()
    await window.unminimize()
    await window.setFocus()
    return true
  }
  return false
}

async function open_note_list(){
  let flag = await showAndFocusWindow('note-list')
  if (flag) return
  const webview = new WebviewWindow('note-list', {
    url: '/#/note-list',
    center: true,
    title: '笔记列表',
    width: 800,
    height: 600,
    minWidth: 500,
    minHeight: 400,
    decorations: false,
    resizable: true,
    dragDropEnabled: false,
    visible: false,
  });
  await webview.once('tauri://created', async () => {
    await webview.show()
    await webview.setFocus()
  });

}
</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="w-5"></div> <!-- 占位符，不知道为什么给app.ico设置左边距不好使，直接用这个占空了 -->
    <n-tooltip>
      <template #trigger>
        <img src="../../assets/icon/app.ico" class="w-4 h-4" alt="ICO Icon">
      </template>
      v3.1.3
    </n-tooltip>
    <label class="pl-1">天书</label>
    <div class="pl-5">
      <n-dropdown trigger="click" :options="options" @select="handleSelect" :render-label="renderLabel">
        <inline-svg src="../assets/svg/MdMenu.svg" class="svg-button w-5 h-5"></inline-svg>
      </n-dropdown>
    </div>
    <div class="ml-auto flex gap-0">
      <inline-svg
          src="../assets/svg/minimize.svg"
          class="window-button min"
          @click.left="window_minimize"
      ></inline-svg>
      <inline-svg
          :src="`../assets/svg/${max_state_name}.svg`"
          :class="`window-button ${max_state_name}`"
          @click.left="window_maximize"
      ></inline-svg>
      <inline-svg
          src="../assets/svg/close.svg"
          class="window-button close"
          @click.left="window_close"
      ></inline-svg>
    </div>
  </div>
</template>

<style>
/*样式转移到/components/TitleBar.vue中了*/
</style>