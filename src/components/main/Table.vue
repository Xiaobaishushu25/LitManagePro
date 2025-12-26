<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {DocumentTags, TagResponse} from "./main-type.ts";
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

// let unlistenFile: () => void;
let unlistenDoc: () => void;
let unlistenDocUp: () => void;
let unlistenDocPathUp: () => void;
let unlistenParse: () => void;
let unlisten1: () => void;
let unlisten2: () => void;

const selectedRowIndex = ref(-1);
//会监听docsStore.docs的currentSelectDoc变化自动改变，也会根据多选点击来改变
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
  zIndex: 3,
  x: 500,
  y: 200,
  minWidth:300,
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
  unlisten1 = await listen('关闭/打开所有可展开行', () => {
    expandedAll.value = !expandedAll.value
  })
  unlisten2 = await listen('复制文件', () => {
    copyToClipboard()
  })
  // 监听summary_doc事件，其中payload是[id,true]表示开始解析，[id,false]表示解析完成
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
  unlistenDocPathUp = await listen('docs_path_update', (event: {payload:Record<number, string>}) => {
    let payload = event.payload;
    for (const key in payload) {
      const value = payload[key];
      // console.log(`Key: ${key}, Value: ${value}`);
      docsStore.updateDocPathById(Number(key), value)
    }
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
  await nextTick(() => {
    //todo 本来想在应用启动后焦点给到表格，这样就能直接上下键切换文献了，但是发现没用。
    tableRef.value?.$el.focus();
  })
})
onUnmounted(()=>{
  // unlistenFile()
  unlisten1()
  unlisten2()
  unlistenDoc()
  unlistenDocUp()
  unlistenDocPathUp()
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
      if (selectedRows.value.length==0){
        //如果当前选中文档为空，则选中当前文档，其实这个判断有待商榷，好像不用加也可以，因为肯定不能多选的时候上下键切换
        selectedRows.value = newVal ? [newVal] : []
      }
      const index = docsStore.docs.findIndex(doc => doc.id === newVal.id)
      selectedRowIndex.value = index >= 0 ? index : -1
    },
    {deep: true, immediate: true}
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
    ellipsis: {
      tooltip: true
    },
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
    selectedRows.value = []  //注意，这个不能移出去，因为只有上下箭头时才可以删除
    selectedRowIndex.value = (selectedRowIndex.value - 1 + docsStore.docs.length) % docsStore.docs.length;
    lastClickedIndex.value = selectedRowIndex.value;
  } else if (e.key === 'ArrowDown') { // Move down
    selectedRows.value = []//注意，这个不能移出去，因为只有上下箭头时才可以删除
    selectedRowIndex.value = (selectedRowIndex.value + 1) % docsStore.docs.length;
    lastClickedIndex.value = selectedRowIndex.value;
  }
  let doc = docsStore.docs[selectedRowIndex.value];
  // selectedRows.value = [doc]
  // Update the selected document
  docsStore.setCurrentSelectDoc(doc);
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
  let lastClickTime = 0;
  return {
    style: {
      cursor: 'pointer'
    },
    onClick:(e: MouseEvent) => {
      const now = Date.now();
      // 300ms 内第二次点击视为双击前导点击，直接忽略
      if (now - lastClickTime < 300) return;

      lastClickTime = now;
      if (e.detail === 1) {
        rowClick(row, index, e);
      }
      // rowClick(row, index, e);
    },
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
      lastClickTime = 0; // 双击后重置时间
      openByApp();
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
function copyToClipboard(){
  let docs = selectedRows.value;
  if (docs.length === 0)return
  let filePaths = docs.map(doc => doc.path);
  console.log(filePaths)
  invoke('copy_files_to_clipboard', {filePaths: filePaths}).then(_ => {
    message.success(`成功复制到剪切板`)
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
function suggestTagsByAi(){
  let docs = selectedRows.value;
  let doc = docs[docs.length-1]
  console.log(doc)
  invoke<TagResponse>('suggest_tag_by_ai', {pathS: doc.path,docId: doc.id,tagAndGroups:tagStore.tagGroups}).then(data => {
    console.log(data)
    message.success("成功获取ai建议标签")
  }).catch(e => {
    console.log("进入ai建议标签功能出错")
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
                style="font-size: 15px;user-select: none;"
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
              <context-menu-item label="打开文件所在目录" @click="openDir"/>
              <context-menu-item
                  :label="selectedRows.length === 1 ? '复制' : `复制这${selectedRows.length}条文档`"
                  shortcut="?"
                  @click="copyToClipboard">
                <template #icon>
                  <inline-svg
                      src="../assets/svg/DocumentCopy48Regular.svg"
                      class="svg-button"
                  ></inline-svg>
                </template>
                <template #shortcut>
                  <span class="bg-gray-100 text-gray-600 px-2 py-1 rounded-md text-xs font-mono">{{configStore.getShortcutByName('复制文件')}}</span>
                </template>
              </context-menu-item>
              <context-menu-sperator />
              <context-menu-item
                  :label="selectedRows.length === 1 ? '用AI总结' : `用AI总结这${selectedRows.length}条文档`"
                  @click="summaryByAi">
                <template #icon>
                  <inline-svg
                      src="../assets/svg/ai.svg"
                      class="svg-button"
                  ></inline-svg>
                </template>
              </context-menu-item>
              <context-menu-item
                  label="用AI建议标签"
                  @click="suggestTagsByAi">
                <template #icon>
                  <inline-svg
                      src="../assets/svg/ai.svg"
                      class="svg-button"
                  ></inline-svg>
                </template>
              </context-menu-item>
              <context-menu-sperator />
              <context-menu-item
                  @click="expandedAll = !expandedAll"
                  class="outline-none context-menu-item"
              >
                <template #label>
                  {{expandedAll ? '关闭所有可展开行' : '展开所有可展开行'}}
                </template>
                <template #shortcut>
                  <span class="bg-gray-100 text-gray-600 px-2 py-1 rounded-md text-xs font-mono">{{configStore.getShortcutByName('关闭/打开所有可展开行')}}</span>
                </template>
              </context-menu-item>
              <context-menu-sperator />
              <context-menu-item label="管理标签"></context-menu-item>
              <context-menu-sperator />
<!--              使用键盘选中时变红需要额外设置：https://github.com/imengyu/vue3-context-menu/issues/115#issuecomment-2764879969 -->
              <context-menu-item
                  id="delete-menu-item"
                  class="group"
                  @click.stop="showDocDeleteModal = true"
              >
                <template #icon>
                  <inline-svg
                      src="../assets/svg/Delete24Regular.svg"
                      class="svg-button group-hover:text-red-600 "
                  ></inline-svg>
                </template>
                <template #label>
                  <span class="group-hover:text-red-600">{{ selectedRows.length === 1 ? '删除' : `删除(已选中${selectedRows.length}条)` }}</span>
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