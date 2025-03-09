<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {DocumentTags} from "./main-type.ts";
import { h } from 'vue'
import {DataTableColumn, DataTableColumns, NSpin} from "naive-ui";
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
let unlistenParse: () => void;

const selectedRowIndex = ref(-1);
const selectedRows = ref<DocumentTags[]>([]); // Store selected rows
const lastClickedIndex = ref<number | null>(null); // Store the index of the last clicked row
const parseIngIds = ref<number[]>([])
const tableRef = ref()

const splitSize = ref(configStore.config?.ui_config.split_size[1]||0.65);

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
    console.log(selectTagId)
    await invoke('insert_docs', {paths: event.payload.paths, tagsId: selectTagId})
  })
  unlistenParse = await listen('parse_doc', (event: {payload:[boolean,number]}) => {
    if (event.payload[0]){
      parseIngIds.value.push(event.payload[1])
    }else{
      parseIngIds.value = parseIngIds.value.filter(id => id !== event.payload[1])
    }
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
  unlistenParse()
  window.removeEventListener('blur', handleBlur);
})
// watch(()=>configStore.config,async (_newValue, oldValue)=>{
//   if (oldValue==undefined){
//     splitSize.value = configStore.config?.ui_config.split_size[1]||0.65;
//   }
// })
watch([watchAndTags, watchOrTags], (_, _oldValue) => {
  invoke<DocumentTags[]>('query_docs_by_tags',
      {andTagsId: tagStore.andTags.map(tag => tag.id), orTagsId: tagStore.orTags.map(tag => tag.id)}
  ).then(data => {
    docsStore.setAllDocs(data)
  }).catch(e=>{message.error(e)})
},{immediate:true,deep:true})
// 监听currentSelectDoc的变化
watch(
    () => docsStore.currentSelectDoc,
    (newVal) => {
      // 使用唯一标识符比较（假设每个文档有唯一id）
      if (docsStore.docs==null||newVal==undefined) return
      const index = docsStore.docs.findIndex(doc => doc.id === newVal.id)
      selectedRowIndex.value = index >= 0 ? index : -1
    },
    {
      deep: true,      // 深度监听对象内部变化
      immediate: true  // 立即触发一次初始值计算
    }
)
watch(() => parseIngIds.value, (newValue, _oldValue) => {
  //不知道为什么第一次变化时old和new一样的，所以只能判断有没有解析列了
  const hasParseColumn = columns.value.some(column => column!.key === parseColumn!.key);
  if (newValue.length>0&&!hasParseColumn){
    columns.value.push(parseColumn)
  }else if (newValue.length==0){
    columns.value = columns.value.filter(column => column?.key !== parseColumn?.key);
  }
},{deep:true})
const createColumns = (): DataTableColumns<DocumentTags> => [
  {
    type: 'expand',
    width: 20,
    expandable: (rowData: DocumentTags) => rowData.remark !== null,
    renderExpand: (rowData: DocumentTags) => {
      return h('div', {
        style: 'white-space: normal;font-size:13px;font-weight:bold;margin-left:50px',
        class: 'text-blue-600'
      },
          rowData.remark || '无备注')
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
let columns = ref<DataTableColumns<DocumentTags>>(createColumns())
const parseColumn: DataTableColumn<DocumentTags> = {
  title: '解析',
  key: 'parse',
  render(row: DocumentTags) {
    if (parseIngIds.value.includes(row.id)) {
      return h(NSpin, { size: 13 });
    }
  },
};
// 定义当前选中的行
const getRowClassName = (row: DocumentTags) => {
  if (selectedRows.value.includes(row)||docsStore.currentSelectDoc===row) {
    return "highlighted-row";
  }
};

const rowClick = (row: DocumentTags, index: number, event: MouseEvent) => {
  if (event.button === 2) {
    if (selectedRows.value.length>1){
      return
    }
  }
  const ctrlKey = event.ctrlKey;
  const shiftKey = event.shiftKey;

  if (ctrlKey) {
    // Ctrl + Click: Toggle selection
    lastClickedIndex.value = index;
    if (selectedRows.value.includes(row)) {
      selectedRows.value = selectedRows.value.filter((r) => r !== row);
      return;//不要把该行设置为选中行，要提前返回
    } else {
      selectedRows.value = [...selectedRows.value, row];
    }
  } else if (shiftKey && lastClickedIndex.value !== null) {
    // Shift + Click: Select range
    const startIndex = Math.min(index, lastClickedIndex.value);
    const endIndex = Math.max(index, lastClickedIndex.value);

    selectedRows.value = []; // Clear previous selection
    for (let i = startIndex; i <= endIndex; i++) {
      if (docsStore.docs) { // Check if docsStore.docs is not null
        for (let i = startIndex; i <= endIndex; i++) {
          if (!selectedRows.value.includes(docsStore.docs[i])) {
            selectedRows.value.push(docsStore.docs[i]);
          }
        }
      }
    }
  } else {
    // Single click: Select only this row
    selectedRows.value = [row];
    lastClickedIndex.value = index;
  }
  docsStore.setCurrentSelectDoc(row);
};
const handleKeyDown = async (e: KeyboardEvent) => {
  if (docsStore.docs == null) return;
  if (docsStore.docs!.length === 0) return;
  if (e.key === 'ArrowUp') { // Move up
    selectedRows.value = []
    selectedRowIndex.value = (selectedRowIndex.value - 1 + docsStore.docs.length) % docsStore.docs.length;
    lastClickedIndex.value = selectedRowIndex.value;
  } else if (e.key === 'ArrowDown') { // Move down
    selectedRows.value = []
    selectedRowIndex.value = (selectedRowIndex.value + 1) % docsStore.docs.length;
    lastClickedIndex.value = selectedRowIndex.value;
  }
  // Update the selected document
  docsStore.setCurrentSelectDoc(docsStore.docs[selectedRowIndex.value]);
  // 等待 DOM 更新
  await nextTick()
  // 执行滚动
  scrollToSelectedRow()
};
//todo 滚动没作用
const scrollToSelectedRow = () => {
  const table = tableRef.value
  if (!table || selectedRowIndex.value === -1||docsStore.docs==null) return
  const selectedId = docsStore.docs[selectedRowIndex.value].id
  table.scrollTo({ rowKey: selectedId })
}
// 行点击事件
function setRowProps(row: DocumentTags,index: number) {
  return {
    style: {
      cursor: 'pointer'
    },
    onClick:(e: MouseEvent) => rowClick(row, index, e),
    onContextmenu: (e: MouseEvent) => {
      rowClick(row, index, e)
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
function deleteDocs(){
  showDocDeleteModal.value = false
  //拿出selrows的id组成一个数组
  let ids = selectedRows.value.map(row => row.id)
  // let title = docsStore.currentSelectDoc!.title;
  // invoke('delete_docs', {id: docsStore.currentSelectDoc!.id}).then(_ => {
  invoke('delete_docs', {ids: ids}).then(_ => {
    console.log(ids)
    selectedRows.value = []
    docsStore.deleteDocs(ids)
    message.success(`删除文档成功`)
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
const handleDragEnd = () => {
  //注意，这个配置改变config那边没监听到，但是最后保存时是没问题的。
  configStore.config!.ui_config.split_size[1] = parseFloat(splitSize.value.toFixed(2));
};
</script>

<template>
  <div class="flex flex-col h-full">
    <tag-complete></tag-complete>
    <div>
<!--      这里的v-if是为了在data有数据时才渲染，不然default-expand-all无法作用-->
      <n-split direction="horizontal" v-if="docsStore.docs!==null" class="h-full mt-2" :max="1" :min="0" :size="splitSize"  @update:size="(e:number) => splitSize = e" @drag-end="handleDragEnd" >
        <template #1>
          <div @keydown="handleKeyDown" tabindex="0" class="outline-none">
            <n-scrollbar class="h-[80vh]" :size="5">
              <n-data-table
                  ref="tableRef"
                  size="small"
                  :columns="columns"
                  :data="docsStore.docs"
                  :row-props="setRowProps"
                  :row-class-name="getRowClassName"
                  :row-key="(row:DocumentTags) => row.id"
                  style="font-size: 15px;user-select: none"
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
                <context-menu-item label="用AI总结文档" @click="">
                  <template #icon>
                    <inline-svg
                        src="../assets/svg/ai.svg"
                        class="svg-button"
                    ></inline-svg>
                  </template>
                </context-menu-item>
                <context-menu-item
                    v-for="exe in configStore.config?.exe_configs"
                    :key="exe.name"
                    :label="`用${exe.name}打开`"
                    @click="openWithExe(exe.path)"
                >
                  <template #icon>
                    <img :src="convertFileSrc(exe.icon_path)" alt="icon" class="w-4 h-4">
                  </template>
                </context-menu-item>
                <context-menu-item label="打开文件目录" @click="openDir" />
                <context-menu-sperator />
                <context-menu-item label="管理标签"></context-menu-item>
                <context-menu-sperator />
                <context-menu-item  :label="selectedRows.length === 1 ? '删除' : `删除(已选中${selectedRows.length}条)`" class="cursor-pointer" @click.stop="showDocDeleteModal=true">
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
      :onConfirm="deleteDocs"
  >
    <div>
      <span v-if="selectedRows.length === 1">
        确定删除文件：<span class="text-orange-600 text-base">{{ docsStore.currentSelectDoc?.title }}</span>?<p>删除后不可恢复，请谨慎操作。</p>
      </span>
      <span v-else>
        你确定要删除这<span class="text-orange-600 text-base">{{ selectedRows.length }}</span>条？删除后不可恢复，请谨慎操作。
      </span>
<!--      <span>确定删除文件：</span>-->
<!--      <span class="text-orange-600 text-base">{{ docsStore.currentSelectDoc?.title }}</span>-->
<!--      <span>?</span>-->
<!--      <p>删除后不可恢复，请谨慎操作。</p>-->
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