<script setup lang="ts">

</script>

<template>
  <div>
    <n-space vertical>
      <n-tag
          class="text-xs tag"
          type="success"
          :closable="true"
          :color="{color: '#d2666650',textColor: '#101010'}"
          :bordered="false"
      >
        扩散模型
      </n-tag>
    </n-space>
  </div>

</template>

<style scoped>
.tag :deep(g) {
  @apply  fill-black;
}
.tag :deep(button) {
  @apply  hover:border-red-600 shadow-none active:border-transparent;
}
</style>