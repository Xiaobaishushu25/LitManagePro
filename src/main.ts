import { createApp } from "vue";
import App from "./App.vue";
import "./style.css";
import router from './router.ts'
import { createPinia } from 'pinia'


let app = createApp(App);
app.use(router).use(createPinia()).mount("#app");
