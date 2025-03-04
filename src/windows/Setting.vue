<script setup lang="ts">

import Appearance from "../components/setting/Appearance.vue";
import {h, ref} from "vue";
import InlineSvg from "vue-inline-svg";
const renderLabel = (option:{ label: string, value: string, icon: string}) => {
  return h('div', { style: { display: 'flex', alignItems: 'center' } }, [
    h(InlineSvg, { src: option.icon, class: 'w-4 h-4' }),
    h('span', { style: { marginLeft: '8px' } }, option.label)
  ]);
};
const value = ref('microsoft'); // 当前选中的值
const options = ref([
  { label: '百度翻译', value: 'baidu', icon: '../assets/icon/chatgpt.svg' },
  { label: '微软翻译', value: 'microsoft', icon: '../assets/icon/deepseek.svg' },
  { label: '谷歌翻译', value: 'google', icon: '../assets/icon/kimi.svg' },
  { label: '有道翻译', value: 'youdao', icon: '../assets/icon/tongyi.svg' },
  { label: '阿里翻译', value: 'ali', icon: '../assets/icon/wenxin.svg' },
  { label: 'DeepL 翻译', value: 'deepl', icon: '../assets/icon/chatgpt.svg' }
]);
// 自定义渲染标签

const renderOption = (option:{ label: string, value: string, icon: string}) => {
  return h('div', { style: { display: 'flex', alignItems: 'center' } }, [
    h(InlineSvg, { src: option.icon, class: 'w-4 h-4' }),
    h('span', { style: { marginLeft: '8px' } }, option.label)
  ]);
};
</script>

<template>
  <div>
    <n-tabs
        type="card"
        placement="left"
    >
      <n-tab-pane name="外观" tab="外观"><appearance></appearance></n-tab-pane>
      <n-tab-pane name="功能" tab="功能">
        <div>
          <n-select
              v-model:value="value"
              :options="options"
              :render-label="renderLabel"
              clearable
              filterable
              style="width: 200px"
          />
        </div>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>

</style>