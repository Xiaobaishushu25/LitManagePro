interface TagGroup {
    id: number;
    name: string;
}
interface Tag {
    groupId: number;
    id: number;
    value: string;
    color: string;
}
// 定义 Tags 接口
interface Tags {
    tag_group: TagGroup;
    tags: Tag[];
}
export type {Tags};
