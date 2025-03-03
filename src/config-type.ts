interface UiConfig {
    // tag组是否打开，key为tag_group_name，value为boolean
    tag_group_state: { [key: number]: boolean };
    save_tag_groups: number[][];
    table_expand:boolean,
}

interface ExeConfig {
    name: string,
    path: string,
    icon_path: string,
}

interface AiConfig {
    use_ai:boolean,
    //默认使用的ai，分别为：kimi,deepseek
    default_ai:String,
    //模型，key为ai名称，value为模型名称集合
    default_model:String,
    models:Map<string, string[]>
    //key为ai名称，value为ai的key
    keys:Map<string,string>,
    online:boolean,
}

interface Config {
    ui_config: UiConfig;
    ai_config: AiConfig;
    exe_configs: ExeConfig[];
}

export type { Config, UiConfig,ExeConfig, AiConfig}