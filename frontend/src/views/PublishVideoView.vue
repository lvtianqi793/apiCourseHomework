<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

import { publishVideo } from '../api/videos'

const router = useRouter()
const title = ref('')
const description = ref('')
const file = ref(null)
const loading = ref(false)
const progress = ref(0)
const error = ref('')
const success = ref('')

function handleFileChange(event) {
  const selectedFile = event.target.files?.[0]
  error.value = ''

  if (!selectedFile) {
    file.value = null
    return
  }

  if (!selectedFile.name.toLowerCase().endsWith('.mp4')) {
    file.value = null
    error.value = '只允许上传 mp4 视频'
    return
  }

  if (selectedFile.size > 500 * 1024 * 1024) {
    file.value = null
    error.value = '视频大小不能超过 500MB'
    return
  }

  file.value = selectedFile
}

async function submit() {
  error.value = ''
  success.value = ''

  if (!file.value) {
    error.value = '请选择 mp4 视频文件'
    return
  }

  const formData = new FormData()
  formData.append('title', title.value)
  formData.append('description', description.value)
  formData.append('file', file.value)

  loading.value = true
  progress.value = 0

  try {
    await publishVideo(formData, (event) => {
      if (event.total) {
        progress.value = Math.round((event.loaded * 100) / event.total)
      }
    })
    success.value = '发布成功，正在打开我的作品...'
    setTimeout(() => router.push('/my/videos'), 600)
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <section class="form-page">
    <form class="content-card" @submit.prevent="submit">
      <p class="eyebrow">作品发布</p>
      <h1>发布作品</h1>
      <p class="muted">上传本地 mp4 视频，发布后会进入推荐序列。</p>

      <label>
        标题
        <input v-model.trim="title" required maxlength="128" placeholder="输入作品标题" />
      </label>

      <label>
        简介
        <textarea v-model.trim="description" rows="4" placeholder="添加作品简介"></textarea>
      </label>

      <label>
        视频文件
        <input accept="video/mp4,.mp4" required type="file" @change="handleFileChange" />
      </label>

      <div class="progress-bar" v-if="loading">
        <span :style="{ width: `${progress}%` }"></span>
      </div>

      <p v-if="error" class="error-text">{{ error }}</p>
      <p v-if="success" class="success-text">{{ success }}</p>

      <button class="primary-button" type="submit" :disabled="loading">
        {{ loading ? `上传中 ${progress}%` : '发布作品' }}
      </button>
    </form>
  </section>
</template>
