<script setup lang="ts">
import useConfigStore from "../../stroe/config.ts";
import { open } from '@tauri-apps/plugin-dialog';
import {message} from "../../message.ts";
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {ExeConfig} from "../../config-type.ts";
import {emit} from "@tauri-apps/api/event";
import {h, ref, watch} from "vue";
import InlineSvg from "vue-inline-svg";

const configStore = useConfigStore()

const use_ai = ref(configStore.config?.ai_config.use_ai)

const aiSupport = ref('kimi'); // 当前选中的值
const key = ref()
const model = ref()
const modelOptions = ref(configStore.config?.ai_config.models[aiSupport.value])



watch(()=>use_ai.value,async (value)=>{
  if (configStore.config==undefined||value==undefined) return
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
}
})
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
          emit('update_exe_config',configStore.config!.exe_configs)
          message.success("添加成功")
        }
    ).catch((e) => {message.error(e)})
  } catch (error) {
    message.error(`打开文件选择器时出错:${error}`)
  }
}
async function removeExeConfig(name:string){
  if (configStore.config==undefined) return
  configStore.config.exe_configs = configStore.config.exe_configs.filter(item => item.name !== name)
  await emit('update_exe_config',configStore.config!.exe_configs)
}
function setNewKey(){
  if (configStore.config && configStore.config.ai_config && configStore.config.ai_config.keys) {
    configStore.config.ai_config.keys[aiSupport.value] = key.value;
  }
}
watch(()=>configStore.config,async ()=>{
  // await emit('update_exe_config',configStore.config!.exe_configs)
  console.log(configStore.config)
},{deep:true})

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
</script>

<template>
  <div>
    {{configStore.config}}
    <n-scrollbar>
      <n-flex vertical>
        <label class="text-2xl ml-5 font-bold text-gray-800">外观</label>
        <div class="setting-card">
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
          <n-divider />
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
            <n-input v-model:value="key" placeholder="请填入对应的apiKey" size="small" style="width: 300px" class="w-56" @blur="setNewKey"></n-input>
          </div>
          <div class="setting-card-row">
            <n-flex class="items-center" :size="5">
              <label>选择使用模型:</label>
              <n-select
                  v-model:value="aiSupport"
                  :options="options"
                  :render-label="renderLabel"
                  size="small"
                  style="width: 200px"
              />
            </n-flex>
          </div>
        </div>
        <div class="setting-card-row">

        </div>
      </n-flex>
    </n-scrollbar>
  </div>
</template>

<style scoped>

</style>