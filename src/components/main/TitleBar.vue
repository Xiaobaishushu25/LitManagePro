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
import {listen} from "@tauri-apps/api/event";

const configStore = useConfigStore()
const tagStore = useTagGroupsStore()

const isMaximize = ref();
const max_state_name = ref('maximize')

let unlisten1: () => void;
let unlisten2: () => void;
let unlisten3: () => void;

onMounted(async ()=>{
  await WebviewWindow.getCurrent().isMaximized().then(res => {
    isMaximize.value = res
  })
  unlisten1 = await listen('导入文件', async (_event) => {
    await openFileSelect()
  })
  unlisten2 = await listen('打开设置', async (_event) => {
    await open_setting()
  })
  unlisten3 = await listen('拖拽上传', async (_event) => {
    await openDragImport()
  })
  //这个作用是监听窗口大小变化来改变窗口最大化状态，主要是用于鼠标点击标题栏拖拽时会改变最大化状态的监听。
  window.addEventListener('resize', () => {
    const isCurrentlyMaximized = isWindowMaximized();
    if (isCurrentlyMaximized) {
      isMaximize.value = true
    } else if (!isCurrentlyMaximized) {
      isMaximize.value = false
    }
  });
})
onUnmounted(async ()=>{
  unlisten1();
  unlisten2();
  unlisten3();
})
// 判断窗口是否处于最大化状态
function isWindowMaximized() {
  // 获取窗口的内宽度和屏幕的宽度
  const innerWidth = window.innerWidth;
  const screenWidth = window.screen.width;
  // 获取窗口的外宽度（包括边框）
  const outerWidth = window.outerWidth;
  // 如果窗口的外宽度等于屏幕宽度，并且内宽度等于外宽度减去浏览器边框宽度 则认为窗口处于最大化状态
  return outerWidth === screenWidth && innerWidth === outerWidth;
}
//---------------------------------------------窗口操作相关开始--------------------------------------------------------------
watch(isMaximize, async (newValue) => {
  if (newValue==undefined)return
  if(newValue){ //当前状态是最大化
    max_state_name.value = 'restore'
    await WebviewWindow.getCurrent().maximize()
  }else{
    max_state_name.value = 'maximize'
    await WebviewWindow.getCurrent().unmaximize()
  }
}, {immediate: true})

async function window_minimize(){
  await WebviewWindow.getCurrent().minimize()
}
function window_maximize(){
  isMaximize.value =!isMaximize.value
}
async function window_close(){
  await saveWindowState(StateFlags.ALL);
  configStore.saveLastUseTags()
  await invoke('save_config',{config: configStore.config}).then(_ => {}).catch(e => {
    message.error(`保存配置出错${e}`);
  })
  await WebviewWindow.getCurrent().close();
}
//---------------------------------------------窗口操作相关结束--------------------------------------------------------------

//---------------------------------------------下拉菜单相关开始--------------------------------------------------------------
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
]);
// const options = [
//   {
//     label: '设置',
//     key: 'setting',
//     shortKey:configStore.shortcuts?.find(item => item.key === '打开设置')!.value,
//     iconPath: '../assets/svg/setting.svg',//注意这里面不能像n-select一样用icon，会出渲染问题
//     props: {
//       onClick: () => {
//         open_setting()
//       }
//     }
//   },
//   {
//     type: 'divider',
//     key: 'd1'
//   },
//   {
//     label: '导入',
//     key: 'import',
//     shortKey:configStore.shortcuts?.find(item => item.key === '导入文件')!.value,
//     iconPath: '../assets/svg/Import32.svg',//注意这里面不能像n-select一样用icon，会出渲染问题
//     props: {
//       onClick: () => {
//         openFileSelect()
//       }
//     }
//   },
//   {
//     label: '拖拽上传',
//     key: 'dragImport',
//     iconPath: '../assets/svg/DragFile32.svg',
//     props: {
//       onClick: () => {
//         openDragImport()
//       }
//     }
//   },
// ];
const renderLabel = (option:{ label: string, value: string,shortKey:string, iconPath: string}) => {
  return h('div', { class: 'flex items-center w-52' }, [
    h(InlineSvg, { src: option.iconPath, class: 'w-4 h-4' }),
    h('span', { style: { marginLeft: '8px' } }, option.label),
    option.shortKey?h('span', { class: 'ml-auto bg-gray-100 text-gray-600 px-2 py-1 rounded-md text-xs font-mono' }, option.shortKey):null,
  ]);
};
//目前没用到，目前是每个选项单独设置点击事件的的，后续看看有用没，需不需要封装
function handleSelect(value: string) {
  console.log(value)
}
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
  await webview.once('tauri://created', async function () {
    await webview.show()
  });
  await webview.once('tauri://error', function (e) {
    // an error happened creating the webview
    console.error(e);
  });
}
async function openFileSelect(){
  try {
    const paths = await open({
      multiple: true,
      directory: false, // 设置为 true 可以选择目录
    });
    if (paths==null)return
    console.log(paths)
    let selectTagId = tagStore.currentSelectTags.map(tag => tag.id)
    await invoke('insert_docs', {paths: paths, tagsId: selectTagId})
  } catch (error) {
    message.error(`打开文件选择器时出错:${error}`)
  }
}
async function openDragImport() {
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
  await webview.once('tauri://created', async function () {
    await webview.show()
  });
  await webview.once('tauri://error', function (e) {
    // an error happened creating the webview
    console.error(e);
  });
}
async function showAndFocusWindow(label:string){
  const window = await WebviewWindow.getByLabel(label);
  if (window!=null) {
    await window.unminimize()
    await window.setFocus()
    return true
  }
  return false
}
//---------------------------------------------下拉菜单相关结束--------------------------------------------------------------



</script>

<template>
  <div data-tauri-drag-region class="title-bar">
    <div class="w-5"></div> <!-- 占位符，不知道为什么给app.ico设置左边距不好使，直接用这个占空了 -->
    <n-tooltip>
      <template #trigger>
        <img src="../../assets/icon/app.ico" class="w-4 h-4" alt="ICO Icon">
      </template>
      v0.1.3
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
.title-bar{
  align-items: center; /* 垂直居中 */
  display: flex;
  flex-direction:row;
  height: 30px;
  user-select: none;
  background-color: #e8e8e5;
}
.window-button {
  width: 40px;
  height: 30px;
  outline: none;
  cursor: pointer;
}

.min path,
.maximize path,
.restore path,
.close path {
  transform-origin: center;
}

.min path {
  transform: scale(0.5);
}

.maximize path {
  transform: scale(0.6);
  stroke: #0f0f0f;
  stroke-width: 0.1;
}

.restore path {
  transform: scale(0.5);
  stroke-width: 0.2;
}

.close path {
  transform: scale(0.6);
  stroke-width: 20;
  stroke: #0f0f0f;
}

.min:hover,
.maximize:hover,
.restore:hover {
  background-color: #33303020;
}

.close:hover {
  background-color: red;
}

.close:hover path {
  fill: white;
  stroke: white;
}
</style>