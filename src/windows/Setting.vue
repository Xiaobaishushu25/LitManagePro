<script setup lang="ts">
import Appearance from "../components/setting/Appearance.vue";
import TitleBar from "../components/TitleBar.vue";
import Keymap from "../components/setting/Keymap.vue";


</script>

<template>
  <TitleBar title="设置" :show-maximize="false"></TitleBar>
  <div class="setting-container">
    <n-tabs
        type="line"
        placement="left"
        class="setting-tabs"
    >
      <n-tab-pane name="外观" tab="外观">
        <template #tab>
          <div class="custom-tab-label">
            <span class="tab-icon">🎨</span>
            <span>外观</span>
          </div>
        </template>
        <appearance></appearance>
      </n-tab-pane>
      <n-tab-pane name="功能" tab="功能">
        <template #tab>
          <div class="custom-tab-label">
            <span class="tab-icon">⚙️</span>
            <span>功能</span>
          </div>
        </template>
      </n-tab-pane>
      <n-tab-pane name="快捷键" tab="快捷键">
        <template #tab>
          <div class="custom-tab-label">
            <span class="tab-icon">⌨️</span>
            <span>快捷键</span>
          </div>
        </template>
        <keymap></keymap>
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>
.setting-container {
  height: calc(100vh - 40px);
  padding: 8px 12px;
}

.setting-tabs {
  height: 100%;
  --n-tab-border-radius: 6px;
  --n-text-color-primary: #333;
  --n-text-color-hover: #18a058;
  --n-color-embedded: #f5f7f9;
}

/* 自定义标签样式 */
.custom-tab-label {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  font-size: 14px;
  font-weight: 500;
}

.tab-icon {
  font-size: 16px;
  width: 20px;
  text-align: center;
}

/* 选中状态的标签 */
:deep(.n-tabs-tab--active) {
  color: var(--n-text-color-hover);
  background-color: rgba(24, 160, 88, 0.08);
  border-radius: 6px;
  margin: 4px 8px;
}

/* 标签栏容器 */
:deep(.n-tabs-rail) {
  background-color: transparent;
  padding: 4px;
}

/* 内容区域 */
:deep(.n-tabs-content) {
  background-color: transparent;
  padding-left: 8px;
}

/* 移除默认的内边距 */
:deep(.n-tab-pane) {
  padding: 0;
}
</style>