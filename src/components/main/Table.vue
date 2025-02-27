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
  {
    title: '序号',
    key: 'index',
    render: (_, index) => h('span', index + 1)
  },
  { title: '标题', key: 'title' },
  { title: '作者', key: 'author' },
  { title: '摘要', key: 'abstract' },
  { title: '年份', key: 'year' },
  { title: '期刊', key: 'journal' },
  { title: '贡献', key: 'contributions' },
  {
    title: '标签',
    key: 'tags',
    render(row: DocumentTags) {
      return h(
          'div',
          row.tags.map(tag =>
              h(
                  NTag,
                  {
                    style: {
                      marginRight: '6px',
                      backgroundColor: tag.bg_color,
                      color: tag.text_color
                    },
                    bordered: false
                  },
                  { default: () => tag.value }
              )
          )
      )
    }
  },
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

// const data = createData()
// const columns = createColumns({
//   sendMail(rowData) {
//     message.info(`send mail to ${rowData.name}`)
//   }
// })
const pagination = {
  pageSize: 10
}

const fetchSuggestions = async (query: string) => {
  if (query===" "){ //如果输入一个空格，则返回所有标签
    return tagStore.allTags
  }
  return tagStore.allTags.filter(option =>
      option.value.includes(query)
  )
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
            <n-data-table
                :columns="columns"
                :data="docsStore.docs"
                :pagination="pagination"
                :min-height="800"
                class="size-full"
                default-expand-all
            />
          </template>
          <template #2>
            <div>
              {{docsStore.docs}}
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