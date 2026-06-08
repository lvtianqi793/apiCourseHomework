<script setup>
import { onMounted, ref } from 'vue'

import LikeButton from '../components/LikeButton.vue'
import VideoPlayer from '../components/VideoPlayer.vue'
import { getNextVideo, getPrevVideo, likeVideo, unlikeVideo } from '../api/videos'

const currentVideo = ref(null)
const loading = ref(false)
const liking = ref(false)
const message = ref('')
const touchStartY = ref(0)
let lastSwitchTime = 0

onMounted(() => {
  loadNext()
})

async function loadNext() {
  if (loading.value) return

  loading.value = true
  message.value = ''

  try {
    const response = await getNextVideo(currentVideo.value?.id)
    if (response.data.data) {
      currentVideo.value = response.data.data
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
  message.value = ''

  try {
    const response = await getPrevVideo(currentVideo.value.id)
    if (response.data.data) {
      currentVideo.value = response.data.data
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
    currentVideo.value = {
      ...currentVideo.value,
      liked: result.liked,
      like_count: result.like_count
    }
  } catch (err) {
    message.value = err.message
  } finally {
    liking.value = false
  }
}

function switchByDirection(direction) {
  const now = Date.now()
  if (now - lastSwitchTime < 650) return

  lastSwitchTime = now
  if (direction === 'next') {
    loadNext()
  } else {
    loadPrev()
  }
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
    <VideoPlayer :video="currentVideo" />

    <aside class="video-overlay" v-if="currentVideo">
      <p class="eyebrow">@{{ currentVideo.author }}</p>
      <h1>{{ currentVideo.title }}</h1>
      <p v-if="currentVideo.description">{{ currentVideo.description }}</p>
    </aside>

    <div class="side-actions" v-if="currentVideo">
      <button class="round-button" type="button" :disabled="loading" @click="loadPrev">⌃</button>
      <LikeButton
        :liked="currentVideo.liked"
        :count="currentVideo.like_count"
        :disabled="liking"
        @toggle="toggleLike"
      />
      <button class="round-button" type="button" :disabled="loading" @click="loadNext">⌄</button>
    </div>

    <div class="feed-tip">
      <span v-if="loading">正在加载</span>
      <span v-else-if="message">{{ message }}</span>
      <span v-else>上下滑动切换内容</span>
    </div>
  </section>
</template>
