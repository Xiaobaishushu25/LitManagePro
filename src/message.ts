// src/utils/message.ts
import { h, createApp, Component, App } from 'vue';
import { NMessageProvider, useMessage, MessageApi, MessageOptions, MessageType } from 'naive-ui';

// 单例模式变量
let messageApi: MessageApi | null = null;
let appInstance: App<Element> | null = null;

// 初始化消息实例
function initMessage() {
    if (messageApi) return;

    // 创建挂载容器
    const container = document.createElement('div');
    document.body.appendChild(container);

    // 创建子组件用于获取 messageApi
    const MessageApp: Component = {
        setup() {
            // 在 NMessageProvider 的上下文中获取 messageApi
            messageApi = useMessage();
            return () => null;
        }
    };

    // 创建包含 Provider 的 Vue 实例
    appInstance = createApp(() =>
        h(NMessageProvider, {placement:'bottom-right'}, {
            default: () => h(MessageApp)
        })
    );

    // 挂载实例
    appInstance.mount(container);
}

/**
 * 显示全局消息
 * @param content - 要显示的消息内容
 * @param options - 额外配置选项
 */
export function showMessage(
    content: string,
    options?: {
        type?: MessageType;
        duration?: number;
    }
) {
    if (!messageApi) {
        initMessage();
    }

    const { type = 'info' } = options || {};
    const duration = type === 'error' ? 0 : 5000; // error 不关闭，其他类型 10 秒
    // 添加完整的 MessageOptions 类型
    const messageOptions: MessageOptions = {
        duration,
        keepAliveOnHover: true,
        closable : true
    };

    // 调用对应的消息类型
    messageApi?.[type](content, messageOptions);
}

// 快捷方法类型定义
interface MessageMethods {
    info: (content: string, duration?: number) => void;
    success: (content: string, duration?: number) => void;
    warning: (content: string, duration?: number) => void;
    error: (content: string, duration?: number) => void;
}

// 快捷方法导出
export const message: MessageMethods = {
    info: (content, duration) => showMessage(content, { type: 'info', duration }),
    success: (content, duration) => showMessage(content, { type: 'success', duration }),
    warning: (content, duration) => showMessage(content, { type: 'warning', duration }),
    error: (content, duration) => showMessage(content, { type: 'error', duration })
};
