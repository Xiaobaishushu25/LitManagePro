import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import router from './router.ts'
import { createPinia } from 'pinia'
import InlineSvg from 'vue-inline-svg';


let app = createApp(App);
app.component('inline-svg', InlineSvg);
app.use(router).use(createPinia()).mount("#app");
