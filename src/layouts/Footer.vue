<script setup lang="ts">
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import {listen} from "@tauri-apps/api/event";

interface Progress {
  id:number,
  show:boolean,
  msg:string
  now:number,
  max:number,
}
const progresss = ref<Progress[]>([])
const total_progress = ref<Progress|null>(null)
watch(
    progresss,
    () => {
      if (!progresss.value) {
        return null;
      }
      if (progresss.value.length === 0) {
        return null;
      }
      const totalNow = progresss.value.reduce((sum, p) => sum + p.now, 0);
      const totalMax = progresss.value.reduce((sum, p) => sum + p.max, 0);
      total_progress.value = {
        id: -1, // 虚拟 ID
        show: true,
        msg: 'Total Progress',
        now: totalNow,
        max: totalMax,
      };
      // 检测进度是否完成
      if (totalNow >= totalMax) {
        processing.value = false; // 停止处理动画
        // 5秒后重置 data
        setTimeout(() => {
          progresss.value = [];
          total_progress.value = null;
        }, 5000);
      } else {
        processing.value = true; // 继续处理动画
      }
    },
    { immediate: true,deep:true } // Trigger the watcher immediately to handle initial value
)

//total_progress的msg不好用，直接新建一个newMessage
const newMessage = ref("处理中...")
//控制进度条的处理状态
const processing = ref<boolean>(false);

let unlistenProgress: () => void;
onMounted(async ()=>{
// 定义监听进度事件的函数，返回一个取消监听的函数
  unlistenProgress = await listen('progress_event', (event: { payload: Progress }) => {
    // 解构事件中的新进度数据
    const newProgress = event.payload
    // 更新消息内容
    newMessage.value = newProgress.msg
    // 在 progresss 数组中查找是否存在具有相同 ID 的进度项
    const existingIndex = progresss.value.findIndex((p) => p.id === newProgress.id)
    // 如果找到对应的进度项
    if (existingIndex !== -1) {
      // 将现有项的属性与新进度数据合并，覆盖更新
      progresss.value[existingIndex] = { ...progresss.value[existingIndex], ...newProgress }
    } else {
      // 如果未找到，将新进度数据推入数组
      progresss.value.push(newProgress)
    }
  })
})

onUnmounted(()=>{
  unlistenProgress()
})
// 计算进度百分比
const percentage = computed(() => {
  if (!total_progress.value) {
    return 0; // 没有数据时不显示进度
  }
  const { now, max } = total_progress.value;
  if (max === 0) {
    return 100; // 防止除以零
  }
  const percent = (now / max) * 100;
  return Math.round(percent * 100) / 100; // 保留两位小数
});
</script>

<template>
  <div>
    <n-flex class="bg-gray-200 size-full" align="center">
      <n-flex align="center" v-if="total_progress" class="ml-auto mr-5" :size="10">
        {{total_progress}}
        {{ newMessage }}
        <n-progress
            type="line"
            :percentage="percentage"
            indicator-placement="inside"
            stroke-width="1"
            :height="10"
            style="width: 200px;"
            rail-color="#87CEFA"
            :processing="processing"
        />
      </n-flex>
    </n-flex>
  </div>
</template>

<style scoped>

</style>