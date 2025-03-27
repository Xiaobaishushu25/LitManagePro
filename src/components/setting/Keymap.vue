<script setup lang="ts">
import {h, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import useConfigStore from "../../stroe/config.ts";
import {isGroup, ShortcutNode} from "../../config-type.ts";
import {TreeOption} from "naive-ui";
import EditableShortcutKey from "./EditableShortcutKey.vue";

const configStore = useConfigStore()

const contextMenuShow = ref(false)
const contextOptions = {
  theme: 'flat',
  name:"",
  shortcut:"",
  zIndex: 3,
  x: 500,
  y: 200
}
const handleBlur = () => {
  contextMenuShow.value = false
};
onMounted(async ()=>{
  window.addEventListener('blur', handleBlur);
})
onUnmounted(()=>{
  window.removeEventListener('blur', handleBlur);
})
const pattern = ref('');
const selectedKeys = ref<(string | number)[]>([]);
// 自定义过滤函数
const filter = (inputValue: string, option: TreeOption) => {
  if (!inputValue) return true;
  const lowerPattern = inputValue.toLowerCase();
  const key = typeof option.key === 'string' ? option.key.toLowerCase() : '';
  return option.label?.toLowerCase().includes(lowerPattern) || key.includes(lowerPattern);
};

// 将 ShortcutNode 转换为 Naive UI 的 TreeOption
const transformToTreeOption = (nodes: ShortcutNode[]): TreeOption[] => {
  return nodes.map(node => {
    if (isGroup(node)) {
      return {
        label: node.name,
        key: node.name,
        children: transformToTreeOption(node.children),
      };
    } else {
      return {
        label: node.name,
        key: node.shortcut,
      };
    }
  });
};
const renderLabel = (info: { option: TreeOption }) => {
  const option = info.option;
  if (option.children) {
    // 如果是分组节点，只显示名称
    return h('div', { style: { display: 'flex', alignItems: 'center' } }, [
      h('span', { style: { whiteSpace: 'nowrap' } }, option.label),
    ]);
  } else {
    const editableShortcutKeyRef = ref(null); // 创建一个 ref 用于 EditableShortcutKey
    return h(
        'div',
        {
          style: { display: 'flex', alignItems: 'center' } ,
          class:'width-full',
          onClick: () => {
            selectedKeys.value = [option.key as string | number];
            editableShortcutKeyRef.value?.enterEditMode(); // 点击时调用 enterEditMode
          },
          onContextmenu: (e: MouseEvent) => {
            e.preventDefault()
            selectedKeys.value = [option.key as string | number];
            nextTick().then(() => {
              contextMenuShow.value = true
              contextOptions.x = e.clientX
              contextOptions.y = e.clientY
              contextOptions.name = option.label as string;
              contextOptions.shortcut = option.key as string;
            })
          },
        },
        [
          h('span', { style: { whiteSpace: 'nowrap' } }, option.label),
          h(EditableShortcutKey, {name:option.label, initialKey: option.key, ref: editableShortcutKeyRef } as any),
        ]
    );
  }
};
let transformedData = transformToTreeOption(configStore.config!.shortcut_tree);
watch(()=>configStore.shortcuts, () => {
  transformedData = transformToTreeOption(configStore.config!.shortcut_tree);
})
const searchValue = ref('');
const shortCutValue = ref('');
const handleKeyDown = (e: KeyboardEvent) => {
  e.preventDefault();
  if (e.key === "Backspace"|| e.key === "Escape"){
    shortCutValue.value = "";
    return ;
  }
  // 捕获按键组合
  let keyCombo = "";
  if (e.ctrlKey) {keyCombo += "Ctrl+";}
  if (e.shiftKey) {keyCombo += "Shift+";}
  if (e.altKey) {keyCombo += "Alt+";}
  // 处理 e.key 的值，将其转换为更友好的显示形式
  let displayKey = e.key;
  switch (displayKey) {
    case "Control":
      displayKey = "Ctrl";
      break;
    case "Shift":
      displayKey = "Shift";
      break;
    case "Alt":
      displayKey = "Alt";
      break;
    default:
      break;
  }
  // 如果当前按下的键不是修饰键，则添加到组合中
  //https://github.com/jaywcjlove/hotkeys-js/issues/13
  if (displayKey !== "Ctrl" && displayKey !== "Shift" && displayKey !== "Alt") {
    if (displayKey === "ArrowDown") {
      keyCombo += "Down";
    } else if (displayKey === "ArrowUp") {
      keyCombo += "Up";
    } else if (displayKey === "ArrowLeft") {
      keyCombo += "Left";
    } else if (displayKey === "ArrowRight") {
      keyCombo += "Right";
    } else {
      keyCombo += displayKey.toUpperCase();
    }
  }
  shortCutValue.value = keyCombo;
  pattern.value = keyCombo;
};
function deleteShortCut(){
  configStore.updateShortcut(contextOptions.name, '')
}
</script>

<template>
  <div>
    <n-scrollbar class="h-[calc(100vh-50px)]">
      <n-flex vertical>
        <div class="setting-card mt-8">
          <div class="setting-card-row">
            <label>使用默认快捷键</label>
            <button>重置</button>
          </div>
        </div>
        <label class="text-2xl ml-5 mt-4 font-bold text-gray-800">综合</label>
        <div class="setting-card">
          <div class="setting-card-row">
            <n-input
                v-model:value="searchValue"
                placeholder="搜索键值"
                @update:value="pattern = $event"
                class="ml-auto px-1 text-base border rounded bg-white"
            />
            <n-input
                type="text"
                v-model:value="shortCutValue"
                @keydown="handleKeyDown"
                placeholder="请按下快捷键"
                class="ml-auto px-1 text-base border rounded bg-white"
                autofocus
            />
          </div>
          <div class="setting-card-row">
            <n-tree
                :pattern="pattern"
                :data="transformedData"
                :render-label="renderLabel"
                :filter="filter"
                :selected-keys="selectedKeys"
                style="width: 100%"
                show-line
                expand-on-click
                block-line
            />
            <context-menu
                v-model:show="contextMenuShow"
                :options="contextOptions"
            >
<!--              todo-->
              <context-menu-item label="重置该快捷键"></context-menu-item>
              <context-menu-item  @click.stop="deleteShortCut" class="group">
                <template #icon>
                  <inline-svg
                      src="../assets/svg/Delete24Regular.svg"
                      class="svg-button cursor-default group-hover:text-red-600"
                  ></inline-svg>
                </template>
                <template #label><span class="group-hover:text-red-600">删除该快捷键</span></template>
              </context-menu-item>
            </context-menu>
          </div>
        </div>
      </n-flex>
    </n-scrollbar>
  </div>
</template>

<style scoped>

</style>