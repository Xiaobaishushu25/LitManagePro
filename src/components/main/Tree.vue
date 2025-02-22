<template>
  <n-tree
      block-line
      :data="data"
      :default-expanded-keys="defaultExpandedKeys"
      key-field="whateverKey"
      label-field="whateverLabel"
      children-field="whateverChildren"
      selectable
      :render-prefix="renderPrefix"
      :node-props="nodeProps"
  />
  <n-dropdown
      trigger="manual"
      placement="bottom-start"
      :show="showDropdown"
      :options="options"
      :x="x"
      :y="y"
      @select="handleSelect"
      @clickoutside="handleClickoutside"
  />
</template>

<script setup lang="ts">
import type { TreeOption, DropdownOption } from 'naive-ui'
import {h, ref} from 'vue'
import {message} from "../../message.ts";

// 模拟初始数据
const data = ref<TreeOption[]>([
  {
    whateverKey: '1',
    whateverLabel: '根节点 1',
    whateverChildren: [
      {
        whateverKey: '1-1',
        whateverLabel: '子节点 1-1',
        whateverChildren: [
          {
            whateverKey: '1-1-1',
            whateverLabel: '子节点 1-1-1'
          },
          {
            whateverKey: '1-1-2',
            whateverLabel: '子节点 1-1-2'
          }
        ]
      },
      {
        whateverKey: '1-2',
        whateverLabel: '子节点 1-2'
      }
    ]
  },
  {
    whateverKey: '2',
    whateverLabel: '根节点 2',
    whateverChildren: [
      {
        whateverKey: '2-1',
        whateverLabel: '子节点 2-1'
      },
      {
        whateverKey: '2-2',
        whateverLabel: '子节点 2-2'
      }
    ]
  }
])

// 默认展开的节点
const defaultExpandedKeys = ref(['1', '1-1'])

// 右键菜单相关
const showDropdown = ref(false)
const options = ref<DropdownOption[]>([
  {
    label: '选项1',
    key: 'option1'
  },
  {
    label: '选项2',
    key: 'option2'
  },
  {
    label: '选项3',
    key: 'option3'
  }
])
const x = ref(0)
const y = ref(0)

const handleSelect = (key: string) => {
  message.info(`选择了: ${key}`)
  showDropdown.value = false
}

const handleClickoutside = () => {
  showDropdown.value = false
}

const nodeProps = ({ option }: { option: TreeOption }) => {
  return {
    onClick() {
      message.info(`[点击] ${option.whateverLabel}`)
    },
    onContextmenu(e: MouseEvent): void {
      e.preventDefault()
      showDropdown.value = true
      x.value = e.clientX
      y.value = e.clientY
    }
  }
}

// 自定义节点图标
const renderPrefix = ({ option }: { option: TreeOption }) => {
  return h('img', {
    src: 'public/assets/svg/Folder24Regular.svg',
    style: {
      width: '16px',
      height: '16px'
    }
  })
}
</script>

<style scoped>

</style>