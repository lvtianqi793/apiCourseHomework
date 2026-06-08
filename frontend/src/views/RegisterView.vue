<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()
const username = ref('')
const password = ref('')
const confirmPassword = ref('')
const loading = ref(false)
const error = ref('')

async function submit() {
  error.value = ''

  if (password.value !== confirmPassword.value) {
    error.value = '两次输入的密码不一致'
    return
  }

  loading.value = true

  try {
    await authStore.register(username.value, password.value)
    router.push('/login')
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
      <p class="eyebrow">创建账号</p>
      <h1>注册</h1>
      <p class="muted">完成注册后即可发布作品，并进入推荐序列。</p>

      <label>
        用户名
        <input v-model.trim="username" required minlength="3" maxlength="64" placeholder="3-64 个字符" />
      </label>

      <label>
        密码
        <input v-model="password" required minlength="6" maxlength="64" type="password" placeholder="6-64 个字符" />
      </label>

      <label>
        确认密码
        <input v-model="confirmPassword" required type="password" placeholder="再次输入密码" />
      </label>

      <p v-if="error" class="error-text">{{ error }}</p>

      <button class="primary-button" type="submit" :disabled="loading">
        {{ loading ? '注册中...' : '注册' }}
      </button>

      <p class="muted center">
        已有账号？
        <RouterLink to="/login">去登录</RouterLink>
      </p>
    </form>
  </section>
</template>
