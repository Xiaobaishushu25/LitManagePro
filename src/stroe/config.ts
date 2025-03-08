import {defineStore} from "pinia";
import {computed, reactive, ref, watch, watchEffect} from "vue";
import {Config, ExeConfig} from "../config-type.ts";
import {Tag} from "../components/main/main-type.ts";
import useTagGroupsStore from "./tag.ts";
import {invoke} from "@tauri-apps/api/core";

const useConfigStore = defineStore('config', ()=>{
    const tagStore = useTagGroupsStore();
    const config = ref<Config>();
    // 使用普通对象存储 ref（外层不需要再用 ref 包裹）
    const tagGroupStates = reactive<{ [key: number]: boolean }>({});
    const save_tags = ref<Tag[][]>()

    watch(() => config.value, (newConfig) => {
        if (newConfig?.ui_config?.tag_group_state) {
            for (const [key, value] of Object.entries(newConfig.ui_config.tag_group_state)) {
                const numericKey = Number(key);
                if (!isNaN(numericKey)) {
                    tagGroupStates[numericKey] = value;
                }
            }
        }
        if (newConfig!=undefined){
            invoke('update_config', { config: newConfig }).then(() => {}).catch(() => {})
        }
    }, { immediate: true,deep : true });

    watchEffect(() => {//不知道为啥用watch([tagStore.allTags,config.value]不行
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
    function addSaveTags(ids:number[]){
        config.value?.ui_config.save_tag_groups.push(ids)
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
    return {
        config,
        // save_tags_id,
        save_tags,
        addSaveTags,
        removeSaveTags,
        getTagGroupState,
        addNewExecution,
    }
}, {
    share: {
        enable: true, // 启用共享
        initialize: true, // 在初始化时尝试恢复状态
    },
})
export default useConfigStore