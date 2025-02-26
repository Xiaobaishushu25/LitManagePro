<script setup lang="ts">
import {nextTick, onMounted, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Tag, TagAndGroups, TagGroup} from "./main-type.ts";
import {message} from "../../message.ts";
import CustomModal from "../../util/CustomModal.vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import useConfigStore from "../../stroe/config.ts";

const store = useTagGroupsStore()
const configStore = useConfigStore()

const inputRef = ref<HTMLInputElement | null>(null)
const groupInputRef = ref<HTMLInputElement | null>(null)
const newNameInputRef = ref<HTMLInputElement | null>(null)

const showNewTagModal = ref(false)
const showGroupModal = ref(false)
const showDeleteGroupModal = ref(false)
const showRenameGroupModal = ref(false)

const currentTagGroup = ref<TagGroup|null>(null)
const newGroupName = ref('')
const newReName = ref('')
const newTagBgColor = ref('#c89f9f')
const newTagTextColor = ref('#000000')
const newTagValue = ref('')


onMounted(async ()=>{
  invoke<TagAndGroups[]>('query_tag_groups',{}).then(data =>{
    store.tagGroups = data;
  }).catch(e => {message.error(e)})
})

function toShowNewTagModal(){
  showNewTagModal.value = true;
  nextTick(() => {
    inputRef.value!.focus()
  })
}
function showNewGroupModal(){
  showGroupModal.value = true;
  nextTick(() => {//去掉后不会有焦点
    groupInputRef.value!.focus()
  })
}
watch(
    () => showRenameGroupModal.value,
    (newValue, _oldValue) => {
      if (newValue) {
        newReName.value = currentTagGroup.value!.name;
        nextTick(() => {
          newNameInputRef.value!.focus()
          newNameInputRef.value!.select()
        });
      }
    }
);
// 计算当前标签是否禁用
const isTagDisabled = (tagId: number) => {
  return store.currentSelectTags.some(selectedTag => selectedTag.id === tagId)
}
const canTagDelete = ref(false)
function clickTag(event:MouseEvent, id: number, _value: string){
  if (isTagDisabled(id)) {
    event.preventDefault()
    event.stopPropagation()
    return
  }
  if(canTagDelete.value){
    return;
  }
  // 0 表示鼠标左键，2 表示鼠标右键
  if (event.button === 0) {
    store.addTagToAndTags(id)
  } else if (event.button === 2) {
    store.addTagToOrTags(id)
  }
}
function hoverGroup(index: number,id: number, name: string){
  currentTagGroup.value={
    index: index,
    id: id,
    name: name
  }
}
function createNewTag(){
  let value = newTagValue.value;
  if(value.length === 0){
    message.error(`标签名不能为空`)
    return;
  }
  if (!value){
    message.error(`创建标签失败，请检查当前标签组信息。`)
  }
  let tag:Tag = {
    index: 0,
    group_id: currentTagGroup.value!.id,
    id: 0,
    value: value,
    bg_color: newTagBgColor.value,
    text_color: newTagTextColor.value
  }
  invoke<Tag>('create_tag', {
    tag: tag
  }).then(data => {//回来的data是一个Tag结构体
    store.addNewTag(data)
    newTagValue.value = '';
    message.success(`创建标签成功`)
  }).catch(e => {
    message.error(e)
  });
  showNewTagModal.value = false;
}
function deleteTag(group_id:number, id: number,name: string){
  invoke('delete_tag', {
    id: id
  }).then(_ => {//回来的data是一个Tag结构体
    store.deleteTag(group_id,id)
    message.success(`删除标签${name}成功`)
  }).catch(e => {
    message.error(e)
  });
}
function createNewTagGroup(){
  showGroupModal.value = false;
  let tagGroupName = newGroupName.value;
  invoke<TagGroup>('create_tag_group', {
    groupName: tagGroupName,
  }).then(data => {
    store.addNewTagGroup(data)
    message.success(`创建标签组${tagGroupName}成功`)
  }).catch(e => {
    message.error(e)
  });
  newGroupName.value = '';
}
function renameGroup(){
  showRenameGroupModal.value = false;
  let newName = newReName.value;
  if (!newName){
    message.error(`标签组名不能为空`)
    return
  }
  let id = currentTagGroup.value!.id;
  invoke('rename_tag_group', {
    id: id,
    name: newName
  }).then(_ => {
    store.renameTagGroup(id,newName)
    message.success(`修改标签组名称成功`)
  }).catch(e => {
    message.error(e)
  });
}
function deleteGroup(){
  showDeleteGroupModal.value = false;
  let tagGroupName = currentTagGroup.value!.name;
  let id = currentTagGroup.value!.id;
  invoke('delete_group', {
    id: id
  }).then(_ => {
    store.deleteTagGroup(id)
    message.success(`删除标签组${tagGroupName}成功`)
  }).catch(e => {
    message.error(e)
  });
}
</script>

<template>
  <custom-modal
      v-model:show="showRenameGroupModal"
      title="重命名标签组"
      :onConfirm="renameGroup"
  >
    <n-input placeholder="请输入标签组名" v-model:value="newReName" ref="newNameInputRef" @keydown.enter.prevent="renameGroup"></n-input>
  </custom-modal>
  <custom-modal
      v-model:show="showDeleteGroupModal"
      title="删除标签组"
      :onConfirm="deleteGroup"
  >
    <div style="flex-wrap: wrap;">
      <span>确定要删除</span>
      <span class="text-orange-400 text-base">{{currentTagGroup!.name}}</span>
      <span>标签组？（组内标签将全部删除）</span>
    </div>
  </custom-modal>
  <custom-modal
      v-model:show="showGroupModal"
      title="新建标签组"
      :onConfirm="createNewTagGroup"
  >
    <n-flex vertical>
          <n-input placeholder="请输入标签组名" v-model:value="newGroupName" ref="groupInputRef" @keydown.enter.prevent="createNewTagGroup" />
    </n-flex>
  </custom-modal>
  <custom-modal
      v-model:show="showNewTagModal"
      title="新建标签"
      :onConfirm="createNewTag"
  >
    <template #header>
      <n-flex :size="0" class="text-base">
        <div>为</div>
        <div class="text-orange-400">{{currentTagGroup!.name}}</div>
        <div>组添加标签</div>
      </n-flex>
    </template>
    <n-flex vertical>
      <n-tag class="w-fit" :color="{color: newTagBgColor, textColor: newTagTextColor}">{{ newTagValue }}</n-tag>
      <n-input v-model:value="newTagValue" placeholder="请输入标签名" ref="inputRef" @keydown.enter.prevent="createNewTag"  />
      <n-color-picker
          v-model:value="newTagBgColor"
          :swatches="[
      '#FFFFFF',
      '#18A058',
      '#2080F0',
      '#F0A020',
      '#ba6d93',
    ]"
      >
        <template #label>
          背景颜色
        </template>
      </n-color-picker>
      <n-color-picker
          v-model:value="newTagTextColor"
          :show-alpha="false"
          :swatches="[
      '#000000',
      '#FFFFFF',
      '#18A058',
      '#2080F0',
      '#F0A020',
      'rgba(208, 48, 80)',
    ]"
      >
        <template #label>
          文字颜色
        </template>
      </n-color-picker>
    </n-flex>
  </custom-modal>

  <div class="h-full">
    <n-scrollbar trigger="none">
      <n-grid :x-gap="0" :y-gap="5" :cols="1">
        <n-grid-item>
          <n-space>
            <n-input placeholder="请输入标签名" />
            <n-button @click="showNewGroupModal">+</n-button>
          </n-space>
        </n-grid-item>
        <n-grid-item>
          <div class="light-green" />
        </n-grid-item>
        <n-grid-item>
          <n-card
              header-style="padding:5px 5px 5px 25px;font-size:17px;font-weight:bold;"
              content-style="padding:5px 5px 5px 15px;"
              v-for="tags in store.tagGroups" :key="tags.tag_group.id" :title="tags.tag_group.name"
              class="cursor-pointer group"
              @mouseenter="hoverGroup(tags.tag_group.index, tags.tag_group.id, tags.tag_group.name)"
              @click="configStore.getTagGroupState(tags.tag_group.id).value=!configStore.getTagGroupState(tags.tag_group.id).value"
          >
            <template #header-extra>
              <div class="hidden group-hover:block">
                <n-flex class="flex items-center pr-4" :size="2">
                  <n-tooltip trigger="hover" class="text-xs p-0">
                    <template #trigger>
                      <inline-svg
                          src="../assets/svg/Rename24Regular.svg"
                          class="svg-button"
                          @click.stop="showRenameGroupModal=true"
                      ></inline-svg>
                    </template>
                    重命名
                  </n-tooltip>
                  <n-tooltip trigger="hover" class="text-xs p-0">
                    <template #trigger>
                      <inline-svg
                          src="../assets/svg/Delete24Regular.svg"
                          class="svg-button hover:text-red-600 -rotate-90"
                          @click.stop="showDeleteGroupModal=true"
                      ></inline-svg>
                    </template>
                    删除标签组
                  </n-tooltip>
                  <n-tooltip trigger="hover" class="text-xs p-0">
                    <template #trigger>
                      <inline-svg
                          src="../assets/svg/Delete24Regular.svg"
                          class="svg-button hover:text-red-600 rotate-180"
                          @click.stop="canTagDelete = !canTagDelete"
                      ></inline-svg>
                    </template>
                    删除标签
                  </n-tooltip>
                  <n-tooltip trigger="hover" class="text-xs p-0">
                    <template #trigger>
                      <inline-svg
                          src="../assets/svg/Add24Filled.svg"
                          class="svg-button"
                          @click.stop="toShowNewTagModal"
                      ></inline-svg>
                    </template>
                    添加标签
                  </n-tooltip>
<!--                  <n-switch v-model:value="configStore.getTagGroupState(tags.tag_group.id).value"></n-switch>-->
                </n-flex>
              </div>
            </template>
            <n-collapse-transition :show="configStore.getTagGroupState(tags.tag_group.id).value">
              <div class="flex flex-wrap gap-1 flex-row items-center">
                <n-tag
                    v-for="tag in tags.tags" :key="tag.id"
                    :color="{ color: tag.bg_color, textColor: tag.text_color }"
                    :disabled="isTagDisabled(tag.id)"
                    :closable="canTagDelete"
                    @click.stop=""
                    @close.stop="deleteTag(tag.group_id,tag.id,tag.value)"
                    @mousedown.stop="clickTag($event,tag.id, tag.value)"
                    class="cursor-pointer"
                >
                  {{ tag.value }}
                </n-tag>
              </div>
            </n-collapse-transition>
          </n-card>
        </n-grid-item>
      </n-grid>
    </n-scrollbar>
  </div>
</template>

<style scoped>

</style>