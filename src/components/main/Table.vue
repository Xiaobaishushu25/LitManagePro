<script setup lang="ts">
import AutoCompleteTag from "../../util/AutoCompleteTag.vue";
import {computed, onMounted, onUnmounted, watch} from "vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {DocumentTags} from "./main-type.ts";
import { h } from 'vue'
import {DataTableColumns, NButton} from "naive-ui";
import {message} from "../../message.ts";
import { listen } from '@tauri-apps/api/event'
import {invoke} from "@tauri-apps/api/core";
import useDocStore from "../../stroe/doc.ts";
import Detail from "./Detail.vue";
import useConfigStore from "../../stroe/config.ts";

const tagStore = useTagGroupsStore()
const docsStore = useDocStore()
const configStore = useConfigStore()

//不知道为什么直接监听tagStore.两个Tags不可以，只有添加的时候可以监听到，删除一次后执行任何操作不会触发监听，
const watchAndTags = computed(() => tagStore.andTags)
const watchOrTags = computed(() => tagStore.orTags)

let unlistenFile: () => void;
let unlistenDoc: () => void;
let unlistenDocUp: () => void;

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
})
onUnmounted(()=>{
  unlistenFile()
  unlistenDoc()
  unlistenDocUp()
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
      return h('div', { style: 'white-space: normal;color:orange;margin-left:50px' }, rowData.remark || '无备注')
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
  {
    title: '操作',
    key: 'actions',
    resizable:true,
    render(row: DocumentTags) {
      return h(
          NButton,
          {
            size: 'small',
            onClick: () => message.info(`打开文件 ${row.title}`)
          },
          { default: () => '打开' }
      )
    }
  }
]

const columns = createColumns()

// 定义当前选中的行
const getRowClassName = (row: DocumentTags) => {
  if (row === docsStore.currentSelectDoc) {
    return 'highlighted-row';
  }
};

// 行点击事件
// const currentSelectDoc = ref<DocumentTags>()
function setRowProps(row: DocumentTags) {
  return {
    style: {
      cursor: 'pointer'
    },
    onClick: () => {
      docsStore.setCurrentSelectDoc(row)
    }
  };
}

const fetchSuggestions = async (query: string) => {
  if (query === " ") { // 如果输入一个空格，则返回所有标签
    return tagStore.allTags;
  }
  // 将查询字符串和标签值都转换为小写，以便进行大小写不敏感的匹配
  const queryLower = query.toLowerCase();
  return tagStore.allTags.filter(option =>
      option.value.toLowerCase().includes(queryLower)
  );
};
</script>

<template>
  <div class="flex flex-col h-full">
    <auto-complete-tag
        v-model:modelValue="tagStore.andTags"
        placeholder="按空格提示所有标签"
        :options="tagStore.allTags"
        :fetchSuggestions="fetchSuggestions"
    />
    <auto-complete-tag
        v-model:modelValue="tagStore.orTags"
        placeholder="按空格提示所有标签"
        :options="tagStore.allTags"
        :fetchSuggestions="fetchSuggestions"
    />
    <div >
<!--      :default-expand-all="configStore.config?.ui_config.table_expand ?? true"-->
      <n-split direction="horizontal" v-if="docsStore.docs!==null" class="h-full mt-2" :max="1" :min="0" :default-size="0.8" >
        <template #1>
          <n-data-table
              size="small"
              :columns="columns"
              :data="docsStore.docs"
              :row-props="setRowProps"
              :row-class-name="getRowClassName"
              :row-key="(row) => row.id"
              class="h-[calc(100vh-20rem)]"
              :default-expand-all="configStore.config?.ui_config.table_expand ?? true"
              striped
          >
            <template #empty>
              没有数据
            </template>
          </n-data-table>
        </template>
        <template #2>
          <detail />
        </template>
      </n-split>
    </div>
  </div>
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