interface TagGroup {
    index: number;
    id: number;
    name: string;
}
interface Tag {
    index: number;
    groupId: number;
    id: number;
    value: string;
    bg_color: string;
    text_color: string;
}
// 定义 Tags 接口
interface TagAndGroups {
    tag_group: TagGroup;
    tags: Tag[];
}
export type {TagGroup,Tag,TagAndGroups};
