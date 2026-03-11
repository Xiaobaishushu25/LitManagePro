interface TagGroup {
    index: number;
    id: number;
    name: string;
}
interface Tag {
    index: number;
    group_id: number;
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
interface Document {
    index: number;
    id: number;
    title: string;
    file_name: string;
    author?: string | null;
    abstract?: string | null;
    year?: string | null;
    journal?: string | null;
    contributions?: string | null;
    remark?: string | null;
    path: string;
}
interface DocumentTags extends Document {
    tags: Tag[];
}

interface TagResponse {
    doc_id: number;
    tagsId: number[];
}
// interface DocumentTags {
//     index: number;
//     id: number;
//     title: string;
//     author?: string | null;
//     abstract?: string | null;
//     year?: string | null;
//     journal?: string | null;
//     contributions?: string | null;
//     remark?: string | null;
//     path: string;
//     tags: Tag[];
// }
interface NoteResponseDto {
    id: number
    document_id: number
    title?: string | null
    content: string
    created_at: string
    updated_at: string
}
interface NoteTab {
    tabId: string
    noteId: number
    original: NoteResponseDto
    draft: NoteResponseDto
    dirty: boolean
    saving: boolean
    loading: boolean
    closable: boolean
}
export type {TagGroup,Tag,TagAndGroups,DocumentTags,Document,TagResponse,NoteResponseDto,NoteTab};
