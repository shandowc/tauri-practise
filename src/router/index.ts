import { createRouter, createWebHashHistory } from "vue-router";
import SelectView from '../views/SelectView.vue';
import HomeView from '../views/HomeView.vue';
import ShowMain from '../components/ShowMain.vue';
import Setting from '../components/Setting.vue';

const router = createRouter({
    history: createWebHashHistory(),
    routes: [{
        path: '/select',
        component: SelectView,
    }, {
        path: '/index',
        component: HomeView,
        children: [
            {
                path: '',
                component: ShowMain,
            }, {
                path: 'setting',
                component: Setting,
            },
        ]
    }],
});

router.beforeEach((to, from, next) => {
    const debugfolder: string | null = sessionStorage.getItem("debugfolder")
    if (!debugfolder && to.path !== "/select") {
        next("/select")
    } else if (to.path === '/') {
        next('/index');
    } else {
        next()
    }
});

export default router