import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import router from './router.ts'
import { createPinia } from 'pinia'
import { PiniaSharedState } from 'pinia-shared-state';
import InlineSvg from 'vue-inline-svg';
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import ContextMenu from '@imengyu/vue3-context-menu'

let pinia = createPinia();
// 使用插件
pinia.use(
    PiniaSharedState({
        enable: true, // 启用插件，默认为 true
        initialize: true, // 是否在初始化时尝试从其他标签页恢复状态，默认为 true
        type: 'localstorage', // 存储类型，可选值为 'native', 'idb', 'localstorage' 或 'node'
    })
);

let app = createApp(App);
app.component('inline-svg', InlineSvg);
app.use(router).use(pinia).use(ContextMenu).mount("#app");
