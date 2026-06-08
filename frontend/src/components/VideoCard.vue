<script setup>
defineProps({
  video: {
    type: Object,
    required: true
  },
  deleting: {
    type: Boolean,
    default: false
  }
})

defineEmits(['delete'])
</script>

<template>
  <article class="video-card">
    <video class="card-video" :src="video.video_url" controls preload="metadata" />

    <div class="card-body">
      <h3>{{ video.title }}</h3>
      <p>{{ video.description || '未填写简介' }}</p>
      <div class="card-meta">
        <span>点赞 {{ video.like_count }}</span>
        <span>{{ video.created_at }}</span>
      </div>
      <button class="danger-button" type="button" :disabled="deleting" @click="$emit('delete')">
        {{ deleting ? '删除中...' : '删除作品' }}
      </button>
    </div>
  </article>
</template>
