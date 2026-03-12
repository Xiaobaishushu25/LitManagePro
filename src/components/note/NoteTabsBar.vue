<script setup lang="ts">
import { computed } from "vue"
import { NButton, NEllipsis } from "naive-ui"
import {NoteTab} from "../main/main-type.ts";

const props = defineProps<{
  tabs: NoteTab[]
  activeTabId: string | null
}>()

const emit = defineEmits<{
  (e: "change", tabId: string): void
  (e: "close", tabId: string): void
}>()
const normalizedTabs = computed(() =>
    props.tabs.map(tab => ({
      ...tab,
      displayTitle: (tab.draft.title || tab.original.title || "无标题").trim() || "无标题"
    }))
)
function handleClickTab(tabId: string) {
  emit("change", tabId)
}

function handleClose(tabId: string, e: MouseEvent) {
  e.stopPropagation()
  emit("close", tabId)
}
</script>

<template>
  <div class="tabs-bar">
    <div
        v-for="tab in normalizedTabs"
        :key="tab.tabId"
        class="tab-item"
        :class="{ active: tab.tabId === activeTabId }"
        @click="handleClickTab(tab.tabId)"
    >
      <div class="tab-main">
        <span v-if="tab.dirty" class="dirty-dot" />
        <NEllipsis style="max-width: 160px">
          {{ tab.displayTitle }}
        </NEllipsis>
      </div>

      <NButton
          quaternary
          circle
          size="tiny"
          class="tab-close-btn"
          @click="handleClose(tab.tabId, $event)"
      >
        <template #icon>
          <inline-svg
              src="../assets/svg/close.svg"
              class="svg-button group-hover:text-red-600 "
          ></inline-svg>
        </template>
      </NButton>
    </div>
  </div>
</template>

<style scoped>
.tabs-bar {
  height: 42px;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 10px;
  border-bottom: 1px solid #e5e7eb;
  overflow-x: auto;
  overflow-y: hidden;
  background: #fafafa;
}

.tab-item {
  height: 30px;
  min-width: 120px;
  max-width: 220px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 6px;
  padding: 0 8px 0 10px;
  border: 1px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  background: #f3f4f6;
  transition: all 0.2s ease;
  user-select: none;
  flex-shrink: 0;
}

.tab-item:hover {
  background: #e5e7eb;
}

.tab-item.active:hover {
  background: linear-gradient(135deg, #5b6fd6 0%, #6a4190 100%);
}

.tab-item.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-color: #667eea;
  color: #ffffff;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
  font-weight: 600;
}

.tab-main {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 0;
}

.dirty-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #f59e0b;
  flex-shrink: 0;
}

.tab-close-btn {
  flex-shrink: 0;
}
</style>