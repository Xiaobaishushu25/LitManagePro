// history模式
import {
    createRouter,
    createWebHashHistory, RouteRecordRaw,
} from 'vue-router'
import MainWindow from "./windows/MainWindow.vue";
import Setting from "./windows/Setting.vue";
import DragImport from "./windows/DragImport.vue";


const routes: Array<RouteRecordRaw> = [
    // 主界面的路由
    {
        path: '/',
        redirect: '/main'
    },
    {
        path: '/main',
        component: MainWindow, // 主界面的布局组件
    },
    {
        path: '/setting',
        component: Setting, // 工具界面的布局组件
    },
    {
        path: '/dragImport',
        component: DragImport, // 工具界面的布局组件
    },
]

// 创建路由对象
const router = createRouter({
    history: createWebHashHistory(),
    routes
})
export default router;