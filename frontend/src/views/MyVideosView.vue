<script setup>
import { onMounted, ref } from 'vue'

import VideoCard from '../components/VideoCard.vue'
import { deleteMyVideo, listMyVideos } from '../api/videos'

const videos = ref([])
const page = ref(1)
const pageSize = ref(6)
const total = ref(0)
const loading = ref(false)
const deletingId = ref(null)
const error = ref('')

onMounted(() => {
  loadVideos()
})

async function loadVideos() {
  loading.value = true
  error.value = ''

  try {
    const response = await listMyVideos({
      page: page.value,
      page_size: pageSize.value
    })
    const result = response.data.data
    videos.value = result.items
    total.value = result.total
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}

async function removeVideo(video) {
  if (!confirm(`确认删除《${video.title}》吗？本地视频文件也会被删除。`)) {
    return
  }

  deletingId.value = video.id
  error.value = ''

  try {
    await deleteMyVideo(video.id)
    if (videos.value.length === 1 && page.value > 1) {
      page.value -= 1
    }
    await loadVideos()
  } catch (err) {
    error.value = err.message
  } finally {
    deletingId.value = null
  }
}

function totalPages() {
  return Math.max(1, Math.ceil(total.value / pageSize.value))
}

function changePage(nextPage) {
  if (nextPage < 1 || nextPage > totalPages()) return
  page.value = nextPage
  loadVideos()
}
</script>

<template>
  <section class="list-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">作品管理</p>
        <h1>我的作品</h1>
        <p class="muted">共 {{ total }} 个作品，删除时会同步移除本地视频文件。</p>
      </div>
      <RouterLink class="primary-button small" to="/publish">发布作品</RouterLink>
    </div>

    <p v-if="error" class="error-text">{{ error }}</p>
    <p v-if="loading" class="muted">加载中...</p>

    <div v-else-if="videos.length" class="video-grid">
      <VideoCard
        v-for="video in videos"
        :key="video.id"
        :video="video"
        :deleting="deletingId === video.id"
        @delete="removeVideo(video)"
      />
    </div>

    <div v-else class="empty-state">
      <h2>暂无作品</h2>
      <p>发布后会在这里统一管理。</p>
      <RouterLink class="primary-button small" to="/publish">发布作品</RouterLink>
    </div>

    <div class="pagination" v-if="totalPages() > 1">
      <button type="button" :disabled="page === 1" @click="changePage(page - 1)">上一页</button>
      <span>第 {{ page }} / {{ totalPages() }} 页</span>
      <button type="button" :disabled="page === totalPages()" @click="changePage(page + 1)">下一页</button>
    </div>
  </section>
</template>
