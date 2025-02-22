<script setup lang="ts">
import {ref} from "vue";

const show = ref(false)
</script>

<template>
  <div>
    <n-grid :x-gap="0" :y-gap="5" :cols="1">
      <n-grid-item>
        <div class="light-green" />
      </n-grid-item>
      <n-grid-item>
        <n-card title="卡片">
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