import {defineStore} from "pinia";
import {TagAndGroups, Tag, TagGroup} from "../components/main/main-type.ts";
import {ref, watch} from "vue";

const useTagGroupsStore = defineStore('tagGroups', ()=>{
    const tagGroups = ref<TagAndGroups[]>([])
    const currentSelectTags = ref<Tag[]>([])
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
    function deleteTagGroup(tagGroup:TagGroup){
        tagGroups.value = tagGroups.value.filter((group)=>{
            return group.tag_group.id !== tagGroup.id
        })
    }
    function addNewTag(tag:Tag){
        // 找到第一个 tag_group.id 等于 tag.groupId 的元素，并添加 tag 到它的 tags 数组中
        const tagGroup = tagGroups.value.find((group) => group.tag_group.id === tag.group_id);
        if (tagGroup) {
            tagGroup.tags.push(tag);
        }
    }
    function deleteTag(tag:Tag){
        // 找到第一个 tag_group.id 等于 tag.groupId 的元素，并删除 tag 对应的元素
        const tagGroup = tagGroups.value.find((group) => group.tag_group.id === tag.group_id);
        if (tagGroup) {
            tagGroup.tags = tagGroup.tags.filter((t) => t.id !== tag.id);
        }
    }
    function addTagToCurrentSelectTags(id:number){
        let tag = allTags.value.find(tag => tag.id === id);
        currentSelectTags.value.push(tag);
    }
    return {
        tagGroups,
        currentSelectTags,
        allTags,
        addNewTagGroup,
        deleteTagGroup,
        addNewTag,
        deleteTag,
        addTagToCurrentSelectTags,
    }
})
export default useTagGroupsStore