interface UiConfig {
    // tag组是否打开，key为tag_group_name，value为boolean
    tag_group_state: { [key: number]: boolean };
}

interface Config {
    ui_config: UiConfig;
}

export type { Config, UiConfig}