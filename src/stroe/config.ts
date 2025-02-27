import {defineStore} from "pinia";
import {computed, reactive, ref, watch} from "vue";
import {Config} from "../config-type.ts";

const useConfigStore = defineStore('config', ()=>{
    const config = ref<Config>();
    // 使用普通对象存储 ref（外层不需要再用 ref 包裹）
    const tagGroupStates = reactive<{ [key: number]: boolean }>({});

    // 初始化逻辑（需要确保在 config 加载后执行）
    // watch(() => config.value, (newConfig) => {
    //     if (newConfig?.ui_config.tag_group_state) {
    //         for (const [key, value] of Object.entries(newConfig.ui_config.tag_group_state)) {
    //             tagGroupStates[parseInt(key)] = value;
    //         }
    //     }
    // }, { immediate: true });
    watch(() => config.value, (newConfig) => {
        if (newConfig?.ui_config?.tag_group_state) {
            for (const [key, value] of Object.entries(newConfig.ui_config.tag_group_state)) {
                const numericKey = Number(key);
                if (!isNaN(numericKey)) {
                    tagGroupStates[numericKey] = value;
                }
            }
        }
    }, { immediate: true });

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

    return {
        config,
        getTagGroupState,
    }
})
export default useConfigStore