<script setup lang="ts">
import AutoCompleteTag from "../../util/AutoCompleteTag.vue";
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {DocumentTags} from "./main-type.ts";
import { h } from 'vue'
import {DataTableColumns, NButton, NTag} from "naive-ui";
import {message} from "../../message.ts";
import { listen } from '@tauri-apps/api/event'
import {invoke} from "@tauri-apps/api/core";
import useDocStore from "../../stroe/doc.ts";

const tagStore = useTagGroupsStore()
const docsStore = useDocStore()

//不知道为什么直接监听tagStore.两个Tags不可以，只有添加的时候可以监听到，删除一次后执行任何操作不会触发监听，
const watchAndTags = computed(() => tagStore.andTags)
const watchOrTags = computed(() => tagStore.orTags)

let unlistenFile: () => void;
let unlistenDoc: () => void;

onMounted(async ()=>{
  unlistenFile = await listen('tauri://drag-drop', async (event:{ payload:{paths: string[]}})=>{
    console.log(event.payload.paths)
    let selectTagId = tagStore.currentSelectTags.map(tag => tag.id)
    await invoke('insert_docs', {paths: event.payload.paths, tagsId: selectTagId})
  })
  unlistenDoc = await listen('insert_doc', (event: {payload:DocumentTags}) => {
    docsStore.addNewDoc(event.payload)
    console.log(event.payload)
  })
})
onUnmounted(()=>{
  unlistenFile()
  unlistenDoc()
})
watch([watchAndTags, watchOrTags], (_, _oldValue) => {
  console.log('doubleValue changed to:')
  invoke<DocumentTags[]>('query_docs_by_tags',
      {andTagsId: tagStore.andTags.map(tag => tag.id), orTagsId: tagStore.orTags.map(tag => tag.id)}
  ).then(data => {
    console.log(data)
    docsStore.docs = data
  }).catch(e=>{message.error(e)})
},{immediate:true,deep:true})

const createColumns = (): DataTableColumns<DocumentTags> => [
  {
    type: 'expand',
    expandable: (rowData: DocumentTags) => rowData.remark !== null,
    renderExpand: (rowData: DocumentTags) => {
      return h('div', { style: 'white-space: normal;' }, rowData.remark || '无备注')
    }
  },
  { title: '标题', key: 'title' , resizable:true, width:'60%'},
  // { title: '作者', key: 'author' },
  { title: '年份', key: 'year' },
  { title: '刊物', key: 'journal' },
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

const pagination = {
  pageSize: 10
}
// 行点击事件
const onRowClick = (row: DocumentTags) => {
  message.info(`点击了行：${row.id}`);
  console.log('Clicked row:', row);
};

const fetchSuggestions = async (query: string) => {
  if (query===" "){ //如果输入一个空格，则返回所有标签
    return tagStore.allTags
  }
  return tagStore.allTags.filter(option =>
      option.value.includes(query)
  )
}
const currentSelectDoc = ref<DocumentTags>()
function setRowProps(row: DocumentTags) {
  return {
    style: {
      cursor: 'pointer'
    },
    onClick: () => {
      currentSelectDoc.value = row
      onRowClick(row);
    }
  };
}
</script>

<template>
  <div>
    <n-flex vertical class="size-full">
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
      <div>
        <n-split direction="horizontal" class="size-full" :max="1" :min="0" :default-size="0.7">
          <template #1>
            <n-scrollbar style="max-height: 800px">
              <n-data-table
                  :columns="columns"
                  :data="docsStore.docs"
                  :pagination="pagination"
                  :header-height="50"
                  :flex-height="true"
                  :min-height="500"
                  :row-props="setRowProps"
                  default-expand-all
              />
            </n-scrollbar>
          </template>
          <template #2>
            <div>
              <n-card title="文档详细信息">
                <n-flex align="center" justify="space-between" style="margin-bottom: 20px;">
                  <div class="title-container">
                    <n-text tag="h2" class="text-green-700" >标题: {{ currentSelectDoc?.title }}</n-text>
                    <n-button type="info" size="tiny" style="margin-left: 10px;">
                      编辑标题
                    </n-button>
                  </div>
                </n-flex>
              </n-card>
              <n-card title="文档标签信息">
                <n-flex style="margin-top: 20px;">
                  <n-button type="success" size="small">添加标签</n-button>
                </n-flex>
                <div style="margin-top: 10px;">
                  <n-tag
                      v-for="tag in currentSelectDoc?.tags"
                      :key="tag.id"
                      :style="{ backgroundColor: tag.bg_color, color: tag.text_color }"
                      size="medium"
                      class="tag-item"
                  >
                    {{ tag.value }}
                  </n-tag>
                </div>
              </n-card>
            </div>
          </template>
        </n-split>
      </div>
    </n-flex>
  </div>

</template>

<style scoped>
/*.tag :deep(g) {
  @apply  fill-black;
}
.tag :deep(button) {
  @apply  hover:border-red-600 shadow-none active:border-transparent;
}*/
</style>