<script setup lang="ts">
import {nextTick, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {TagGroup, Tags} from "./MainType.ts";
import {message} from "../../message.ts";
import CustomModal from "../../util/CustomModal.vue";

const inputRef = ref<HTMLInputElement | null>(null)
const groupInputRef = ref<HTMLInputElement | null>(null)

const show = ref(false)
const showModal = ref(false)
const showGroupModal = ref(false)

const showColorPicker = ref(false)
const selectedColor = ref('#000000')

const currentTagGroup = ref<TagGroup|null>(null)
const newTagColor = ref('#c89f9f')
const newTagTextColor = ref('#000000')
const newTagValue = ref('')

const open = ref<Record<number, boolean>>({});

const tagsData = ref<Tags[] | null>(null);
onMounted(async ()=>{
  invoke<Tags[]>('query_tags',{}).then(data =>{
    console.log(data)
    tagsData.value = data;
  }).catch(e => {message.error(`查询标签数据出错${e}`)})
})

function showNewTagModal(id: number, name: string){
  console.log(`Tag Group ID: ${id}, Name: ${name}`);
  currentTagGroup.value={
    id: id,
    name: name
  }
  // currentTagGroup.value = name;
  showModal.value = true;
  nextTick(() => {
    inputRef.value!.focus()
  })
}
function showNewGroupModal(){
  showGroupModal.value = true;
  nextTick(() => {
    groupInputRef.value!.focus()
  })
}
function createNewTag(){
  showModal.value = false;
  let value = currentTagGroup.value;
  if (!value){
    message.error(`创建标签失败，请检查当前标签组信息。`)
  }
  invoke('create_tag', {
    tag_group_id: value!.id,
    tag_value: newTagValue.value,
    tag_color: newTagColor.value
  }).then(data => {
    console.log(data)
    message.success(`创建标签成功`)
  }).catch(e => {
    message.error(`创建标签失败${e}`)
  })
}
function createNewTagGroup(){
  showGroupModal.value = false;
  invoke('create_tag_group', {
    tag_group_name: "sd",
  }).then(data => {
    console.log(data)
    message.success(`创建标签组成功`)
  }).catch(e => {
    message.error(`创建标签组失败${e}`)
  })
}
</script>

<template>
  <custom-modal
      v-model:show="showGroupModal"
      title="新建标签2组"
      :onConfirm="createNewTagGroup"
  >
<!--    <template #header>-->
<!--      <div>自定内容</div>-->
<!--      <div class="text-red-800">自定义标题内容</div>-->
<!--    </template>-->
    <n-flex vertical>
          <n-input  placeholder="请输入标签组名" ref="groupInputRef"  />
    </n-flex>
  </custom-modal>
<!--  <n-modal v-model:show="showGroupModal"-->
<!--           preset="card"-->
<!--           class="w-80"-->
<!--           :mask-closable="false"-->
<!--           :draggable="true"-->
<!--  >-->
<!--    <template #header>-->
<!--      <div>新建标签组</div>-->
<!--    </template>-->
<!--    <n-flex vertical>-->
<!--      <n-input  placeholder="请输入标签组名" ref="groupInputRef"  />-->
<!--    </n-flex>-->
<!--    <template #action>-->
<!--      <n-space>-->
<!--        <n-button @click="showModal = false">取消</n-button>-->
<!--        <n-button @click="createNewTag">确认</n-button>-->
<!--      </n-space>-->
<!--    </template>-->
<!--  </n-modal>-->
  <n-modal v-model:show="showModal"
           preset="card"
           class="w-80"
           :mask-closable="false"
           :draggable="true"
           >
    <template #header>
      <n-flex :size="0" class="text-base">
        <div>为</div>
        <div class="text-orange-400">{{currentTagGroup!.name}}</div>
        <div>组添加标签</div>
      </n-flex>
    </template>
    <n-flex vertical>
      <n-tag class="w-fit" :color="{color: newTagColor, textColor: newTagTextColor}">{{ newTagValue }}</n-tag>
      <n-input v-model:value="newTagValue" placeholder="请输入标签名" ref="inputRef"  />
      <n-color-picker
          v-model:value="newTagColor"
          :swatches="[
      '#FFFFFF',
      '#18A058',
      '#2080F0',
      '#F0A020',
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
    <template #action>
      <n-space>
        <n-button @click="showModal = false">取消</n-button>
        <n-button @click="createNewTag">确认</n-button>
      </n-space>
    </template>
  </n-modal>
  <div>
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
        <n-card v-for="tags in tagsData" :key="tags.tag_group.id" :title="tags.tag_group.name">
          <template #header-extra>
<!--            <n-switch v-model:value="show[tags.tagGroup.id]">-->
            <n-switch v-model:value="show">
              <template #checked>
                展开
              </template>
              <template #unchecked>
                折叠
              </template>
            </n-switch>
          </template>
          <n-collapse-transition :show="show">
            <div class="flex flex-wrap gap-1">
              <n-tag v-for="tag in tags.tags" :key="tag.id" :color="{ color: `${tag.color}30`, textColor: tag.color }">
                {{ tag.value }}
              </n-tag>
              <n-button @click="showNewTagModal(tags.tag_group.id, tags.tag_group.name)">+</n-button>
            </div>
          </n-collapse-transition>
        </n-card>
      </n-grid-item>
      <n-grid-item>
        <n-card
            header-style="padding:5px 5px 5px 15px;font-size:17px;font-weight:bold;"
            content-style="padding:5px 5px 5px 15px;font-weight:bold;"
            title="深度学习"
        >
          <template #header-extra>
            <n-switch v-model:value="show">
              <template #checked>
                展开
              </template>
              <template #unchecked>
                折叠
              </template>
            </n-switch>
          </template>
          <n-collapse-transition :show="show">
            <div class="flex flex-wrap gap-1">
              <n-tag
                  class="text-xs"
                  type="success"
                  :bordered="false"
                  @click="showColorPicker = true"
                  :style="{ color: selectedColor }"
              >
                扩散模型
              </n-tag>
              <n-tag
                  class="text-xl"
                  type="warning"
                  :color="{color: '#00E5EE50',textColor: '#00E5EE'}"
                  @click="showColorPicker = true"
                  :style="{ color: selectedColor }"
              >
                深度神经网络！！
              </n-tag>
              <n-tag
                  type="error"
                  @click="showColorPicker = true"
                  :style="{ color: selectedColor }"
              >
                LSTM
              </n-tag>
              <n-tag
                  type="info"
                  @click="showColorPicker = true"
                  :style="{ color: selectedColor }"
              >
                GAN
              </n-tag>

              <n-color-picker
                  v-if="showColorPicker"
                  :swatches="[
        '#FFFFFF',
        '#18A058',
        '#2080F0',
        '#F0A020',
        'rgba(208, 48, 80, 1)',
      ]"
                  @close="showColorPicker = false"
              />
            </div>
          </n-collapse-transition>
        </n-card>
      </n-grid-item>
      <n-grid-item>
        <n-card title="卡片2">
          <template #header-extra>
            <n-switch v-model:value="show">
              <template #checked>
                展开
              </template>
              <template #unchecked>
                折叠
              </template>
            </n-switch>
          </template>
          <n-collapse-transition :show="show">
            <div class="flex flex-wrap gap-1">
              <n-tag type="success" >
                成功
              </n-tag>
              <n-tag type="warning" >
                我真的警告！！
              </n-tag>
              <n-tag type="error" >
                错误
              </n-tag>
              <n-tag type="info" >
                信息
              </n-tag>
            </div>
          </n-collapse-transition>
        </n-card>
      </n-grid-item>
    </n-grid>
  </div>
</template>

<style scoped>

</style>