<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {DocumentTags} from "./main-type.ts";
import { h } from 'vue'
import {DataTableColumns} from "naive-ui";
import {message} from "../../message.ts";
import { listen } from '@tauri-apps/api/event'
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import useDocStore from "../../stroe/doc.ts";
import Detail from "./Detail.vue";
import useConfigStore from "../../stroe/config.ts";
import CustomModal from "../../util/CustomModal.vue";
import TagComplete from "./TagComplete.vue";

const tagStore = useTagGroupsStore()
const docsStore = useDocStore()
const configStore = useConfigStore()

//不知道为什么直接监听tagStore.两个Tags不可以，只有添加的时候可以监听到，删除一次后执行任何操作不会触发监听，
const watchAndTags = computed(() => tagStore.andTags)
const watchOrTags = computed(() => tagStore.orTags)

let unlistenFile: () => void;
let unlistenDoc: () => void;
let unlistenDocUp: () => void;

const selectedRowIndex = ref(-1);

const showDocDeleteModal = ref(false)

const contextMenuShow = ref(false)
const contextOptions = {
  theme: 'flat',
  name:"",
  code:"",
  zIndex: 3,
  x: 500,
  y: 200
}
const handleBlur = () => {
  contextMenuShow.value = false
};

onMounted(async ()=>{
  unlistenFile = await listen('tauri://drag-drop', async (event:{ payload:{paths: string[]}})=>{
    let selectTagId = tagStore.currentSelectTags.map(tag => tag.id)
    await invoke('insert_docs', {paths: event.payload.paths, tagsId: selectTagId})
  })
  unlistenDoc = await listen('insert_doc', (event: {payload:DocumentTags}) => {
    docsStore.addNewDoc(event.payload)
  })
  unlistenDocUp = await listen('doc_update', (event: {payload:DocumentTags}) => {
    docsStore.updateDoc(event.payload)
  })
  window.addEventListener('blur', handleBlur);
})
onUnmounted(()=>{
  unlistenFile()
  unlistenDoc()
  unlistenDocUp()
  window.removeEventListener('blur', handleBlur);
})
watch([watchAndTags, watchOrTags], (_, _oldValue) => {
  invoke<DocumentTags[]>('query_docs_by_tags',
      {andTagsId: tagStore.andTags.map(tag => tag.id), orTagsId: tagStore.orTags.map(tag => tag.id)}
  ).then(data => {
    docsStore.setAllDocs(data)
    console.log(data)
    // docsStore.docs = data
  }).catch(e=>{message.error(e)})
},{immediate:true,deep:true})

const createColumns = (): DataTableColumns<DocumentTags> => [
  {
    type: 'expand',
    width: 20,
    expandable: (rowData: DocumentTags) => rowData.remark !== null,
    renderExpand: (rowData: DocumentTags) => {
      return h('div', { style: 'white-space: normal;color:#00BFFF;font-size:13px;font-weight:bold;margin-left:50px' }, rowData.remark || '无备注')
    }
  },
  { title: '标题', key: 'title' , resizable:true},
  {
    title: '年份',
    key: 'year',
    resizable:true,
    sorter: (rowA: DocumentTags, rowB: DocumentTags) => {
      if (rowA.year === null||rowA.year==undefined) {
        return -1
      }
      if (rowB.year === null||rowB.year==undefined) {
        return 1
      }
      return rowA.year!.localeCompare(rowB.year)
    }
  },
  { title: '刊物', key: 'journal',resizable:true},
  // {
  //   title: '标签',
  //   key: 'tags',
  //   render(row: DocumentTags) {
  //     return h(
  //         'div',
  //         row.tags.map(tag =>
  //             h(
  //                 NTag,
  //                 {
  //                   style: {
  //                     marginRight: '5px',
  //                     backgroundColor: tag.bg_color,
  //                     color: tag.text_color
  //                   },
  //                   bordered: false
  //                 },
  //                 { default: () => tag.value }
  //             )
  //         )
  //     )
  //   }
  // },
  // {
  //   title: '操作',
  //   key: 'actions',
  //   resizable:true,
  //   render(row: DocumentTags) {
  //     return h(
  //         NButton,
  //         {
  //           size: 'small',
  //           onClick: () => message.info(`打开文件 ${row.title}`)
  //         },
  //         { default: () => '打开' }
  //     )
  //   }
  // }
]
const columns = createColumns()
// 定义当前选中的行
const getRowClassName = (row: DocumentTags) => {
  if (row === docsStore.currentSelectDoc) {
    return 'highlighted-row';
  }
};

const rowClick = (row: DocumentTags) => {
  docsStore.setCurrentSelectDoc(row);
};
const handleKeyDown = (e: KeyboardEvent) => {
  if (docsStore.docs == null) return;
  if (docsStore.docs!.length === 0) return;

  if (e.key === 'ArrowUp') { // Move up
    selectedRowIndex.value = (selectedRowIndex.value - 1 + docsStore.docs.length) % docsStore.docs.length;
  } else if (e.key === 'ArrowDown') { // Move down
    selectedRowIndex.value = (selectedRowIndex.value + 1) % docsStore.docs.length;
  }
  // Update the selected document
  docsStore.setCurrentSelectDoc(docsStore.docs[selectedRowIndex.value]);
};
// 行点击事件
function setRowProps(row: DocumentTags) {
  return {
    style: {
      cursor: 'pointer'
    },
    onClick:() => rowClick(row),
    onContextmenu: (e: MouseEvent) => {
      rowClick(row)
      e.preventDefault()
      nextTick().then(() => {
        contextMenuShow.value = true
        contextOptions.x = e.clientX
        contextOptions.y = e.clientY
      })
    },
    onDblclick: () => {
      console.log("dblclick")
    },
  };
}
///-------------------------------------右键事件---------------begin---------
function deleteDoc(){
  showDocDeleteModal.value = false
  let title = docsStore.currentSelectDoc!.title;
  invoke('delete_doc', {id: docsStore.currentSelectDoc!.id}).then(_ => {
    docsStore.deleteDoc(docsStore.currentSelectDoc!.id)
    message.success(`删除文档${title}成功`)
  }).catch(e => {
    message.error(e)
  })
}
function openDocDefault(){
  if(docsStore.currentSelectDoc===undefined){return}
  invoke('open_doc_default', {path: docsStore.currentSelectDoc!.path}).then(_ => {
    console.log("open dir")
  }).catch(e => {
    message.error(e)
  })
}
function openDir(){
  if(docsStore.currentSelectDoc===undefined){return}
  invoke('open_dir', {path: docsStore.currentSelectDoc!.path}).then(_ => {
    console.log("open dir")
  }).catch(e => {
    message.error(e)
  })
}
function openWithExe(exePath:string){
  if (docsStore.currentSelectDoc === undefined){
    return
  }
  invoke('open_with_exe', {exePath: exePath, filePath: docsStore.currentSelectDoc!.path}).then(_ => {
    console.log("open by exe")
  }).catch(e => {
    message.error(e)
  })
}
///-------------------------------------右键事件---------------end---------

// const fetchSuggestions = async (query: string) => {
//   if (query === " ") { // 如果输入一个空格，则返回所有标签
//     return tagStore.allTags;
//   }
//   // 将查询字符串和标签值都转换为小写，以便进行大小写不敏感的匹配
//   const queryLower = query.toLowerCase();
//   return tagStore.allTags.filter(option =>
//       option.value.toLowerCase().includes(queryLower)
//   );
// };
</script>

<template>
  <div class="flex flex-col h-full">
    <tag-complete></tag-complete>
    <div>
<!--      这里的v-if是为了在data有数据时才渲染，不然default-expand-all无法作用-->
      <n-split direction="horizontal" v-if="docsStore.docs!==null" class="h-full mt-2" :max="1" :min="0" :default-size="0.65" >
        <template #1>
          <div @keydown="handleKeyDown" tabindex="0" class="outline-none">
            <n-scrollbar class="h-[80vh]" :size="5">
              <n-data-table
                  tabindex
                  size="small"
                  :columns="columns"
                  :data="docsStore.docs"
                  :row-props="setRowProps"
                  :row-class-name="getRowClassName"
                  :row-key="(row:DocumentTags) => row.id"
                  style="font-size: 15px"
                  :default-expand-all="configStore.config?.ui_config.table_expand ?? true"
                  striped
              >
                <template #empty>
                  没有数据
                </template>
              </n-data-table>
              <context-menu
                  v-model:show="contextMenuShow"
                  :options="contextOptions"
              >
                <context-menu-item label="用系统默认打开" @click="openDocDefault"/>
                <context-menu-item label="用WPS打开" />
                <context-menu-item
                    v-for="exe in configStore.config?.exe_configs"
                    :key="exe.name"
                    :label="`用${exe.name}打开`"
                    @click="openWithExe(exe.path)"
                >
                  <template #icon>
                    <img :src="convertFileSrc(exe.icon_path)" alt="icon">
                  </template>
                </context-menu-item>
                <context-menu-item label="打开文件目录" @click="openDir" />
                <context-menu-sperator />
                <context-menu-item label="管理标签"/>
                <context-menu-sperator />
                <context-menu-item label="删除" class="cursor-pointer" @click.stop="showDocDeleteModal=true">
                  <template #icon>
                    <inline-svg
                        src="../assets/svg/Delete24Regular.svg"
                        class="svg-button text-red-600 hover:text-red-600"
                    ></inline-svg>
                  </template>
                </context-menu-item>
              </context-menu>
            </n-scrollbar>
          </div>
        </template>
        <template #2>
          <detail />
        </template>
      </n-split>
    </div>
  </div>
  <custom-modal
      v-model:show="showDocDeleteModal"
      title="删除文件"
      :onConfirm="deleteDoc"
  >
    <div>
      <span>确定删除文件：</span>
      <span class="text-orange-600 text-base">{{ docsStore.currentSelectDoc?.title }}</span>
      <span>?</span>
      <p>删除后不可恢复，请谨慎操作。</p>
    </div>
  </custom-modal>
</template>
<style>
.n-data-table-td{
  padding: 3px!important;
}
.n-data-table-th{
  padding: 3px!important;
}
/* 定义高亮行的样式 */
.highlighted-row td {
  background-color: #88917450 !important;
}
</style>

<style scoped>
</style>