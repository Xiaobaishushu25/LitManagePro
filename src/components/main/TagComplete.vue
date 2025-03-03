<script setup lang="ts">

import AutoCompleteTag from "../../util/AutoCompleteTag.vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {message} from "../../message.ts";
import {DropdownOption, NFlex, NTag} from "naive-ui";
import {Tag} from "./main-type.ts";
import {Component, computed, h, resolveComponent} from "vue";
import useConfigStore from "../../stroe/config.ts";

// 修改后（正确）
type TagRowOption = DropdownOption & {
  rowData: Tag[]
}

const tagStore = useTagGroupsStore()
const configStore = useConfigStore()

const fetchSuggestions = async (query: string) => {
  if (query === " ") { // 如果输入一个空格，则返回所有标签
    return tagStore.allTags;
  }
  // 将查询字符串和标签值都转换为小写，以便进行大小写不敏感的匹配
  const queryLower = query.toLowerCase();
  return tagStore.allTags.filter(option =>
      option.value.toLowerCase().includes(queryLower)
  );
};

// const options: Tag[][] = [
//   [
//     {
//       index: 0,
//       group_id: 1,
//       id: 1,
//       value: '标签1',
//       bg_color: 'bg-blue-100',
//       text_color: 'text-blue-800'
//     },
//     {
//       index: 1,
//       group_id: 1,
//       id: 2,
//       value: '标签2',
//       bg_color: 'bg-green-100',
//       text_color: 'text-green-800'
//     }
//   ],
//   [
//     {
//       index: 2,
//       group_id: 2,
//       id: 3,
//       value: '标签3',
//       bg_color: 'bg-yellow-100',
//       text_color: 'text-yellow-800'
//     },
//     {
//       index: 3,
//       group_id: 2,
//       id: 4,
//       value: '标签4',
//       bg_color: 'bg-purple-100',
//       text_color: 'text-purple-800'
//     }
//   ]
// ]
// 处理下拉选项数据
// const processedOptions = options.map((tagArray, index) => ({
// const processedOptions = configStore.save_tags.map((tagArray, index) => ({
//   key: `row-${index}`,
//   rowData: tagArray
// })) as TagRowOption[]

// 定义 processedOptions
// const processedOptions = computed<TagRowOption[]>(() => {
//   return configStore.save_tags?.map((tagArray, index) => ({
//     key: `row-${index}`,
//     rowData: tagArray
//   }))
// })
// 定义 processedOptions
const processedOptions = computed<TagRowOption[]>(() => {
  // 检查 configStore.save_tags 是否为 undefined
  if (!configStore.save_tags) {
    return [] // 如果为 undefined，返回空数组
  }
  // 如果存在值，执行映射
  return configStore.save_tags.map((tagArray, index) => ({
    key: `row-${index}`,
    rowData: tagArray
  }))
})

// 自定义渲染函数
const renderDropdownLabel: (option: TagRowOption) => Component = (option) => {
  return h(
      'div',
      {
        class: 'flex items-center justify-between p-2 hover:bg-gray-50',
      },
      [
        h(
            'div',
            { class: 'flex flex-wrap gap-2 flex-1' },
            option.rowData.map(tag =>
                h(NTag, {
                  class: `${tag.bg_color} ${tag.text_color} rounded-md pb-2.5`,
                  color: { // 使用对象形式
                    color: tag.bg_color,
                    textColor: tag.text_color
                  },
                  size: 'medium' as const,
                }, {
                  default: () => tag.value
                })
            )
        ),
        // 关闭按钮（始终靠右）
        h(
            'div',
            {
              class: 'flex-shrink-0 ml-4 cursor-pointer text-gray-400 hover:text-red-500',
              onClick: (e: MouseEvent) => {
                e.stopPropagation()
                handleDeleteRow(option)
              }
            },
            [
              h(resolveComponent('inline-svg'), {
                src: '../assets/svg/Delete24Regular.svg',
                class: 'w-5 h-5 transition-colors'
              })
            ]
        )
      ]
  )
}
function handleDeleteRow(option: TagRowOption){
  //删除时直接取出所有id加一起，然后看哪个哪组加起来一样大就删了。
  message.info(`[删除] ${option.rowData[0].value}`)
  // 获取当前行的所有 Tag
  const tags = option.rowData
  // 提取所有 Tag 的 id 并计算总和
  const idSum = tags.reduce((sum, tag) => sum + tag.id, 0)
  // 显示删除信息
  message.info(`[删除] ${tags.map(tag => tag.value).join(', ')}，ID总和为 ${idSum}`)
  configStore.removeSaveTags(idSum)
}
function saveConfigs(){
  // 提取所有 tag 的 id
  const tagIds = tagStore.andTags.map(tag => tag.id)
  configStore.addSaveTags(tagIds)
  message.success("标签组成功保存")
}
const handleSelect = (_key: string | number, option: TagRowOption) => {
  message.info(`[选择] ${option.rowData}`)
}
</script>

<template>
  <div>
    <div class="flex flex-col h-full">
      <n-flex class="w-full flex items-center">
        <auto-complete-tag
            v-model:modelValue="tagStore.andTags"
            placeholder="按空格提示所有标签"
            :options="tagStore.allTags"
            :fetchSuggestions="fetchSuggestions"
            class="flex-grow mr-2"
        />
        <n-tooltip trigger="hover" >
          <template #trigger>
            <inline-svg
                src="../assets/svg/Save24Regular.svg"
                class="svg-button w-6 h-6 mr-3"
                @click.stop="saveConfigs"
            ></inline-svg>
          </template>
            保存标签组
        </n-tooltip>
        <n-dropdown
            trigger="click"
            :options="processedOptions"
            :render-label="renderDropdownLabel"
            :show-arrow="true"
            @select="handleSelect"
            class="custom-dropdown"
        >
          <inline-svg
              src="../assets/svg/DropDown24.svg"
              class="svg-button w-6 h-6 mr-3"
          ></inline-svg>
        </n-dropdown>
      </n-flex>
      <auto-complete-tag
          v-model:modelValue="tagStore.orTags"
          placeholder="按空格提示所有标签"
          :options="tagStore.allTags"
          :fetchSuggestions="fetchSuggestions"
      />
    </div>
  </div>
</template>
<style>
/*.v-binder-follower-content{
  width: 1000px;
}这个会把tooltip也变宽*/
</style>
<style scoped>
/* 仅针对 dropdown 的弹出层  没效果*/
:deep(.custom-dropdown .v-binder-follower-content) {
  min-width: 600px !important; /* 保持与触发器同宽 */
  max-width: 600px; /* 可选的最大宽度限制 */
}
</style>