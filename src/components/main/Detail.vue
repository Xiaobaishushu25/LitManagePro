<script setup lang="ts">
import {
  NTag,
  NTree,
  TransferRenderSourceList,
  TreeOption,
} from "naive-ui";
import useDocStore from "../../stroe/doc.ts";
import {h, ref, watchEffect} from "vue";
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {message} from "../../message.ts";
import CustomModal from "../../util/CustomModal.vue";
import useTagGroupsStore from "../../stroe/tag.ts";

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
const defaultExpandedKeys = ref<string[]>(
    tagStore.tagGroups.map(g => `group-${g.tag_group.id}`)
)

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
  console.log("update tag")
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
    <n-scrollbar class="h-[86vh]">
      <n-card  title="详细信息">
        <n-flex vertical justify="space-between" style="margin-bottom: 20px;">
          <n-text  class="text-base text-green-700" >标题: {{ docsStore.currentSelectDoc?.title }}</n-text>
          <n-text  class="text-base text-green-700" >作者: {{ docsStore.currentSelectDoc?.author }}</n-text>
          <n-text  class="text-base text-green-700" >刊物: {{ docsStore.currentSelectDoc?.journal }}</n-text>
          <n-text  class="text-base text-green-700" >年份: {{ docsStore.currentSelectDoc?.year}}</n-text>
        </n-flex>
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
      <n-card title="核心思想">
        <div >
          {{docsStore.currentSelectDoc?.remark}}
        </div>
      </n-card>
      <n-card title="要点总结">
        <n-flex vertical justify="space-between" style="margin-bottom: 10px;">
          <n-text  class="font-medium text-base text-green-700" >摘要: {{ docsStore.currentSelectDoc?.abstract}}</n-text>
          <n-text  class="text-base text-green-700" >创新点: {{ docsStore.currentSelectDoc?.contributions }}</n-text>
        </n-flex>
      </n-card>
    </n-scrollbar>
  </div>
</template>
<style scoped>

</style>