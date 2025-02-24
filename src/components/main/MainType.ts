interface TagGroup {
    id: number;
    name: string;
}
interface Tag {
    groupId: number;
    id: number;
    value: string;
    bg_color: string;
    text_color: string;
}
// 定义 Tags 接口
interface Tags {
    tag_group: TagGroup;
    tags: Tag[];
}
export type {TagGroup,Tag,Tags};
