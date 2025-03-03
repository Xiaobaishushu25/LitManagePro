<script setup lang="ts">
import useConfigStore from "../../stroe/config.ts";
import { open } from '@tauri-apps/plugin-dialog';
import {message} from "../../message.ts";
import {convertFileSrc, invoke} from "@tauri-apps/api/core";
import {ExeConfig} from "../../config-type.ts";
import {emit} from "@tauri-apps/api/event";

const configStore = useConfigStore()
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
                <img :src="convertFileSrc(config.icon_path)" alt="icon">
                <n-tooltip trigger="hover">
                  <template #trigger>
                    <label>{{config.name}}</label>
                  </template>
                  {{config.path}}
                </n-tooltip>
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
            <label class="label-text">总是将标价线段延伸到图表的右边界:</label>
            <n-button>测试</n-button>
          </div>
        </div>
      </n-flex>
    </n-scrollbar>
  </div>
</template>

<style scoped>

</style>