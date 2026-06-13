<script setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import CommentPanel from '../components/CommentPanel.vue'
import VideoPlayer from '../components/VideoPlayer.vue'
import {
  favoriteVideo,
  getNextVideo,
  getPrevVideo,
  getVideoByUuid,
  incrementShare,
  likeVideo,
  listDanmaku,
  sendDanmaku,
  unfavoriteVideo,
  unlikeVideo,
} from '../api/videos'
import { formatCount } from '../utils/format'

const route = useRoute()
const router = useRouter()
const currentVideo = ref(null)
const loading = ref(false)
const liking = ref(false)
const favoriting = ref(false)
const message = ref('')
const commentPanelOpen = ref(false)
const touchStartY = ref(0)
let lastSwitchTime = 0
const slideDirection = ref('')  // 'up' | 'down' | ''

// danmaku
const playerRef = ref(null)
const danmakuList = ref([])
const activeDanmaku = ref([])
const danmakuInput = ref('')
const sendingDanmaku = ref(false)
const firedIds = new Set()

onMounted(() => {
  const uuid = route.params.uuid
  if (uuid) {
    loadByUuid(uuid)
  } else {
    loadNext()
  }
})

onUnmounted(() => {
  getVideoEl()?.removeEventListener('timeupdate', onTimeUpdate)
})

function getVideoEl() {
  return playerRef.value?.videoEl
}

// ── danmaku logic ─────────────────────────────────────────────────────────────

async function loadDanmaku(videoId) {
  firedIds.clear()
  danmakuList.value = []
  activeDanmaku.value = []
  try {
    const res = await listDanmaku(videoId)
    danmakuList.value = res.data.data || []
  } catch (_) {}
  const el = getVideoEl()
  if (el) {
    el.removeEventListener('timeupdate', onTimeUpdate)
    el.addEventListener('timeupdate', onTimeUpdate)
  }
}

function onTimeUpdate() {
  const ct = getVideoEl()?.currentTime ?? 0
  for (const d of danmakuList.value) {
    if (!firedIds.has(d.id) && ct >= d.timestamp_sec && ct < d.timestamp_sec + 1.0) {
      firedIds.add(d.id)
      spawnDanmaku(d)
    }
  }
}

function spawnDanmaku(d) {
  const key = `${d.id}-${performance.now()}`
  const track = Math.floor(Math.random() * 7)
  activeDanmaku.value.push({ id: d.id, content: d.content, track, key })
}

function removeDanmaku(key) {
  activeDanmaku.value = activeDanmaku.value.filter(a => a.key !== key)
}

async function submitDanmaku() {
  const content = danmakuInput.value.trim()
  if (!content || sendingDanmaku.value || !currentVideo.value) return
  const ts = getVideoEl()?.currentTime ?? 0
  sendingDanmaku.value = true
  try {
    const res = await sendDanmaku(currentVideo.value.id, content, ts)
    const d = res.data.data
    danmakuList.value.push(d)
    // spawn immediately so the sender sees it right away
    firedIds.add(d.id)
    spawnDanmaku(d)
    danmakuInput.value = ''
  } catch (_) {}
  sendingDanmaku.value = false
}

function handleDanmakuKey(e) {
  if (e.key === 'Enter') submitDanmaku()
}

// ── video navigation ──────────────────────────────────────────────────────────

function syncUrl(video) {
  const filename = video.video_url.split('/').pop()
  const uuid = filename.replace(/\.mp4$/i, '')
  router.replace(`/recommend/${uuid}`)
}

async function loadByUuid(uuid) {
  if (loading.value) return
  loading.value = true
  message.value = ''
  commentPanelOpen.value = false
  try {
    const response = await getVideoByUuid(uuid)
    if (response.data.data) {
      currentVideo.value = response.data.data
      syncUrl(currentVideo.value)
      await loadDanmaku(currentVideo.value.id)
    } else {
      message.value = response.data.message || '视频不存在'
    }
  } catch (err) {
    message.value = err.message
  } finally {
    loading.value = false
  }
}

async function loadNext() {
  if (loading.value) return
  loading.value = true
  slideDirection.value = 'up'
  message.value = ''
  commentPanelOpen.value = false
  try {
    const response = await getNextVideo(currentVideo.value?.id)
    if (response.data.data) {
      currentVideo.value = response.data.data
      syncUrl(currentVideo.value)
      await loadDanmaku(currentVideo.value.id)
    } else {
      message.value = response.data.message || '暂无可播放内容'
    }
  } catch (err) {
    message.value = err.message
  } finally {
    loading.value = false
  }
}

async function loadPrev() {
  if (loading.value || !currentVideo.value) return
  loading.value = true
  slideDirection.value = 'down'
  message.value = ''
  commentPanelOpen.value = false
  try {
    const response = await getPrevVideo(currentVideo.value.id)
    if (response.data.data) {
      currentVideo.value = response.data.data
      syncUrl(currentVideo.value)
      await loadDanmaku(currentVideo.value.id)
    } else {
      message.value = response.data.message || '暂无可播放内容'
    }
  } catch (err) {
    message.value = err.message
  } finally {
    loading.value = false
  }
}

async function toggleLike() {
  if (!currentVideo.value || liking.value) return
  liking.value = true
  message.value = ''
  try {
    const response = currentVideo.value.liked
      ? await unlikeVideo(currentVideo.value.id)
      : await likeVideo(currentVideo.value.id)
    const result = response.data.data
    currentVideo.value = { ...currentVideo.value, liked: result.liked, like_count: result.like_count }
  } catch (err) {
    message.value = err.message
  } finally {
    liking.value = false
  }
}

async function toggleFavorite() {
  if (!currentVideo.value || favoriting.value) return
  favoriting.value = true
  try {
    const response = currentVideo.value.favorited
      ? await unfavoriteVideo(currentVideo.value.id)
      : await favoriteVideo(currentVideo.value.id)
    const delta = response.data.data.favorited ? 1 : -1
    currentVideo.value = {
      ...currentVideo.value,
      favorited: response.data.data.favorited,
      favorite_count: currentVideo.value.favorite_count + delta,
    }
  } catch (err) {
    message.value = err.message
  } finally {
    favoriting.value = false
  }
}

function toggleCommentPanel() {
  commentPanelOpen.value = !commentPanelOpen.value
}

function goToProfile() {
  if (currentVideo.value?.user_id) {
    router.push(`/users/${currentVideo.value.user_id}`)
  }
}

async function shareVideo() {
  const filename = currentVideo.value.video_url.split('/').pop()
  const uuid = filename.replace(/\.mp4$/i, '')
  const url = `${window.location.origin}/recommend/${uuid}`
  try {
    await navigator.clipboard.writeText(url)
    message.value = '链接已复制到剪贴板'
  } catch (_) {
    message.value = '复制失败，请手动复制地址栏'
  }
  setTimeout(() => { message.value = '' }, 2000)
  if (currentVideo.value) {
    try {
      const res = await incrementShare(currentVideo.value.id)
      currentVideo.value = { ...currentVideo.value, share_count: res.data.data.share_count }
    } catch (_) {}
  }
}

function onCommentCountChange(count) {
  if (currentVideo.value) {
    currentVideo.value = { ...currentVideo.value, comment_count: count }
  }
}

function switchByDirection(direction) {
  const now = Date.now()
  if (now - lastSwitchTime < 650) return
  lastSwitchTime = now
  if (direction === 'next') loadNext()
  else loadPrev()
}

function handleWheel(event) {
  if (Math.abs(event.deltaY) < 20) return
  switchByDirection(event.deltaY > 0 ? 'next' : 'prev')
}

function handleTouchStart(event) {
  touchStartY.value = event.touches[0]?.clientY || 0
}

function handleTouchEnd(event) {
  const endY = event.changedTouches[0]?.clientY || 0
  const diff = touchStartY.value - endY
  if (Math.abs(diff) < 50) return
  switchByDirection(diff > 0 ? 'next' : 'prev')
}
</script>

<template>
  <section
    class="recommend-page"
    @wheel.prevent="handleWheel"
    @touchstart="handleTouchStart"
    @touchend="handleTouchEnd"
  >
    <!-- video info overlay (bottom-left of player area) -->
    <aside class="video-overlay" v-if="currentVideo">
      <p class="eyebrow">@{{ currentVideo.author }}</p>
      <h1>{{ currentVideo.title }}</h1>
      <p v-if="currentVideo.description">{{ currentVideo.description }}</p>
    </aside>

    <div class="video-stage">
      <Transition :name="`slide-${slideDirection}`">
        <VideoPlayer
          :key="currentVideo?.id ?? 'empty'"
          ref="playerRef"
          :video="currentVideo"
          :danmaku-items="activeDanmaku"
          @danmaku-ended="removeDanmaku"
        />
      </Transition>
    </div>

    <!-- danmaku input bar — BELOW the video, not inside it -->
    <div class="danmaku-bar" v-if="currentVideo">
      <input
        v-model="danmakuInput"
        class="danmaku-input"
        placeholder="发送弹幕…"
        maxlength="200"
        :disabled="sendingDanmaku"
        @keydown="handleDanmakuKey"
        aria-label="弹幕内容"
      />
      <button
        class="danmaku-send"
        :disabled="sendingDanmaku || !danmakuInput.trim()"
        @click="submitDanmaku"
      >发送</button>
      <div class="feed-tip-inline">
        <span v-if="loading">正在加载</span>
        <span v-else-if="message">{{ message }}</span>
        <span v-else>上下滑动切换</span>
      </div>
    </div>

    <!-- TikTok-style sidebar -->
    <div class="side-actions" v-if="currentVideo">
      <button class="action-btn avatar-btn" type="button" :title="`查看 ${currentVideo.author} 的主页`" @click="goToProfile" aria-label="查看发布者主页">
        <span class="action-avatar">{{ currentVideo.author[0].toUpperCase() }}</span>
      </button>
      <button class="action-btn" :class="{ 'action-btn--active': currentVideo.liked }" type="button" :disabled="liking" @click="toggleLike" aria-label="点赞">
        <span class="action-icon">♥</span>
        <span class="action-label">{{ formatCount(currentVideo.like_count) }}</span>
      </button>
      <button class="action-btn" :class="{ 'action-btn--active': commentPanelOpen }" type="button" @click="toggleCommentPanel" aria-label="评论">
        <span class="action-icon">💬</span>
        <span class="action-label">{{ currentVideo.comment_count }}</span>
      </button>
      <button class="action-btn" :class="{ 'action-btn--active': currentVideo.favorited }" type="button" :disabled="favoriting" @click="toggleFavorite" aria-label="收藏">
        <span class="action-icon">★</span>
        <span class="action-label">{{ formatCount(currentVideo.favorite_count) }}</span>
      </button>
      <button class="action-btn" type="button" @click="shareVideo" aria-label="分享视频">
        <span class="action-icon">↗</span>
        <span class="action-label">{{ formatCount(currentVideo.share_count) }}</span>
      </button>
      <button class="round-button" type="button" :disabled="loading" @click="loadPrev" aria-label="上一个视频">⌃</button>
      <button class="round-button" type="button" :disabled="loading" @click="loadNext" aria-label="下一个视频">⌄</button>
    </div>

    <CommentPanel
      :video="currentVideo"
      :visible="commentPanelOpen"
      @close="commentPanelOpen = false"
      @comment-count-change="onCommentCountChange"
    />
  </section>
</template>
