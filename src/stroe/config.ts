import {defineStore} from "pinia";
import {computed, reactive, ref, watch, watchEffect} from "vue";
import {Config, ExeConfig, isGroup, ShortcutNode} from "../config-type.ts";
import {Tag} from "../components/main/main-type.ts";
import useTagGroupsStore from "./tag.ts";
import {invoke} from "@tauri-apps/api/core";

const useConfigStore = defineStore('config', ()=>{
    const tagStore = useTagGroupsStore();
    const config = ref<Config>();
    // 使用普通对象存储 ref（外层不需要再用 ref 包裹）
    const tagGroupStates = reactive<{ [key: number]: boolean }>({});
    const save_tags = ref<Tag[][]>()
    const shortcuts = ref<{ key: string; value: string }[]>()
    watch(
        () => config.value?.shortcut_tree,
        (newVal, _oldVal) => {
            if (newVal===undefined)return
            const result = newVal.flatMap(node => {
                if (isGroup(node)) {
                    return node.children.flatMap(child => {
                        if (!isGroup(child)) {
                            const shortcutEntry = { key: child.name, value: child.shortcut };
                            return [shortcutEntry];
                        } else {
                            return [];
                        }
                    });
                } else {
                    const shortcutEntry = { key: node.name, value: node.shortcut };
                    return [shortcutEntry];
                }
            });
            shortcuts.value = result||[];
        },
        { deep: true }
    );
    //注意，由于这个store是多个页面共享的，每个页面都会监听到变化，要注意多次触发的问题
    watch(() => config.value, (newConfig, oldConfig) => {
        //每个页面都会监听到变化，所以防止多次触发
        if (JSON.stringify(newConfig)==JSON.stringify(oldConfig))return
        if (newConfig?.ui_config?.tag_group_state) {
            for (const [key, value] of Object.entries(newConfig.ui_config.tag_group_state)) {
                const numericKey = Number(key);
                if (!isNaN(numericKey)) {
                    tagGroupStates[numericKey] = value;
                }
            }
        }
        //JSON.stringify(newConfig)!=JSON.stringify(oldConfig)防止多次触发
        // if (oldConfig!=undefined&&newConfig!=undefined&&JSON.stringify(newConfig)!=JSON.stringify(oldConfig)){
        if (oldConfig!=undefined&&newConfig!=undefined){
            invoke('update_config', { config: newConfig }).then(() => {}).catch(() => {})
        }

    }, { immediate: true,deep : true });

    watchEffect(() => {//不知道为啥用watch([tagStore.allTags,config.value]不行,监听不到变化
        if (config.value===undefined) return
        const result: Tag[][] = config.value.ui_config.save_tag_groups.map(rowIds => {
            return rowIds.map(id => {
                const tag = tagStore.allTags.find(t => t.id === id)
                return tag // 如果找不到，返回 undefined，后续会过滤掉
            }).filter(tag => tag !== undefined) as Tag[] // 过滤掉 undefined
        })
        save_tags.value = result
    });

    // 监听 tagGroupStates 的变化并更新 config
    watch(tagGroupStates, (newStates, _oldStates) => {
        if (config.value?.ui_config) {
            config.value.ui_config.tag_group_state = newStates;
        }
    }, { deep: true });

    // 方法：返回带有 getter/setter 的计算属性
    function getTagGroupState(id: number) {
        return computed({
            get: () => tagGroupStates[id] ?? true, // 默认 true
            set: (val) => { tagGroupStates[id] = val; }
        });
    }
    //保存tags
    function addSaveTags(ids:number[]){
        config.value?.ui_config.save_tag_groups.push(ids)
    }
    // 保存最后使用的tags,必须在应用关闭时调用！！
    function saveLastUseTags(){
        config.value!.ui_config.last_use_tags[0] = tagStore.andTags.map(tag=>tag.id)
        config.value!.ui_config.last_use_tags[1] = tagStore.orTags.map(tag=>tag.id)
    }
    function removeSaveTags(id_sum:number){
        // 找到和为 id_sum 的元素的索引
        const index = config.value?.ui_config.save_tag_groups?.findIndex(row => {
            const sum = row.reduce((acc, curr) => acc + curr, 0)
            return sum === id_sum
        })
        if(index==undefined) return
        // 如果找到，删除该元素
        if (index !== -1) {
            config.value?.ui_config.save_tag_groups.splice(index, 1)
        }
    }

    function addNewExecution(exe_config:ExeConfig){
        config.value?.exe_configs.push(exe_config)
    }
    function getShortcutByName(name:string){
        return shortcuts.value?.find(item => item.key === name)?.value
    }
    function updateShortcut(name:string,shortcut:string){
        const updateShortcut = (nodes: ShortcutNode[], name: string, newShortcut: string) => {
            for (const node of nodes) {
                if (isGroup(node)) {
                    // 如果是 Group 类型，递归遍历其 children
                    updateShortcut(node.children, name, newShortcut);
                } else {
                    // 如果是 Item 类型，检查名称是否匹配
                    if (node.name === name) {
                        node.shortcut = newShortcut;
                    }
                }
            }
        };
        // 调用递归函数更新快捷键
        updateShortcut(config.value?.shortcut_tree || [], name, shortcut);
    }
    return {
        config,
        save_tags,
        shortcuts,
        saveLastUseTags,
        addSaveTags,
        removeSaveTags,
        getTagGroupState,
        addNewExecution,
        updateShortcut,
        getShortcutByName,
    }
}, {
    share: {
        enable: true, // 启用共享
        initialize: true, // 在初始化时尝试恢复状态
    },
})
export default useConfigStore