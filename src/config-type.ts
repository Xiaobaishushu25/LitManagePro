interface UiConfig {
    split_size:number[],
    // tag 组是否打开，key 为 tag_group_name，value 为 boolean
    tag_group_state: { [key: number]: boolean };
    save_tag_groups: number[][];
    last_use_tags:[number[], number[]];
    table_expand:boolean,
    // 最近打开的笔记 ID 列表
    last_opened_notes: number[];
    // 编辑器模式："ir" (即时渲染), "wysiwyg" (所见即所得), "sv" (分屏)
    editor_mode: "ir" | "wysiwyg" | "sv";
}
interface AppConfig {
    auto_start:boolean,
}

interface ExeConfig {
    name: string,
    path: string,
    icon_path: string,
    is_default:boolean,
}
interface AiConfig {
    use_ai: boolean;
    default_ai: string;
    // default_model: string;
    default_model: Record<string, string>;
    models: Record<string, string[]>; // 使用 Record 来表示键值对
    keys: Record<string, string>; // 使用 Record 来表示键值对
    online: boolean;
    max_concurrency:number;
}

type ShortcutNode = Group | Item;

interface Group {
    name: string;
    children: ShortcutNode[];
}

interface Item {
    name: string;
    shortcut: string;
}
// 类型保护函数
export function isGroup(node: ShortcutNode): node is Group {
    return (node as Group).children !== undefined;
}
interface Config {
    ui_config: UiConfig;
    app_config: AppConfig;
    ai_config: AiConfig;
    exe_configs: ExeConfig[];
    shortcut_tree: ShortcutNode[];
}

export type { Config, UiConfig,ExeConfig, AiConfig, ShortcutNode}