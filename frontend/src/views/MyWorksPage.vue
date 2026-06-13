<script setup>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import VideoCard from '../components/VideoCard.vue'
import { listMyVideos } from '../api/videos'

const router = useRouter()

const videos = ref([])
const page = ref(1)
const pageSize = ref(9)
const total = ref(0)
const loading = ref(false)
const error = ref('')

onMounted(() => load())

async function load() {
  loading.value = true
  error.value = ''
  try {
    const res = await listMyVideos({ page: page.value, page_size: pageSize.value })
    const data = res.data.data
    videos.value = data.items
    total.value = data.total
  } catch (err) {
    error.value = err.message
  } finally {
    loading.value = false
  }
}

function totalPages() {
  return Math.max(1, Math.ceil(total.value / pageSize.value))
}

function changePage(n) {
  if (n < 1 || n > totalPages()) return
  page.value = n
  load()
}

function goToVideo(video) {
  const uuid = video.video_url.split('/').pop().replace(/\.mp4$/i, '')
  router.push(`/recommend/${uuid}`)
}

function onDeleted(id) {
  videos.value = videos.value.filter(v => v.id !== id)
  total.value = Math.max(0, total.value - 1)
  if (videos.value.length === 0 && page.value > 1) {
    page.value -= 1
    load()
  }
}

function onUpdated(updated) {
  const idx = videos.value.findIndex(v => v.id === updated.id)
  if (idx !== -1) videos.value[idx] = updated
}
</script>

<template>
  <section class="list-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">作品管理</p>
        <h1>我的作品</h1>
        <p class="muted">共 {{ total }} 个作品</p>
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
        @deleted="onDeleted"
        @updated="onUpdated"
        @navigate="goToVideo"
      />
    </div>

    <div v-else class="empty-state">
      <h2>暂无作品</h2>
      <RouterLink class="primary-button small" to="/publish">发布作品</RouterLink>
    </div>

    <div class="pagination" v-if="totalPages() > 1">
      <button type="button" :disabled="page === 1" @click="changePage(page - 1)">上一页</button>
      <span>第 {{ page }} / {{ totalPages() }} 页</span>
      <button type="button" :disabled="page === totalPages()" @click="changePage(page + 1)">下一页</button>
    </div>
  </section>
</template>
