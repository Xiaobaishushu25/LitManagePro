<script setup lang="ts">
import {
  NTag,
  NTree,
  TransferRenderSourceList,
  TreeOption,
} from "naive-ui";
import useDocStore from "../../stroe/doc.ts";
import {computed, h, ref, watch, watchEffect} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {message} from "../../message.ts";
import CustomModal from "../../util/CustomModal.vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import EditableText from "../../util/EditableText.vue";
import {Document} from "./main-type.ts"

const docsStore = useDocStore()
const tagStore = useTagGroupsStore()

const showTagModal = ref(false)

const canTagDelete = ref(false)

const selectedTags = ref<(string | number)[]>()

watchEffect(() => {
  selectedTags.value = docsStore.currentSelectDoc?.tags.map(tag => tag.id)
})

// 转换树形数据结构
const treeData = ref<TreeOption[]>([])
const flatTags = ref<TreeOption[]>([])

// 监听tagGroups变化生成数据结构
watchEffect(() => {
  treeData.value = tagStore.tagGroups.map(group => ({
    label: group.tag_group.name,
    key: `group-${group.tag_group.id}`,
    disabled: true, // 禁用一级节点
    checkable: false,
    children: group.tags.map(tag => ({
      label: tag.value,
      key: tag.id,
      style: {
        color: tag.text_color,
        backgroundColor: tag.bg_color
      }
    }))
  }))

  flatTags.value = tagStore.tagGroups.flatMap(group =>
      group.tags.map(tag => ({
        label: tag.value,
        value: tag.id,
        style: {
          color: tag.text_color,
          backgroundColor: tag.bg_color
        }
      }))
  )
})

// 默认展开所有分组
// const defaultExpandedKeys = ref<string[]>(
//     tagStore.tagGroups.map(g => `group-${g.tag_group.id}`)
// )
const defaultExpandedKeys = computed(() => {
  return tagStore.tagGroups.map(g => `group-${g.tag_group.id}`);
});
// 处理穿梭框源列表渲染
const renderSourceList: TransferRenderSourceList = ({ onCheck, pattern }) => {
  return h(NTree, {
    style: 'margin: 0 4px;',
    keyField: 'key',
    checkable: true,
    selectable: false,
    blockLine: true,
    checkOnClick: true,
    data: treeData.value,
    pattern,
    defaultExpandedKeys: defaultExpandedKeys.value,
    checkedKeys: selectedTags.value,
    onUpdateCheckedKeys: (keys) => {
      // 过滤掉分组节点的key
      const validKeys = keys.filter(k =>
          !k.toString().startsWith('group-')
      )
      onCheck(validKeys)
      selectedTags.value = validKeys
    },
  })
}

function deleteTag(tagId: number){
  invoke('delete_doc_tag', {docId: docsStore.currentSelectDoc?.id, tagId: tagId})
      .then(_ => {})
      .catch(e => {message.error(e)})
}
function updateTags(){
  showTagModal.value = false
  if(docsStore.currentSelectDoc===undefined){
    return
  }
  invoke('update_doc_tags', {docId: docsStore.currentSelectDoc?.id, tagIds: selectedTags.value})
      .then(_ => {})
      .catch(e => {message.error(e)})
}
const authorEdit = ref()
const titleEdit = ref()
const fileNameEdit = ref()
const abstractEdit = ref()
const yearEdit = ref()
const journalEdit = ref()
const remarkEdit = ref()
const contributionsEdit = ref()
watch(() => docsStore.currentSelectDoc, () => {
  remarkEdit.value = docsStore.currentSelectDoc?.remark
  titleEdit.value = docsStore.currentSelectDoc?.title
  fileNameEdit.value = docsStore.currentSelectDoc?.file_name
  abstractEdit.value = docsStore.currentSelectDoc?.abstract
  yearEdit.value = docsStore.currentSelectDoc?.year
  journalEdit.value = docsStore.currentSelectDoc?.journal
  authorEdit.value = docsStore.currentSelectDoc?.author
  contributionsEdit.value = docsStore.currentSelectDoc?.contributions
})
function updateDoc(){
  if(docsStore.currentSelectDoc===undefined){
    return
  }
  const newDoc:Document = {
    index: docsStore.currentSelectDoc?.index,
    id: docsStore.currentSelectDoc?.id,
    title: titleEdit.value,
    file_name: fileNameEdit.value,
    author: authorEdit.value,
    year: yearEdit.value,
    journal:journalEdit.value,
    abstract: abstractEdit.value,
    remark: remarkEdit.value,
    contributions: contributionsEdit.value,
    path: docsStore.currentSelectDoc?.path
  }
  invoke('update_doc_detail', {doc:newDoc})
      .then(_ => {
        message.success("修改成功")
      })
      .catch(e => {
        message.error(e)
      })
}
</script>

<template>
  <custom-modal title="修改标签"  v-model:show="showTagModal" :on-confirm="updateTags">
    <n-transfer
        v-if="docsStore.currentSelectDoc"
        v-model:value="selectedTags"
        :options="flatTags"
        :render-source-list="renderSourceList"
        source-filterable
        size="large"
        class="h-[66.666vh] min-h-[66.666vh] min-w-[500px] w-[500px]"
    />
    <div v-else>请选择一个文档。</div>
  </custom-modal>
  <div>
<!--    60px是两个标签栏高度，35px是底栏高度，30px是自定义的标题栏高度，按理说innerHeight是不包含系统那个标题栏的，
        但是不知道为什么，还是要再减30px才行-->
    <n-scrollbar class="h-[calc(100vh-60px-30px-30px)]">
      <n-card  title="详细信息">
        <div class="grid grid-cols-[55px_1fr] gap-4 items-center">
          <div class="text-right text-sm">标题:</div>
          <div class="text-left text-base">
            <editable-text
                v-model:value="titleEdit"
                :on-save="updateDoc"
                content-class="font-bold"
            />
          </div>
          <div class="text-right text-sm">文件名:</div>
          <div class="text-left text-base">
            <editable-text
                v-model:value="fileNameEdit"
                :on-save="updateDoc"
                content-class="font-bold"
            />
          </div>
          <div class="text-right text-sm">作者:</div>
          <div class="text-left text-base">
            <editable-text
                v-model:value="authorEdit"
                :on-save="updateDoc"
                content-class="font-bold"
            />
          </div>
          <div class="text-right text-sm">刊物:</div>
          <div class="text-left text-base">
            <editable-text
                v-model:value="journalEdit"
                :on-save="updateDoc"
                content-class="font-bold"
            />
          </div>
          <div class="text-right text-sm">年份:</div>
          <div class="text-left text-base">
            <editable-text
                v-model:value="yearEdit"
                :on-save="updateDoc"
                content-class="font-bold"
            />
          </div>
        </div>
      </n-card>
      <n-card title="标签信息">
        <template #header-extra>
          <n-flex>
            <n-tooltip trigger="hover" class="text-xs p-0">
              <template #trigger>
                <inline-svg
                    src="../assets/svg/Delete24Regular.svg"
                    class="svg-button hover:text-red-600"
                    @click.stop="canTagDelete = !canTagDelete"
                ></inline-svg>
              </template>
              删除标签
            </n-tooltip>
            <n-tooltip trigger="hover" class="text-xs p-0">
              <template #trigger>
                <inline-svg
                    src="../assets/svg/Edit24Regular.svg"
                    class="svg-button"
                    @click.stop="showTagModal=true"
                ></inline-svg>
              </template>
              编辑标签
            </n-tooltip>
          </n-flex>
        </template >
        <div>
          <n-tag
              v-for="tag in docsStore.currentSelectDoc?.tags"
              :key="tag.id"
              :style="{ backgroundColor: tag.bg_color, color: tag.text_color }"
              size="medium"
              :closable="canTagDelete"
              @close="deleteTag(tag.id)"
              class="mx-1 mt-1"
          >
            {{ tag.value }}
          </n-tag>
        </div>
      </n-card>
      <n-card title="核心思想" class="font-bold">
        <editable-text v-model:value="remarkEdit" :on-save="updateDoc" content-class="text-base text-blue-600 font-bold" />
      </n-card>
      <n-card title="摘要">
        <editable-text v-model:value="abstractEdit" :on-save="updateDoc" content-class="text-base"/>
      </n-card>
      <n-card title="创新点">
        <editable-text v-model:value="contributionsEdit" :on-save="updateDoc" content-class="text-base"/>
      </n-card>
    </n-scrollbar>
  </div>
</template>
<style scoped>

</style>