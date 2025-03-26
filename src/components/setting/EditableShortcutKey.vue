<script setup lang="ts">
import {nextTick, ref} from "vue";
import {message} from "../../message.ts";
import useConfigStore from "../../stroe/config.ts";
import {isGroup, ShortcutNode} from "../../config-type.ts";

const configStore = useConfigStore()
const props = defineProps({
  name: {
    type: String,
    required: true,
  },
  initialKey: {
    type: String,
    required: true,
  },
});

const name:string = props.name;
const isEditing = ref(false);
const inputValue = ref(props.initialKey);
const previousShortcut = ref(props.initialKey); // 用于存储旧的快捷键值
const inputRef = ref<HTMLInputElement | null>(null); // 用于引用输入框
const enterEditMode = () => {
  isEditing.value = true;
  previousShortcut.value = inputValue.value; // 保存当前快捷键值
  nextTick(()=>{
    inputRef.value?.focus(); // 确保输入框获得焦点
  })
};

const exitEditMode = () => {
  isEditing.value = false;
  if (inputValue.value=== previousShortcut.value)return
  if (!isValidShortcut(inputValue.value)) {// 如果输入的快捷键无效，则恢复到之前的值
    inputValue.value = previousShortcut.value;
  }else {
    configStore.updateShortcut(name, inputValue.value)
    // 遍历 shortcut_tree，找到对应的 Item 节点并更新快捷键
    // const updateShortcut = (nodes: ShortcutNode[], name: string, newShortcut: string) => {
    //   for (const node of nodes) {
    //     if (isGroup(node)) {
    //       // 如果是 Group 类型，递归遍历其 children
    //       updateShortcut(node.children, name, newShortcut);
    //     } else {
    //       // 如果是 Item 类型，检查名称是否匹配
    //       if (node.name === name) {
    //         node.shortcut = newShortcut;
    //       }
    //     }
    //   }
    // };
    // // 调用递归函数更新快捷键
    // updateShortcut(configStore.config?.shortcut_tree || [], name, inputValue.value);
  }
};

//不知道为啥不支持shift+0这样的，会被识别成pageup啥的，不过思源也不支持
const handleKeyDown = (e: KeyboardEvent) => {
  e.preventDefault();
  if (e.key === "Backspace"|| e.key === "Escape"){
    message.warning(`不能包含${e.key}键`);
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
  if (displayKey !== "Ctrl" && displayKey !== "Shift" && displayKey !== "Alt") {
    keyCombo += displayKey.toUpperCase();
  }
  let item = configStore.shortcuts?.find(item => item.value == keyCombo)
  if (item){
    message.warning(`注意:"${item.key}"已使用快捷键${item.value}`);
  }
  inputValue.value = keyCombo;
};
const isValidShortcut = (keyCombo: string): boolean => {
  if (!keyCombo.includes("Ctrl") && !keyCombo.includes("Shift") && !keyCombo.includes("Alt")) {
    message.warning("必须包含至少一个修饰键");
    return false;
  }
  // 不能只包含修饰键
  // if (keyCombo === "Ctrl+" || keyCombo === "Shift+" || keyCombo === "Alt+") {
  if (keyCombo.endsWith('+')) {
    message.warning("不能只包含修饰键");
    return false;
  }
  let item = configStore.shortcuts?.find(item => item.value == keyCombo)
  if (item){
    message.warning(`"${item.key}"已使用快捷键${item.value}`);
    return false;
  }
  // 不能包含某些特定的键
  // const forbiddenKeys = ["Backspace", "Delete"];
  // for (const key of forbiddenKeys) {
  //   if (keyCombo.includes(key)) {
  //     message.warning(`不能包含${key}键`);
  //     return false;
  //   }
  // }
  return true;
};
defineExpose({
  enterEditMode,
});
</script>

<template>
<span
    v-if="!isEditing"
    @click="enterEditMode"
    class="ml-auto px-1 text-base text-gray-500 whitespace-nowrap cursor-pointer border rounded bg-gray-100"
>
    {{inputValue }}
  </span>
  <input
      v-else
      type="text"
      ref="inputRef"
      v-model="inputValue"
      @keydown="handleKeyDown"
      @blur="exitEditMode"
      class="ml-auto px-1 text-base border rounded bg-white"
      autofocus
  />
</template>

<style scoped>

</style>