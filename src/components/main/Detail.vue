<script setup lang="ts">
import {NTag} from "naive-ui";
import useDocStore from "../../stroe/doc.ts";
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {message} from "../../message.ts";
import CustomModal from "../../util/CustomModal.vue";

const docsStore = useDocStore()

const showTagModal = ref(false)

const canTagDelete = ref(false)

function deleteTag(tagId: number){
  console.log("delete tag")
  invoke('delete_doc_tag', {docId: docsStore.currentSelectDoc?.id, tagId: tagId})
      .then(_ => {
      })
      .catch(e => {message.error(e)})
}
function updateTag(tagId: number){
  showTagModal.value = false
  // invoke('update_doc_tag', {docId: docsStore.currentSelectDoc?.id, tagId: tagId})
  //     .then(_ => {
  //     })
  //     .catch(e => {message.error(e)})
  console.log("update tag")
}

</script>

<template>
  <custom-modal title="修改标签"  v-model:show="showTagModal" :on-confirm="updateTag" class="w-96">
    <n-transfer
        v-model:value="value"
        :options="options"
        :render-source-list="renderSourceList"
        source-filterable
    />
  </custom-modal>
  <div>
    <n-card title="文档详细信息">
      <n-flex align="center" justify="space-between" style="margin-bottom: 20px;">
        <div class="title-container">
          <n-text  class="text-base text-green-700" >标题: {{ docsStore.currentSelectDoc?.title }}</n-text>
        </div>
      </n-flex>
    </n-card>
    <n-card title="文档标签信息">
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
            class="mx-1"
        >
          {{ tag.value }}
        </n-tag>
      </div>
    </n-card>
    <n-card title="文档核心思想">
      <div style="margin-top: 10px;">
        {{docsStore.currentSelectDoc?.remark}}
      </div>
    </n-card>
  </div>
</template>

<style scoped>

</style>