import {defineStore} from "pinia";
import {computed, reactive, ref, watch} from "vue";
import {Config} from "../config-type.ts";
import {Tag} from "../components/main/main-type.ts";
import useTagGroupsStore from "./tag.ts";


const useConfigStore = defineStore('config', ()=>{
    const tagStore = useTagGroupsStore();
    const config = ref<Config>();
    // 使用普通对象存储 ref（外层不需要再用 ref 包裹）
    const tagGroupStates = reactive<{ [key: number]: boolean }>({});
    // const save_tags_id = ref<number[][]>();
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
        if (newConfig?.ui_config!==undefined){
            const result: Tag[][] = newConfig!.ui_config.save_tag_groups.map(rowIds => {
                return rowIds.map(id => {
                    const tag = tagStore.allTags.find(t => t.id === id)
                    return tag // 如果找不到，返回 undefined，后续会过滤掉
                }).filter(tag => tag !== undefined) as Tag[] // 过滤掉 undefined
            })
            save_tags.value = result
        }
        // save_tags_id.value = newConfig?.ui_config.save_tag_groups;
    }, { immediate: true,deep : true });

    // 监听 tagGroupStates 的变化并更新 config
    watch(tagGroupStates, (newStates, _oldStates) => {
        if (config.value?.ui_config) {
            config.value.ui_config.tag_group_state = newStates;
        }
    }, { deep: true });

    // watch(save_tags_id, (newIds) => {
    //     if (!newIds) return
    //     // 根据 newIds 从 allTags 中筛选出对应的标签
    //     const result: Tag[][] = newIds.map(rowIds => {
    //         return rowIds.map(id => {
    //             const tag = tagStore.allTags.find(t => t.id === id)
    //             return tag // 如果找不到，返回 undefined，后续会过滤掉
    //         }).filter(tag => tag !== undefined) as Tag[] // 过滤掉 undefined
    //     })
    //     save_tags.value = result
    // }, { immediate: true,deep : true })

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

    return {
        config,
        // save_tags_id,
        save_tags,
        addSaveTags,
        removeSaveTags,
        getTagGroupState,
    }
})
export default useConfigStore