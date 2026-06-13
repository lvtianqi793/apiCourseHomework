import { createRouter, createWebHistory } from 'vue-router'

import LoginView from '../views/LoginView.vue'
import MyFavoritesPage from '../views/MyFavoritesPage.vue'
import MyVideosView from '../views/MyVideosView.vue'
import MyWorksPage from '../views/MyWorksPage.vue'
import PublishVideoView from '../views/PublishVideoView.vue'
import RecommendView from '../views/RecommendView.vue'
import RegisterView from '../views/RegisterView.vue'
import SearchView from '../views/SearchView.vue'
import UserProfileView from '../views/UserProfileView.vue'

const routes = [
  { path: '/', redirect: '/recommend' },
  { path: '/login', component: LoginView },
  { path: '/register', component: RegisterView },
  { path: '/recommend/:uuid?', component: RecommendView, meta: { requiresAuth: true } },
  { path: '/publish', component: PublishVideoView, meta: { requiresAuth: true } },
  { path: '/search', component: SearchView, meta: { requiresAuth: true } },
  { path: '/my', component: MyVideosView, meta: { requiresAuth: true } },
  { path: '/my/works', component: MyWorksPage, meta: { requiresAuth: true } },
  { path: '/my/favorites', component: MyFavoritesPage, meta: { requiresAuth: true } },
  { path: '/my/videos', redirect: '/my' },
  { path: '/users/:id', component: UserProfileView, meta: { requiresAuth: true } },
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
