<script setup lang="ts">
import {nextTick, onMounted, ref, watch} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Tag, TagAndGroups, TagGroup} from "./main-type.ts";
import {message} from "../../message.ts";
import { VueDraggable } from 'vue-draggable-plus'
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


const tagBgColorPool = [
  '#FFFFFF', '#F8F9FA', '#E9ECEF', '#DEE2E6', '#CED4DA',
  '#18A058', '#2080F0', '#F0A020', '#ba6d93', '#FF6B6B',
  '#4ECDC4', '#45B7D1', '#96CEB4', '#D1D8E0', '#FFEEAD',
  '#FFB7B2', '#A5C9CA', '#C9ADA7', '#E0D5B8'
]

const tagTextColorPool = [
  '#000000', '#FFFFFF', '#18A058', '#2080F0', '#F0A020',
  '#6C757D', '#495057', '#343A40', '#212529',
  '#084956', '#1967cf', '#805855', '#8d0b0b'
]
function getRandomItem<T>(arr: T[]): T {
  return arr[Math.floor(Math.random() * arr.length)]
}
function generateRandomTagColor() {
  const bg = getRandomItem(tagBgColorPool)
  // 尝试从文字色池里挑一个“够对比”的
  let text = getRandomItem(tagTextColorPool)
  if (getContrast(bg, text) < 300) {
    // 对比度不够，退回黑白方案
    text = isLightColor(bg) ? '#000000' : '#FFFFFF'
  }
  newTagBgColor.value = bg
  newTagTextColor.value = text
}
function getContrast(bg: string, text: string) {
  const toRGB = (hex: string) => {
    const c = hex.replace('#', '')
    return {
      r: parseInt(c.slice(0, 2), 16),
      g: parseInt(c.slice(2, 4), 16),
      b: parseInt(c.slice(4, 6), 16),
    }
  }
  const a = toRGB(bg)
  const b = toRGB(text)
  return Math.abs(a.r - b.r) + Math.abs(a.g - b.g) + Math.abs(a.b - b.b)
}
function isLightColor(hex: string) {
  const color = hex.replace('#', '')
  const r = parseInt(color.substring(0, 2), 16)
  const g = parseInt(color.substring(2, 4), 16)
  const b = parseInt(color.substring(4, 6), 16)
  const brightness = (r * 299 + g * 587 + b * 114) / 1000
  return brightness > 160
}


onMounted(async ()=>{
  invoke<TagAndGroups[]>('query_tag_groups',{}).then(data =>{
    store.tagGroups = data;
  }).catch(e => {message.error(e)})
})

function toShowNewTagModal(){
  generateRandomTagColor()
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
  if (!tagGroupName){
    message.error(`标签组名不能为空`)
    return
  }
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
function handleDragEnd(){
  // 遍历 store.tagGroups 并更新每个 tags.tag_group.index
  store.tagGroups.forEach((item, index) => {
    item.tag_group.index = index
  })
  const tagGroups = store.tagGroups.map(item => item.tag_group)
  invoke('reindex_tag_group', {tagGroups: tagGroups}).then(_ => {}).catch(e => {message.error(e)})
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
      <n-button
          size="small"
          quaternary
          @click="generateRandomTagColor"
      >
        🎲 换个颜色
      </n-button>
      <n-input v-model:value="newTagValue" placeholder="请输入标签名" ref="inputRef" @keydown.enter.prevent="createNewTag"  />
      <n-color-picker
          v-model:value="newTagBgColor"
          :swatches="[
  '#FFFFFF',    // 纯白色
  '#F8F9FA',    // 灰白色
  '#E9ECEF',    // 浅灰色
  '#DEE2E6',    // 灰色
  '#CED4DA',    // 深灰色
  '#ADB5BD',    // 暗灰色
  '#18A058',    // 绿色
  '#2080F0',    // 蓝色
  '#F0A020',    // 橙色
  '#ba6d93',    // 粉紫色
  '#FF6B6B',    // 粉红色
  '#4ECDC4',    // 浅绿色
  '#45B7D1',    // 浅蓝色
  '#96CEB4',    // 淡绿色
  '#D1D8E0',    // 浅灰蓝色
  '#FFEEAD',    // 浅黄色
  '#FFB7B2',    // 浅红色
  '#A5C9CA',    // 浅蓝绿色
  '#C9ADA7',    // 浅红棕色
  '#E0D5B8'     // 米色
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
  '#000000',    // 黑色
  '#FFFFFF',    // 白色
  '#18A058',    // 绿色
  '#2080F0',    // 蓝色
  '#F0A020',    // 橙色
  'rgba(208, 48, 80)', // 红色
  '#6C757D',    // 中性灰
  '#495057',    // 深中性灰
  '#343A40',    // 更深中性灰
  '#212529',    // 近黑色
  '#FF6B6B',    // 粉红色
  '#4ECDC4',    // 浅绿色
  '#084956',    // 浅蓝色
  '#17d87a',    // 淡绿色
  '#1967cf',    // 浅灰蓝色
  '#bd9f18',    // 浅黄色
  '#805855',    // 浅红色
  '#8d0b0b',    // 中性灰
  '#A5C9CA',    // 浅蓝绿色
  '#C9ADA7',    // 浅红棕色
  '#E0D5B8'     // 米色
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
          <n-space justify="center" >
            <n-button
                type="primary"
                strong
                secondary
                size="medium"
                @click="showNewGroupModal"
                class="mt-2 text-black"
            >
              新建标签组
            </n-button>
          </n-space>
        </n-grid-item>
        <n-grid-item>
          <VueDraggable ref="el" v-model="store.tagGroups" @end="handleDragEnd">
            <n-card
                header-style="padding:5px 5px 5px 25px;font-size:17px;font-weight:bold;"
                content-style="padding:5px 5px 5px 15px;"
                v-for="tags in store.tagGroups" :key="tags.tag_group.id" :title="tags.tag_group.name"
                class="cursor-pointer group bg-gray-100 rounded-lg hover:bg-gray-300 shadow-md m-2"
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
          </VueDraggable>
        </n-grid-item>
      </n-grid>
    </n-scrollbar>
  </div>
</template>

<style scoped>

</style>