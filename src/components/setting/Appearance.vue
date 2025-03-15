<script setup lang="ts">
import useConfigStore from "../../stroe/config.ts";
import { open } from '@tauri-apps/plugin-dialog';
import {message} from "../../message.ts";
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {ExeConfig} from "../../config-type.ts";
import {h, ref, watch, computed, VNode, nextTick} from "vue";
import InlineSvg from "vue-inline-svg";
import {SelectOption} from "naive-ui";

const configStore = useConfigStore()

const defaultModelRef = ref()

const use_ai = ref(configStore.config?.ai_config.use_ai || false)
const aiSupport = ref(configStore.config?.ai_config.default_ai || "kimi"); // 当前选中的值
const key = ref()
const defaultModel = ref()
const maxConcurrency = ref(configStore.config?.ai_config.max_concurrency || 3)
const onLine = ref(configStore.config?.ai_config.online || false)
// 将字符串数组转换为对象数组
const modelOptions = computed(() => {
  return configStore.config?.ai_config.models[aiSupport.value]?.map(item => ({
    label: item,
    value: item
  }));
});
//------------------------------------表格外观设置begin------------------------------------------
const expand = ref(configStore.config?.ui_config.table_expand||true)
watch(() => expand.value, async (value:boolean) => {
  configStore.config!.ui_config.table_expand = value
})
//------------------------------------表格外观设置end------------------------------------------


//------------------------------------应用程序设置begin------------------------------------------
const defaultExe = ref(configStore.config?.exe_configs.filter((item) => item.is_default)[0]?.name);

const exeOptions = computed(() => {
  return configStore.config?.exe_configs.map(item => ({
    label: item.name,
    value: item.name,
    icon: convertFileSrc(item.icon_path)
  }));
});
watch(()=>defaultExe.value, async (value)=>{
  if (configStore.config==undefined) return
  const exeConfigs = configStore.config.exe_configs;
  exeConfigs.forEach(item => {
    item.is_default = false;
  });
  const foundItem = exeConfigs.find(item => item.name === value);
  if (foundItem) {
    foundItem.is_default = true;
  }
})
async function removeExeConfig(name:string){
  if (configStore.config==undefined) return
  configStore.config.exe_configs = configStore.config.exe_configs.filter(item => item.name !== name)
  // await emit('update_exe_config',configStore.config!.exe_configs)
}
async function openDir(){
  try {
    const path = await open({
      multiple: false,
      directory: false, // 设置为 true 可以选择目录
      filters: [{
        name: '可执行程序',
        extensions: ['exe']
      }]
    });
    console.log(path)
    invoke<ExeConfig>("add_new_exe", {path: path}).then(
        (data) => {
          configStore.addNewExecution(data)
          // emit('update_exe_config',configStore.config!.exe_configs)
          message.success("添加成功")
        }
    ).catch((e) => {message.error(e)})
  } catch (error) {
    message.error(`打开文件选择器时出错:${error}`)
  }
}
const renderExeLabel = (option:{ label: string, value: string, icon: string}) => {
  return h('div', { style: { display: 'flex', alignItems: 'center' } }, [
    h('img', { src: option.icon, class: 'w-4 h-4' }),
    h('span', { style: { marginLeft: '8px' } }, option.label)
  ]);
};
//---------------------------------应用程序设置end------------------------------------------

watch(()=>use_ai.value,async (value)=>{
  if (configStore.config==undefined||value==undefined) return
  console.log("use_ai",value)
  configStore.config.ai_config.use_ai = value
})
watch(()=>aiSupport.value, async (value)=>{{
  if (configStore.config==undefined) return
  let _key: string|undefined = configStore.config.ai_config.keys[value];
  if (_key!=undefined){
    key.value = _key
  }else {
    key.value = ''
  }
  const defaultModelValue = configStore.config.ai_config.default_model[value];
  if (defaultModelValue != undefined) {
    defaultModel.value = defaultModelValue;
  } else {
    // 如果没有默认模型，清除选中值
    defaultModel.value = null;
  }}},{immediate:true})
watch(()=>defaultModel.value, async (value)=>{
  if (configStore.config==undefined) return
  let aiValue = aiSupport.value
  if (value==null){
    delete configStore.config.ai_config.default_model[aiValue];
    return
  }
  configStore.config.ai_config.default_model[aiValue] = value
  let models = configStore.config.ai_config.models;
  // 如果键不存在，初始化为空数组，然后添加值
  if (!models[aiValue]) {
    models[aiValue] = [];
  }
  if (!models[aiValue].includes(value)) {
    // 如果值不存在，添加它
    models[aiValue].push(value);
  }
})
function setNewKey(){
  if (configStore.config && configStore.config.ai_config && configStore.config.ai_config.keys) {
    const currentValue = configStore.config.ai_config.keys[aiSupport.value];
    const newValue = key.value;
    // 检查前后是否有变化
    if (currentValue !== newValue) {
      configStore.config.ai_config.keys[aiSupport.value] = newValue;
    }
  }
}
const options = ref([
  { label: 'ChatGpt', value: 'chatgpt', icon: '../assets/icon/chatgpt.svg' },
  { label: 'DeepSeek', value: 'deepseek', icon: '../assets/icon/deepseek.svg' },
  { label: 'Kimi', value: 'kimi', icon: '../assets/icon/kimi.svg' },
  { label: '通义千问', value: 'tongyi', icon: '../assets/icon/tongyi.svg' },
  { label: '文心一言', value: 'wenxin', icon: '../assets/icon/wenxin.svg' },
]);
// 自定义渲染标签
const renderLabel = (option:{ label: string, value: string, icon: string}) => {
  return h('div', { style: { display: 'flex', alignItems: 'center' } }, [
    h(InlineSvg, { src: option.icon, class: 'w-4 h-4' }),
    h('span', { style: { marginLeft: '8px' } }, option.label)
  ]);
};
const renderOption = (info: { node: VNode, option: SelectOption, selected: boolean }) => {
  const { option } = info;
  // 定义点击事件处理函数
  const handleSvgClick = (event: MouseEvent) => {
    event.stopPropagation(); // 阻止事件冒泡，避免触发选项的默认点击行为
    if (configStore.config?.ai_config.models && option.value) {
      const models = configStore.config.ai_config.models;
      const currentAiSupport = aiSupport.value;
      if (models[currentAiSupport] && Array.isArray(models[currentAiSupport])) {
        // 找到并删除 optionValue
        const index = models[currentAiSupport].indexOf(option.value as string); // 类型断言
        if (index !== -1) {
          models[currentAiSupport].splice(index, 1);
        }
      }
    }
  };
  return h('div',
      {
        style: {display: 'flex', justifyContent: 'space-between', alignItems: 'center', width: '100%',},
        class:'cursor-pointer hover:bg-gray-50',
        onClick: async () => {
          defaultModel.value = option.value
          await nextTick(() => {
            defaultModelRef.value.blur()
          })
        },
  }, [
    h('span', { style: { marginLeft: '15px', color: 'black' } }, option.label),
    h(InlineSvg, { src: '../assets/svg/Delete24Regular.svg', class: 'w-4 h-4 mr-5 svg-button hover:text-red-600',onClick: handleSvgClick, })
  ]);
};
</script>

<template>
  <div>
    <n-scrollbar class="h-[calc(100vh-90px)]">
      <n-flex vertical>
        <label class="text-2xl ml-5 font-bold text-gray-800">外观</label>
        <div class="setting-card">
          <div class="setting-card-row">
            <n-flex class="items-center" :size="2">
              <label>默认展开核心思想行</label>
              <n-tooltip trigger="hover">
                <template #trigger>
                  <inline-svg src="../assets/svg/what.svg" class="w-4 h-4"></inline-svg>
                </template>
                开启后会默认展开所有可展开的核心思想行，关闭后需要单独点击展开。
              </n-tooltip>
            </n-flex>
            <n-switch v-model:value="expand"/>
          </div>
          <div class="setting-card-row">
            <n-flex class="items-center" :size="2">
              <label>添加可执行程序</label>
              <n-tooltip trigger="hover">
                <template #trigger>
                  <inline-svg src="../assets/svg/what.svg" class="w-4 h-4"></inline-svg>
                </template>
                可执行程序将会在表格右键菜单可选，用来打开文件。
              </n-tooltip>
            </n-flex>
            <n-button class="border-red-300" @click="openDir">选择程序</n-button>
          </div>
          <div class="ml-4">
            <div v-for="(config, index) in configStore.config?.exe_configs" :key="index">
              <n-flex class="items-center">
                <n-flex class="items-center w-56">
                  <img :src="convertFileSrc(config.icon_path)" alt="icon" class="w-4 h-4">
                  <n-tooltip trigger="hover">
                    <template #trigger>
                      <label>{{config.name}}</label>
                    </template>
                    {{config.path}}
                  </n-tooltip>
                </n-flex>
                <inline-svg
                    src="../assets/svg/SubtractCircle24Regular.svg"
                    class="svg-button text-red-600 hover:text-red-600 ml-7"
                    @click.stop="removeExeConfig(config.name)"
                ></inline-svg>
              </n-flex>
            </div>
          </div>
          <div class="setting-card-row">
            <n-flex class="items-center" :size="2">
              <label>默认应用</label>
              <n-tooltip trigger="hover">
                <template #trigger>
                  <inline-svg src="../assets/svg/what.svg" class="w-4 h-4"></inline-svg>
                </template>
                双击以及右键菜单中的默认选项会使用该应用打开文件。
              </n-tooltip>
            </n-flex>
            <n-select
                v-model:value="defaultExe"
                :options="exeOptions"
                :render-label="renderExeLabel"
                placeholder="选择一个应用程序"
                size="small"
                style="width: 300px"
            >
              <template #empty>
                <div class="flex items-center">
                  <span>目前没有可选择的应用程序</span>
                </div>
              </template>
            </n-select>
          </div>
          <n-divider class="bg-cyan-700" />
        </div>
        <label class="text-2xl ml-5 font-bold text-gray-800">AI</label>
        <div class="setting-card">
          <div class="setting-card-row">
            <n-flex class="items-center" :size="2">
              <label>是否使用ai分析论文</label>
              <n-tooltip trigger="hover">
                <template #trigger>
                  <inline-svg src="../assets/svg/what.svg" class="w-4 h-4"></inline-svg>
                </template>
                开启后会在添加文档时自动分析论文，总结摘要等数据(目前仅支持pdf文件)。
              </n-tooltip>
            </n-flex>
            <n-switch v-model:value="use_ai"/>
          </div>
          <div class="setting-card-row">
            <n-flex class="items-center" :size="5">
              <label>选择服务商:</label>
              <n-select
                  v-model:value="aiSupport"
                  :options="options"
                  :render-label="renderLabel"
                  size="small"
                  style="width: 200px"
              />
            </n-flex>
            <n-input v-model:value="key" placeholder="请填入对应的apiKey" size="small" style="width: 350px"  @blur="setNewKey"></n-input>
          </div>
          <div class="setting-card-row">
            <label>选择使用模型:</label>
            <n-select
                ref="defaultModelRef"
                v-model:value="defaultModel"
                :options="modelOptions"
                :render-option="renderOption"
                size="small"
                placeholder="选择模型，可以回车添加新模型。"
                filterable
                tag
                clearable
                style="width: 300px"
                :on-create="(v:string) => defaultModel = v"
            >
              <template #empty>
                <div class="flex items-center">
                  <span>没有对应的模型</span>
                </div>
              </template>
            </n-select>
          </div>
          <div class="setting-card-row">
            <n-flex class="items-center" :size="5">
              <label>请求最大并发数量</label>
              <n-tooltip trigger="hover">
                <template #trigger>
                  <inline-svg src="../assets/svg/what.svg" class="w-4 h-4"></inline-svg>
                </template>
                你的请求最大并发数量，可查看ai服务提供商的文档。
              </n-tooltip>
            </n-flex>
            <n-input-number v-model:value="maxConcurrency" size="small" style="width: 100px" :min="1" :max="20" :step="1" @update:value="configStore.config!.ai_config.max_concurrency = $event"></n-input-number>
          </div>
          <div class="setting-card-row">
            <label>是否开启联网搜索</label>
            <n-switch v-model:value="onLine" @update:value="configStore.config!.ai_config.online = $event"/>
          </div>
        </div>
      </n-flex>
    </n-scrollbar>
  </div>
</template>

<style scoped>

</style>