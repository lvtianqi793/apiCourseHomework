import { defineStore } from 'pinia'

import { loginApi, registerApi } from '../api/auth'

function readUser() {
  try {
    return JSON.parse(localStorage.getItem('user') || 'null')
  } catch {
    return null
  }
}

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: localStorage.getItem('token') || '',
    user: readUser()
  }),
  getters: {
    isLoggedIn: (state) => Boolean(state.token)
  },
  actions: {
    async register(username, password) {
      await registerApi({ username, password })
    },
    async login(username, password) {
      const response = await loginApi({ username, password })
      const { token, user } = response.data.data

      this.token = token
      this.user = user
      localStorage.setItem('token', token)
      localStorage.setItem('user', JSON.stringify(user))
    },
    logout() {
      this.token = ''
      this.user = null
      localStorage.removeItem('token')
      localStorage.removeItem('user')
    }
  }
})
