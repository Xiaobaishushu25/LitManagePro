import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import router from './router.ts'
import { createPinia } from 'pinia'
import InlineSvg from 'vue-inline-svg';
import '@imengyu/vue3-context-menu/lib/vue3-context-menu.css'
import ContextMenu from '@imengyu/vue3-context-menu'


let app = createApp(App);
app.component('inline-svg', InlineSvg);
app.use(router).use(createPinia()).use(ContextMenu).mount("#app");
