<script setup>
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from './stores/auth'

const router = useRouter()
const authStore = useAuthStore()
const isLoggedIn = computed(() => authStore.isLoggedIn)
const searchQuery = ref('')

function logout() {
  authStore.logout()
  router.push('/login')
}

function doSearch() {
  const q = searchQuery.value.trim()
  if (!q) return
  router.push({ path: '/search', query: { q } })
}

function onSearchKey(e) {
  if (e.key === 'Enter') doSearch()
}
</script>

<template>
  <div class="app-shell">
    <aside class="side-nav">
      <RouterLink class="brand" to="/recommend">
        <span class="logo-mark">♪</span>
        <span>视频精选</span>
      </RouterLink>

      <nav class="side-links">
        <RouterLink class="side-link" to="/recommend">
          <span>✦</span>
          <strong>推荐</strong>
        </RouterLink>

        <RouterLink v-if="isLoggedIn" class="side-link" to="/publish">
          <span>＋</span>
          <strong>发布</strong>
        </RouterLink>

        <RouterLink v-if="isLoggedIn" class="side-link" to="/my">
          <span>◉</span>
          <strong>我的</strong>
        </RouterLink>

        <RouterLink v-if="!isLoggedIn" class="side-link" to="/login">
          <span>●</span>
          <strong>登录</strong>
        </RouterLink>

        <RouterLink v-if="!isLoggedIn" class="side-link" to="/register">
          <span>◎</span>
          <strong>注册</strong>
        </RouterLink>
      </nav>

      <div class="side-download">
        <span class="logo-mark small">♪</span>
        <span>短视频推荐系统</span>
      </div>
    </aside>

    <header class="top-bar">
      <div class="search-shell">
        <input
          v-model="searchQuery"
          class="search-input"
          placeholder="搜索你感兴趣的内容"
          @keydown="onSearchKey"
          aria-label="搜索"
        />
        <button class="search-btn" type="button" @click="doSearch">⌕ 搜索</button>
      </div>

      <nav class="top-actions">
        <RouterLink v-if="isLoggedIn" to="/publish">投稿</RouterLink>
        <RouterLink v-if="isLoggedIn" to="/my">作品</RouterLink>
        <button v-if="isLoggedIn" class="link-button" type="button" @click="logout">
          退出
        </button>
        <RouterLink v-if="!isLoggedIn" to="/login">登录</RouterLink>
        <RouterLink v-if="!isLoggedIn" to="/register">注册</RouterLink>
        <RouterLink v-if="isLoggedIn" class="avatar" to="/my">{{ authStore.user?.username?.slice(0, 1) || 'V' }}</RouterLink>
        <span v-else class="avatar">V</span>
      </nav>
    </header>

    <main class="main-panel">
      <RouterView />
    </main>
  </div>
</template>
