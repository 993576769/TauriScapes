import { createApp } from 'vue';
import router from './router';
import App from './app.vue';
import '@/styles/tailwind.css';
import '@/styles/global.scss';

const app = createApp(App);

app.use(router).mount('#app');
