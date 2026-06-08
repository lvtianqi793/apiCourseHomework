import { createRouter, createWebHistory } from 'vue-router'

import LoginView from '../views/LoginView.vue'
import MyVideosView from '../views/MyVideosView.vue'
import PublishVideoView from '../views/PublishVideoView.vue'
import RecommendView from '../views/RecommendView.vue'
import RegisterView from '../views/RegisterView.vue'

const routes = [
  { path: '/', redirect: '/recommend' },
  { path: '/login', component: LoginView },
  { path: '/register', component: RegisterView },
  { path: '/recommend', component: RecommendView, meta: { requiresAuth: true } },
  { path: '/publish', component: PublishVideoView, meta: { requiresAuth: true } },
  { path: '/my/videos', component: MyVideosView, meta: { requiresAuth: true } }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to) => {
  const token = localStorage.getItem('token')
  if (to.meta.requiresAuth && !token) {
    return '/login'
  }

  if ((to.path === '/login' || to.path === '/register') && token) {
    return '/recommend'
  }

  return true
})

export default router
