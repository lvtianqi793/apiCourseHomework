<script setup>
import { onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { listMyFavorites } from '../api/videos'
import { formatCount } from '../utils/format'

const router = useRouter()

function goToVideo(video) {
  const uuid = video.video_url.split('/').pop().replace(/\.mp4$/i, '')
  router.push(`/recommend/${uuid}`)
}

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
    const res = await listMyFavorites({ page: page.value, page_size: pageSize.value })
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
</script>

<template>
  <section class="list-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">我的收藏</p>
        <h1>收藏</h1>
        <p class="muted">共 {{ total }} 个收藏</p>
      </div>
    </div>

    <p v-if="error" class="error-text">{{ error }}</p>
    <p v-if="loading" class="muted">加载中...</p>

    <div v-else-if="videos.length" class="video-grid">
      <article v-for="v in videos" :key="v.id" class="video-card" style="cursor:pointer" @click="goToVideo(v)">
        <video class="card-video" :src="v.video_url" preload="metadata" @click.stop />
        <div class="card-body">
          <h3>{{ v.title }}</h3>
          <p>{{ v.description || '未填写简介' }}</p>
          <div class="card-meta">
            <span>浏览 {{ formatCount(v.view_count) }}</span>
            <span>点赞 {{ formatCount(v.like_count) }}</span>
            <span>收藏 {{ formatCount(v.favorite_count) }}</span>
          </div>
        </div>
      </article>
    </div>

    <div v-else class="empty-state">
      <h2>暂无收藏</h2>
      <p>去推荐页收藏喜欢的视频吧。</p>
    </div>

    <div class="pagination" v-if="totalPages() > 1">
      <button type="button" :disabled="page === 1" @click="changePage(page - 1)">上一页</button>
      <span>第 {{ page }} / {{ totalPages() }} 页</span>
      <button type="button" :disabled="page === totalPages()" @click="changePage(page + 1)">下一页</button>
    </div>
  </section>
</template>
