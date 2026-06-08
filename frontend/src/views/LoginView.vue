<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()
const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function submit() {
  error.value = ''
  loading.value = true

  try {
    await authStore.login(username.value, password.value)
    router.push('/recommend')
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <section class="auth-page">
    <form class="auth-card" @submit.prevent="submit">
      <p class="eyebrow">账号登录</p>
      <h1>登录</h1>
      <p class="muted">使用账号进入视频精选，管理你的作品与互动记录。</p>

      <label>
        用户名
        <input v-model.trim="username" required minlength="3" maxlength="64" placeholder="请输入用户名" />
      </label>

      <label>
        密码
        <input v-model="password" required minlength="6" maxlength="64" type="password" placeholder="请输入密码" />
      </label>

      <p v-if="error" class="error-text">{{ error }}</p>

      <button class="primary-button" type="submit" :disabled="loading">
        {{ loading ? '登录中...' : '登录' }}
      </button>

      <p class="muted center">
        还没有账号？
        <RouterLink to="/register">去注册</RouterLink>
      </p>
    </form>
  </section>
</template>
