<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Tags} from "./MainType.ts";
import {message} from "../../message.ts";

const show = ref(false)

const showColorPicker = ref(false)
const selectedColor = ref('#000000')

const open = ref<Record<number, boolean>>({});

const tagsData = ref<Tags[] | null>(null);
onMounted(async ()=>{
  invoke<Tags[]>('query_tags',{}).then(data =>{
    console.log(data)
    tagsData.value = data;
  }).catch(e => {message.error(`查询标签数据出错${e}`)})
})

</script>

<template>
  <div>
    <n-grid :x-gap="0" :y-gap="5" :cols="1">
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
              <n-button>+</n-button>
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