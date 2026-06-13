<script setup>
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { searchContent } from '../api/videos'
import { formatCount } from '../utils/format'

const route = useRoute()
const router = useRouter()
const query = ref(route.query.q || '')
const users = ref([])
const videos = ref([])
const loading = ref(false)

async function doSearch(q) {
  if (!q) { users.value = []; videos.value = []; return }
  loading.value = true
  try {
    const res = await searchContent(q)
    const data = res.data.data
    users.value = data.users || []
    videos.value = data.videos || []
  } catch (_) {}
  loading.value = false
}

watch(() => route.query.q, (q) => {
  query.value = q || ''
  doSearch(query.value)
}, { immediate: true })

function goToUser(user) {
  router.push(`/users/${user.id}`)
}

function goToVideo(video) {
  const uuid = video.video_url.split('/').pop().replace(/\.mp4$/i, '')
  router.push(`/recommend/${uuid}`)
}
</script>

<template>
  <section class="list-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">搜索结果</p>
        <h1>{{ query || '…' }}</h1>
      </div>
    </div>

    <p v-if="loading" class="muted">搜索中...</p>

    <template v-else>
      <!-- Users -->
      <div v-if="users.length" class="my-section">
        <div class="my-section-header">
          <h2>用户</h2>
        </div>
        <div class="search-user-list">
          <button
            v-for="u in users"
            :key="u.id"
            class="search-user-card"
            type="button"
            @click="goToUser(u)"
          >
            <span class="search-user-avatar">{{ u.username.slice(0, 1).toUpperCase() }}</span>
            <div class="search-user-info">
              <strong>{{ u.username }}</strong>
              <span class="muted">{{ u.video_count }} 个视频</span>
            </div>
          </button>
        </div>
      </div>

      <!-- Videos -->
      <div v-if="videos.length" class="my-section">
        <div class="my-section-header">
          <h2>视频</h2>
        </div>
        <div class="video-grid">
          <article
            v-for="v in videos"
            :key="v.id"
            class="video-card"
            style="cursor:pointer"
            @click="goToVideo(v)"
          >
            <video class="card-video" :src="v.video_url" preload="metadata" @click.stop />
            <div class="card-body">
              <h3>{{ v.title }}</h3>
              <p class="muted">@{{ v.author }}</p>
              <p v-if="v.description">{{ v.description }}</p>
              <div class="card-meta">
                <span>浏览 {{ formatCount(v.view_count) }}</span>
                <span>点赞 {{ formatCount(v.like_count) }}</span>
                <span>收藏 {{ formatCount(v.favorite_count) }}</span>
              </div>
            </div>
          </article>
        </div>
      </div>

      <div v-if="!users.length && !videos.length && query" class="empty-state" style="margin-top:48px">
        <p>没有找到与"{{ query }}"相关的用户或视频。</p>
      </div>
    </template>
  </section>
</template>
