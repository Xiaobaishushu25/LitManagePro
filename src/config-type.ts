interface UiConfig {
    split_size:number[],
    // tag组是否打开，key为tag_group_name，value为boolean
    tag_group_state: { [key: number]: boolean };
    save_tag_groups: number[][];
    last_use_tags:[number[], number[]];
    table_expand:boolean,
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

interface Config {
    ui_config: UiConfig;
    ai_config: AiConfig;
    exe_configs: ExeConfig[];
}

export type { Config, UiConfig,ExeConfig, AiConfig}