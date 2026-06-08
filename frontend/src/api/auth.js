import http from './http'

export function registerApi(payload) {
  return http.post('/auth/register', payload)
}

export function loginApi(payload) {
  return http.post('/auth/login', payload)
}
