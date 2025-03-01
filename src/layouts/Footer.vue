<script setup lang="ts">
import {onMounted, ref} from "vue";

interface progress{
  show:boolean,
  msg:string
  now:number,
  max:number,
}
const data = ref<progress|null>(null)
onMounted(async ()=>{

})
</script>

<template>
  <div>
    <n-flex class="bg-gray-200 size-full" align="center">
      <n-flex align="center" class="ml-auto mr-5" :size="2">
        进度
        <n-progress
            type="line"
            :percentage="60"
            indicator-placement="inside"
            stroke-width="1"
            :height="10"
            style="width: 200px;"
            class="ml-auto mr-5"
            rail-color="#87CEFA"
            processing
        />
      </n-flex>
    </n-flex>
  </div>
</template>

<style scoped>

</style>