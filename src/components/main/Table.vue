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

const expandedRowKeys = ref<number[]>([]);
const expandedAll = ref<boolean>(configStore.config?.ui_config.table_expand||true);

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

// 计算表格最大高度 高度是视口高度减去两个标签栏减去底栏减去自定义的标题栏
//60px是两个标签栏高度，35px是底栏高度，30px是自定义的标题栏高度，按理说innerHeight是不包含系统那个标题栏的，但是不知道为什么
//还是要再减30px才行
const maxHeight = ref(window.innerHeight-60-30-30-35) // 对应 calc(100vh - 60px - 30px);
const handleResize = () => {
  // maxHeight.value = window.innerHeight - 60 - 30 - 35;
  maxHeight.value = window.innerHeight - 60 -30-30 - 35;
};

const handleBlur = () => {
  contextMenuShow.value = false
};

onMounted(async ()=>{
  unlistenFile = await listen('tauri://drag-drop', async (event:{ payload:{paths: string[]}})=>{
    let selectTagId = tagStore.currentSelectTags.map(tag => tag.id)
    console.log(selectTagId)
    await invoke('insert_docs', {paths: event.payload.paths, tagsId: selectTagId})
  })
  unlistenParse = await listen('summary_doc', (event: {payload:[number,boolean]}) => {
    if (event.payload[1]){
      parseIngIds.value.push(event.payload[0])
    }else{
      parseIngIds.value = parseIngIds.value.filter(id => id !== event.payload[0])
    }
  })
  unlistenDoc = await listen('insert_doc', (event: {payload:DocumentTags}) => {
    docsStore.addNewDoc(event.payload)
  })
  unlistenDocUp = await listen('doc_update', (event: {payload:DocumentTags}) => {
    docsStore.updateDoc(event.payload)
  })
  if (configStore.config?.ui_config.table_expand ?? true) {
    watch(
        () => docsStore.docs,
        (newDocs) => {
          if (newDocs) {
            expandedRowKeys.value = newDocs.map(doc => doc.id);
          }
        },
        { immediate: true, deep: true }
    );
  }
  //恢复上次使用的tag组
  tagStore.setAllAndTags(configStore.config?.ui_config.last_use_tags[0]||[]);
  tagStore.setAllOrTags(configStore.config?.ui_config.last_use_tags[1]||[]);
  window.addEventListener('blur', handleBlur);
  window.addEventListener('resize', handleResize);
  // window.addEventListener('resize', updateMaxHeight);

})
onUnmounted(()=>{
  unlistenFile()
  unlistenDoc()
  unlistenDocUp()
  unlistenParse()
  window.removeEventListener('blur', handleBlur);
  window.removeEventListener('resize', handleResize);
  // window.removeEventListener('resize', updateMaxHeight);
})

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
watch(() => expandedAll.value, (newValue, _oldValue) => {
  if (newValue&&docsStore.docs!=null){
    expandedRowKeys.value = docsStore.docs!.map(doc => doc.id);
  }else{
    expandedRowKeys.value = [];
  }
})
const createColumns = (): DataTableColumns<DocumentTags> => [
  {
    type: 'expand',
    width: 20,
    expandable: (rowData: DocumentTags) => rowData.remark !== null,
    renderExpand: (rowData: DocumentTags) => {
      return h('div', {
        style: 'white-space: normal;font-size:13px;margin-left:50px',
        class: 'text-blue-400'
      },
          rowData.remark || '无备注')
    }
  },
  {
    title: '标题',
    key: 'title' ,
    width: '70%',
    resizable:true,
    render: (rowData: DocumentTags) => {
      return h('div', {
            style: 'white-space:normal;font-size:16px',
            class: ''
          },
          rowData.title)
    }
  },
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
  { title: '刊物',
    key: 'journal',
    resizable:true,
    ellipsis: {
      tooltip: true
    }
  },
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
  if (event.button === 0 ) {
    if (row!=docsStore.currentSelectDoc&&expandedRowKeys.value.includes(row.id)){
      //如果不是当前选中的行，且该行是展开的的，什么都不要做。
      //情形为从别的行，点击一个展开的行，意味着重点关注该行，这时候不应该关闭该行的展开状态。
    }else {
      toggleExpand(row.id);
    }
  }
  docsStore.setCurrentSelectDoc(row);
};
const toggleExpand = (rowKey: number) => {
  if (expandedRowKeys.value.includes(rowKey)) {
    expandedRowKeys.value = expandedRowKeys.value.filter(key => key !== rowKey);
  } else {
    expandedRowKeys.value.push(rowKey);
  }
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
      openByApp()
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
function openBySystem(){
  if(docsStore.currentSelectDoc===undefined){return}
  invoke('open_by_system', {path: docsStore.currentSelectDoc!.path}).then(_ => {}).catch(e => {message.error(e)})
}
function openByApp(){
  if(docsStore.currentSelectDoc===undefined){return}
  invoke('open_by_app', {path: docsStore.currentSelectDoc!.path}).then(_ => {}).catch(e => {message.error(e)})
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
function summaryByAi(){
  let docs = selectedRows.value;
  if (docs.length === 0)return
  invoke('summarize_docs_by_ai', {documentTagsList: docs}).then(_ => {
  }).catch(e => {
    message.error(e)
  })
}
///-------------------------------------右键事件---------------end---------
const handleDragEnd = () => {
  //注意，这个配置改变config那边没监听到，但是最后保存时是没问题的。
  configStore.config!.ui_config.split_size[1] = parseFloat(splitSize.value.toFixed(2));
};
const scrollbarSize= ref(10)
</script>

<template>
  <div class="flex flex-col h-full">
    <tag-complete></tag-complete>
    <div>
<!--      这里的v-if是为了在data有数据时才渲染，不然default-expand-all无法作用-->
      <n-split direction="horizontal" v-if="docsStore.docs!==null" class="h-full" :max="1" :min="0" :size="splitSize"  @update:size="(e:number) => splitSize = e" @drag-end="handleDragEnd" >
        <template #1>
          <div @keydown="handleKeyDown" tabindex="0" class="outline-none">
<!--            高度是视口高度减去两个标签栏减去底栏-->
<!--            <n-scrollbar class="h-[calc(100vh-60px-30px)]" :size="scrollbarSize">-->
<!--            </n-scrollbar>-->
            <n-data-table
                ref="tableRef"
                size="small"
                :columns="columns"
                :data="docsStore.docs"
                :row-props="setRowProps"
                :row-class-name="getRowClassName"
                :row-key="(row:DocumentTags) => row.id"
                style="font-size: 15px;user-select: none;width: calc(100% - 5px);"
                :max-height="maxHeight"
                :expanded-row-keys="expandedRowKeys"
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
              <context-menu-item label="用系统默认应用打开" @click="openBySystem"/>
              <context-menu-item label="用天书默认应用打开" @click="openByApp"/>
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
              <context-menu-item label="打开文件所在目录" @click="openDir" />
              <context-menu-sperator />
              <context-menu-item :label="selectedRows.length === 1 ? '用AI总结' : `用AI总结这${selectedRows.length}条文档`" @click="summaryByAi">
                <template #icon>
                  <inline-svg
                      src="../assets/svg/ai.svg"
                      class="svg-button"
                  ></inline-svg>
                </template>
              </context-menu-item>
              <context-menu-sperator />
              <context-menu-item
                  :label="expandedAll ? '关闭所有可展开行' : '展开所有可展开行'"
                  @click="expandedAll = !expandedAll"
              ></context-menu-item>
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