<script setup lang="ts">
import AutoCompleteTag from "../../util/AutoCompleteTag.vue";
import useTagGroupsStore from "../../stroe/tag.ts";
import {message} from "../../message.ts";
import {DropdownOption, NFlex, NTag} from "naive-ui";
import {Tag} from "./main-type.ts";
import {ref, Component, computed, h, onMounted, resolveComponent,onUnmounted} from "vue";
import useConfigStore from "../../stroe/config.ts";
import {listen} from "@tauri-apps/api/event";

// 定义 TagRowOption,每行的格式
type TagRowOption = DropdownOption & {
  rowData: Tag[]
}

const tagStore = useTagGroupsStore()
const configStore = useConfigStore()

let unlisten1: () => void;
let unlisten2: () => void;
let unlisten3: () => void;
let unlisten4: () => void;

const tagRef1 = ref()
const tagRef2 = ref()
const isDropdownVisible = ref(false);

onMounted(async ()=>{
  unlisten1 = await listen('聚焦上标签栏', () => {
    console.log('聚焦上标签栏')
    tagRef1?.value.focus()
  })
  unlisten2 = await listen('聚焦下标签栏', () => {
    tagRef2?.value.focus()
  })
  unlisten3 = await listen('保存标签组', () => {
    saveConfigs()
  })
  unlisten4 = await listen('打开常用标签组', () => {
    isDropdownVisible.value = !isDropdownVisible.value
  })
})
onUnmounted(()=>{
  unlisten1()
  unlisten2()
  unlisten3()
  unlisten4()
})

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

// 定义 processedOptions
const processedOptions = computed<TagRowOption[]>(() => {
  // 检查 configStore.save_tags 是否为 undefined
  if (!configStore.save_tags|| configStore.save_tags.length === 0) {
    // 如果为 undefined 或空，返回一个虚拟数据
    //注意，由于有这个虚拟数据的可能，所有操作都要检查是否是虚拟数据。
    return [{
      key: 'empty-row',
      rowData: [{
        index: 0,       // 新增
        group_id: 0,    // 新增
        id: 0,
        value: '没有标签组',
        bg_color: 'bg-gray-200',
        text_color: 'text-gray-800'
      }]
    }];
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
        class: 'flex items-center justify-between h-full  min-w-[200px]',
      },
      [
        h(
            'div',
            { class: 'flex flex-wrap flex-1' },
            option.rowData.map(tag =>
                h(NTag, {
                  class: `cursor-pointer rounded-md ml-2`,
                  color: { // 使用对象形式
                    color: tag.bg_color,
                    textColor: tag.text_color
                  },
                  bordered: false,
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
  // 获取当前行的所有 Tag
  const tags = option.rowData
  // 提取所有 Tag 的 id 并计算总和
  const idSum = tags.reduce((sum, tag) => sum + tag.id, 0)
  if (idSum === 0) return
  // 显示删除信息
  // message.info(`[删除] ${tags.map(tag => tag.value).join(', ')}，ID总和为 ${idSum}`)
  message.info("删除标签组成功")
  configStore.removeSaveTags(idSum)
}
function saveConfigs(){
  // 提取所有 tag 的 id
  const tagIds = tagStore.andTags.map(tag => tag.id)
  if (tagIds.length === 0){
    message.error("请至少添加一个标签")
    return
  }
  configStore.addSaveTags(tagIds)
  message.success("标签组成功保存")
}
const handleSelect = (_key: string | number, option: TagRowOption) => {
  //拿出所有的id，组成一个数组
  const tagIds = option.rowData.map(tag => tag.id)
  if (tagIds.length === 0){
    return
  }
  tagStore.setAllAndTags(tagIds);
}
</script>

<template>
  <div>
    <div class="flex flex-col h-full">
      <n-flex class="w-full flex items-center">
        <auto-complete-tag
            ref = "tagRef1"
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
                class="svg-button w-6 h-6 mr-2"
                @click.stop="saveConfigs"
            ></inline-svg>
          </template>
            保存标签组 {{configStore.getShortcutByName("保存标签组")}}
        </n-tooltip>
        <n-dropdown
            trigger="click"
            :options="processedOptions"
            :render-label="renderDropdownLabel"
            :show-arrow="true"
            v-model:show="isDropdownVisible"
            @select="handleSelect"
        >
          <n-tooltip trigger="hover" >
            <template #trigger>
              <inline-svg
                  src="../assets/svg/DropDown24.svg"
                  class="svg-button w-6 h-6 mr-2"
                  @click="isDropdownVisible = !isDropdownVisible"
              ></inline-svg>
            </template>
            显示常用标签组 {{configStore.getShortcutByName("打开常用标签组")}}
          </n-tooltip>
        </n-dropdown>
      </n-flex>
      <auto-complete-tag
          ref = "tagRef2"
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
}想针对针对 dropdown 的弹出层，但是这个会把tooltip也变宽*/
</style>
<style scoped>

</style>