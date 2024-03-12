import { createApp } from 'vue';
import { createPinia } from 'pinia';
import router from './router';
import App from './app.vue';
import '@/styles/tailwind.css';
import '@/styles/global.scss';

const app = createApp(App);
const pinia = createPinia();

app.use(router);
app.use(pinia);
app.mount('#app');
