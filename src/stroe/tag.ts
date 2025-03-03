import {defineStore} from "pinia";
import {TagAndGroups, Tag, TagGroup} from "../components/main/main-type.ts";
import {computed, ref, watch} from "vue";

const useTagGroupsStore = defineStore('tagGroups', ()=>{
    const tagGroups = ref<TagAndGroups[]>([])
    const andTags = ref<Tag[]>([])
    const orTags = ref<Tag[]>([])
    // 定义一个 computed 属性，自动合并 andTags 和 orTags
    const currentSelectTags = computed<Tag[]>(() => {
        return andTags.value.concat(orTags.value); // 或者使用 [...andTags.value, ...orTags.value]
    });
    const allTags = ref<Tag[]>([])
    // 监听 tagGroups 的变化
    watch(tagGroups, (newTagGroups) => {
        // 将所有 tagGroups 中的 tags 提取出来，形成一个扁平化的数组
        allTags.value = newTagGroups.flatMap((group) => group.tags);
    }, { immediate: true,deep:true });
    function addNewTagGroup(tagGroup:TagGroup){
        tagGroups.value.push({
            tag_group:tagGroup,
            tags:[]
        })
    }
    //遍历tagGroups.value 数组，保留所有 tag_group.id 不等于 tagGroup.id 的元素。
    function deleteTagGroup(group_id:number){
        tagGroups.value = tagGroups.value.filter((group)=>{
            return group.tag_group.id !== group_id
        })
    }
    function renameTagGroup(group_id:number,new_name:string){
        const groupIndex = tagGroups.value.findIndex((group) => group.tag_group.id === group_id);
        if (groupIndex !== -1) {
            // 更新找到的标签组的名称
            tagGroups.value[groupIndex].tag_group.name = new_name;
        }
    }
    function addNewTag(tag:Tag){
        // 找到第一个 tag_group.id 等于 tag.groupId 的元素，并添加 tag 到它的 tags 数组中
        const tagGroup = tagGroups.value.find((group) => group.tag_group.id === tag.group_id);
        if (tagGroup) {
            tagGroup.tags.push(tag);
        }
    }
    function deleteTag(groupId:number,tagId:number){
        // 找到第一个 tag_group.id 等于 tag.groupId 的元素，并删除 tag 对应的元素
        const tagGroup = tagGroups.value.find((group) => group.tag_group.id === groupId);
        if (tagGroup) {
            tagGroup.tags = tagGroup.tags.filter((t) => t.id !== tagId);
        }
    }
    function addTagToAndTags(id:number){
        let tag = allTags.value.find(tag => tag.id === id);
        if (tag) {
            const existingTag = andTags.value.find(t => t.id === tag.id);
            if (!existingTag) {
                andTags.value.push(tag);
            }
        }
    }
    function setAllAndTags(ids:number[]){
        andTags.value = allTags.value.filter(tag => ids.includes(tag.id));
    }
    function addTagToOrTags(id:number){
        // let tag = allTags.value.find(tag => tag.id === id);
        let tag = allTags.value.find(tag => tag.id === id);
        if (tag) {
            // 检查或标签中是否已存在该标签
            const existingTag = orTags.value.find(t => t.id === tag.id);
            if (!existingTag) {
                orTags.value.push(tag);
            }
        }
    }
    return {
        tagGroups,
        currentSelectTags,
        andTags,
        orTags,
        allTags,
        addNewTagGroup,
        deleteTagGroup,
        addNewTag,
        deleteTag,
        addTagToAndTags,
        addTagToOrTags,
        renameTagGroup,
        setAllAndTags,
    }
})
export default useTagGroupsStore