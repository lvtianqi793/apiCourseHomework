<script setup>
defineProps({
  video: {
    type: Object,
    default: null
  }
})
</script>

<template>
  <section class="video-player">
    <video
      v-if="video"
      :key="video.id"
      class="video-element"
      :src="video.video_url"
      autoplay
      controls
      loop
      playsinline
    />

    <div v-else class="empty-video">
      当前暂无可播放内容
    </div>
  </section>
</template>
