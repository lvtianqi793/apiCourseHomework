<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { listMyFavorites, listMyVideos } from '../api/videos'
import { useAuthStore } from '../stores/auth'
import { formatCount } from '../utils/format'

const router = useRouter()
const authStore = useAuthStore()
const myVideos = ref([])
const myFavorites = ref([])
const myTotal = ref(0)
const favTotal = ref(0)
const loading = ref(false)

const greeting = computed(() => {
  const hour = new Date().getHours()
  const name = authStore.user?.username || ''
  const prefix = hour < 12 ? '上午好' : hour < 18 ? '下午好' : '晚上好'
  return `${prefix}，${name}`
})

onMounted(async () => {
  loading.value = true
  try {
    const [worksRes, favsRes] = await Promise.all([
      listMyVideos({ page: 1, page_size: 3 }),
      listMyFavorites({ page: 1, page_size: 3 }),
    ])
    const works = worksRes.data.data
    myVideos.value = works.items
    myTotal.value = works.total

    const favs = favsRes.data.data
    myFavorites.value = favs.items
    favTotal.value = favs.total
  } catch (_) {}
  loading.value = false
})

function goToVideo(video) {
  const uuid = video.video_url.split('/').pop().replace(/\.mp4$/i, '')
  router.push(`/recommend/${uuid}`)
}
</script>

<template>
  <section class="list-page">
    <div class="page-heading">
      <div>
        <p class="eyebrow">个人中心</p>
        <h1>{{ greeting }}</h1>
      </div>
      <RouterLink class="primary-button small" to="/publish">发布作品</RouterLink>
    </div>

    <p v-if="loading" class="muted">加载中...</p>

    <template v-else>
      <!-- My Works section -->
      <div class="my-section">
        <div class="my-section-header">
          <h2>我的作品 <span class="muted" style="font-size:15px;font-weight:400">共 {{ myTotal }} 个</span></h2>
          <RouterLink class="primary-button small" to="/my/works">查看全部</RouterLink>
        </div>
        <div v-if="myVideos.length" class="video-grid">
          <article v-for="v in myVideos" :key="v.id" class="video-card" style="cursor:pointer" @click="goToVideo(v)">
            <video class="card-video" :src="v.video_url" preload="metadata" @click.stop />
            <div class="card-body">
              <h3>{{ v.title }}</h3>
              <p>{{ v.description || '未填写简介' }}</p>
              <div class="card-meta">
                <span>浏览 {{ formatCount(v.view_count) }}</span>
                <span>点赞 {{ formatCount(v.like_count) }}</span>
                <span>收藏 {{ formatCount(v.favorite_count) }}</span>
                <span>{{ v.created_at }}</span>
              </div>
            </div>
          </article>
        </div>
        <div v-else class="empty-state">
          <p>暂无作品，<RouterLink to="/publish">去发布</RouterLink></p>
        </div>
      </div>

      <!-- Favorites section -->
      <div class="my-section">
        <div class="my-section-header">
          <h2>收藏 <span class="muted" style="font-size:15px;font-weight:400">共 {{ favTotal }} 个</span></h2>
          <RouterLink class="primary-button small" to="/my/favorites">查看全部</RouterLink>
        </div>
        <div v-if="myFavorites.length" class="video-grid">
          <article v-for="v in myFavorites" :key="v.id" class="video-card" style="cursor:pointer" @click="goToVideo(v)">
            <video class="card-video" :src="v.video_url" preload="metadata" @click.stop />
            <div class="card-body">
              <h3>{{ v.title }}</h3>
              <p>{{ v.description || '未填写简介' }}</p>
              <div class="card-meta">
                <span>浏览 {{ formatCount(v.view_count) }}</span>
                <span>点赞 {{ formatCount(v.like_count) }}</span>
                <span>收藏 {{ formatCount(v.favorite_count) }}</span>
                <span>{{ v.created_at }}</span>
              </div>
            </div>
          </article>
        </div>
        <div v-else class="empty-state">
          <p>暂无收藏，去推荐页收藏喜欢的视频吧。</p>
        </div>
      </div>
    </template>
  </section>
</template>
