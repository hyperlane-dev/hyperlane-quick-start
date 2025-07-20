import { createRouter, createWebHistory } from 'vue-router';
import ChatView from '../views/ChatView.vue';

const isDev = process.env.NODE_ENV === 'development';

const routes = [
  {
    path: isDev ? '/' : '/chat/index.html',
    name: 'chat',
    component: ChatView,
  },
];

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

export default router;
