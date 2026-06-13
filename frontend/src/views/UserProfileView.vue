<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { getUserProfile } from '../api/videos'

const route = useRoute()
const profile = ref(null)
const videos = ref([])
const loading = ref(true)
const error = ref('')

onMounted(async () => {
  try {
    const res = await getUserProfile(route.params.id)
    profile.value = res.data.data.user
    videos.value = res.data.data.videos || []
  } catch (e) {
    error.value = '无法加载用户信息'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="list-page">
    <div v-if="loading" class="muted center">加载中…</div>
    <p v-else-if="error" class="error-text">{{ error }}</p>

    <template v-else-if="profile">
      <div class="profile-header">
        <div class="profile-avatar" aria-hidden="true">{{ profile.username[0].toUpperCase() }}</div>
        <div>
          <p class="eyebrow">创作者</p>
          <h1 class="profile-name">{{ profile.username }}</h1>
          <p class="muted">{{ videos.length }} 个视频</p>
        </div>
      </div>

      <div v-if="videos.length === 0" class="empty-state">
        <p class="muted">该用户暂无公开视频</p>
      </div>

      <div v-else class="video-grid">
        <article v-for="v in videos" :key="v.id" class="video-card">
          <video
            class="card-video"
            :src="v.video_url"
            muted
            preload="metadata"
            playsinline
          />
          <div class="card-body">
            <h3>{{ v.title }}</h3>
            <p v-if="v.description" class="muted">{{ v.description }}</p>
            <div class="card-meta">
              <span>♥ {{ v.like_count }}</span>
              <span>{{ v.created_at.slice(0, 10) }}</span>
            </div>
          </div>
        </article>
      </div>
    </template>
  </div>
</template>
