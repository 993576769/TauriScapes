import type { RouteRecordRaw } from 'vue-router';
import { createRouter, createWebHistory } from 'vue-router';

const Home = () => import('@/views/home/index.vue');
const Settings = () => import('@/views/settings/index.vue');

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'Home',
    component: Home,
  },
  {
    path: '/',
    name: 'Settings',
    component: Settings,
  },
];

const router = createRouter({
  history: createWebHistory(import.meta.env.VITE_APP_ROUTER_BASE_URL),
  routes,
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return savedPosition;
    } else {
      return { top: 0 };
    }
  },
});

export default router;
