<script setup lang="ts">
import TitleBar from "../components/TitleBar.vue";
import {onMounted,onUnmounted} from "vue";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import useTagGroupsStore from "../stroe/tag.ts";

const tagStore = useTagGroupsStore()

let unlistenFile: () => void;

onMounted(async ()=>{
  unlistenFile = await listen('tauri://drag-drop', async (event:{ payload:{paths: string[]}})=>{
    let selectTagId = tagStore.currentSelectTags.map(tag => tag.id)
    console.log(event.payload.paths)
    await invoke('insert_docs', {paths: event.payload.paths, tagsId: selectTagId})
  })
})
onUnmounted(()=>{
  unlistenFile()
})
</script>

<template>
  <TitleBar title="拖拽上传"></TitleBar>
  <div class="drag-upload-container" >
    <div class="drag-upload-content">
      <div class="upload-text">拖拽文件到此处上传</div>
      <div class="upload-text-small">支持多种文件格式</div>
      <div class="upload-text-small">支持多个文件上传</div>
    </div>
  </div>
</template>

<style scoped>
.drag-upload-container {
  width: 100%;
  height: calc(100vh - 30px);
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #f5f5f5;
}

/*.drag-upload-container:hover {
  background-color: #4a914a40;
}*/

.drag-upload-content {
  width: 85%;
  height: 85%;
  text-align: center;
  padding: 40px;
  border: 2px dashed #ccc;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

.drag-upload-container:hover .drag-upload-content {
  border-color: #999;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
}

.upload-text {
  font-size: 30px;
  color: #333;
  margin-bottom: 10px;
}

.upload-text-small {
  font-size: 20px;
  color: #666;
  margin-top: 15px;
}
</style>