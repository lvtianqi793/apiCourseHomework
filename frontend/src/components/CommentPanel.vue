<script setup>
import { ref, watch } from 'vue'
import { listComments, postComment } from '../api/videos'

const props = defineProps({
  video: { type: Object, default: null },
  visible: { type: Boolean, default: false }
})

const emit = defineEmits(['close', 'comment-count-change'])

const comments = ref([])
const input = ref('')
const sending = ref(false)
const loading = ref(false)
const error = ref('')

watch(() => [props.video?.id, props.visible], async ([id, vis]) => {
  if (!id || !vis) return
  loading.value = true
  error.value = ''
  try {
    const res = await listComments(id)
    comments.value = res.data.data || []
  } catch (e) {
    error.value = '加载失败'
  } finally {
    loading.value = false
  }
}, { immediate: true })

async function submit() {
  const content = input.value.trim()
  if (!content || sending.value || !props.video) return
  sending.value = true
  error.value = ''
  try {
    const res = await postComment(props.video.id, content)
    comments.value.push(res.data.data)
    input.value = ''
    emit('comment-count-change', comments.value.length)
  } catch (e) {
    error.value = '发送失败，请稍后重试'
  } finally {
    sending.value = false
  }
}

function handleKey(e) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    submit()
  }
}
</script>

<template>
  <transition name="panel-slide">
    <aside v-if="visible" class="comment-panel" role="dialog" aria-label="评论">
      <div class="comment-panel-header">
        <span>评论 {{ comments.length > 0 ? `(${comments.length})` : '' }}</span>
        <button class="panel-close" @click="$emit('close')" aria-label="关闭评论">✕</button>
      </div>

      <div class="comment-list" role="list">
        <div v-if="loading" class="comment-loading">加载中…</div>
        <div v-else-if="comments.length === 0" class="comment-empty">暂无评论，来说点什么吧</div>
        <div
          v-for="c in comments"
          :key="c.id"
          class="comment-item"
          role="listitem"
        >
          <span class="comment-author">{{ c.author }}</span>
          <p class="comment-content">{{ c.content }}</p>
          <time class="comment-time">{{ c.created_at }}</time>
        </div>
      </div>

      <div class="comment-compose">
        <p v-if="error" class="error-text" role="alert">{{ error }}</p>
        <div class="comment-compose-row">
          <textarea
            v-model="input"
            class="comment-textarea"
            placeholder="写下你的评论…"
            maxlength="500"
            rows="2"
            :disabled="sending"
            @keydown="handleKey"
            aria-label="评论内容"
          />
          <button
            class="primary-button small"
            :disabled="sending || !input.trim()"
            @click="submit"
          >{{ sending ? '发送中' : '发送' }}</button>
        </div>
      </div>
    </aside>
  </transition>
</template>
