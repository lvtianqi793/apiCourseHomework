<script setup>
import { ref } from 'vue'

const props = defineProps({
  video: { type: Object, default: null },
  danmakuItems: { type: Array, default: () => [] }
})

const emit = defineEmits(['danmaku-ended'])
const videoEl = ref(null)
defineExpose({ videoEl })
</script>

<template>
  <section class="video-player">
    <video
      v-if="video"
      ref="videoEl"
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

    <div v-if="video" class="danmaku-overlay" aria-hidden="true">
      <span
        v-for="d in danmakuItems"
        :key="d.key"
        class="danmaku-item"
        :style="{ top: `${d.track * 13 + 4}%` }"
        @animationend="emit('danmaku-ended', d.key)"
      >{{ d.content }}</span>
    </div>
  </section>
</template>
