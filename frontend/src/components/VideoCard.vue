<script setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { deleteMyVideo, updateMyVideo } from '../api/videos'
import { formatCount } from '../utils/format'

const props = defineProps({
  video: { type: Object, required: true },
})

const emit = defineEmits(['deleted', 'updated', 'navigate'])

const menuOpen = ref(false)
const menuWrap = ref(null)

const deleteModalOpen = ref(false)
const deleting = ref(false)
const deleteError = ref('')

const editModalOpen = ref(false)
const saving = ref(false)
const editError = ref('')
const editTitle = ref('')
const editDescription = ref('')

function onWindowClick(e) {
  if (menuWrap.value && !menuWrap.value.contains(e.target)) {
    menuOpen.value = false
  }
}

onMounted(() => window.addEventListener('click', onWindowClick))
onUnmounted(() => window.removeEventListener('click', onWindowClick))

function openMenu() {
  menuOpen.value = !menuOpen.value
}

function closeMenu() {
  menuOpen.value = false
}

function openEdit() {
  closeMenu()
  editTitle.value = props.video.title
  editDescription.value = props.video.description || ''
  editError.value = ''
  editModalOpen.value = true
}

function openDelete() {
  closeMenu()
  deleteError.value = ''
  deleteModalOpen.value = true
}

async function confirmDelete() {
  deleting.value = true
  deleteError.value = ''
  try {
    await deleteMyVideo(props.video.id)
    deleteModalOpen.value = false
    emit('deleted', props.video.id)
  } catch (err) {
    deleteError.value = err.response?.data?.message || err.message
  } finally {
    deleting.value = false
  }
}

async function confirmEdit() {
  const title = editTitle.value.trim()
  if (!title) {
    editError.value = '标题不能为空'
    return
  }
  saving.value = true
  editError.value = ''
  try {
    const res = await updateMyVideo(props.video.id, {
      title,
      description: editDescription.value.trim() || null,
    })
    editModalOpen.value = false
    emit('updated', res.data.data)
  } catch (err) {
    editError.value = err.response?.data?.message || err.message
  } finally {
    saving.value = false
  }
}
</script>

<template>
  <article class="video-card" style="cursor:pointer" @click="emit('navigate', video)">
    <video class="card-video" :src="video.video_url" controls preload="metadata" @click.stop />

    <div class="card-body">
      <div class="card-title-row">
        <h3>{{ video.title }}</h3>
        <div ref="menuWrap" class="card-menu-wrap">
          <button class="card-menu-btn" type="button" @click.stop="openMenu" aria-label="更多操作">⋯</button>
          <div v-if="menuOpen" class="card-menu-dropdown">
            <button type="button" @click.stop="openEdit">编辑</button>
            <button type="button" class="danger" @click.stop="openDelete">删除</button>
          </div>
        </div>
      </div>
      <p>{{ video.description || '未填写简介' }}</p>
      <div class="card-meta">
        <span>浏览 {{ formatCount(video.view_count) }}</span>
        <span>点赞 {{ formatCount(video.like_count) }}</span>
        <span>收藏 {{ formatCount(video.favorite_count) }}</span>
        <span>{{ video.created_at }}</span>
      </div>
    </div>

    <Teleport to="body">
      <div v-if="deleteModalOpen" class="modal-backdrop" @click.self="deleteModalOpen = false">
        <div class="modal-box" role="dialog" aria-modal="true">
          <h2>确认删除</h2>
          <p>确认删除《{{ video.title }}》吗？本地视频文件也会被删除，此操作不可恢复。</p>
          <p v-if="deleteError" class="error-text">{{ deleteError }}</p>
          <div class="modal-actions">
            <button type="button" class="primary-button small" @click="deleteModalOpen = false" :disabled="deleting">取消</button>
            <button type="button" class="danger-button" @click="confirmDelete" :disabled="deleting">
              {{ deleting ? '删除中...' : '确认删除' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>

    <Teleport to="body">
      <div v-if="editModalOpen" class="modal-backdrop" @click.self="editModalOpen = false">
        <div class="modal-box" role="dialog" aria-modal="true">
          <h2>编辑作品</h2>
          <label>
            标题
            <input v-model="editTitle" maxlength="128" placeholder="视频标题" />
          </label>
          <label>
            简介
            <textarea v-model="editDescription" rows="3" maxlength="500" placeholder="视频简介（可选）" />
          </label>
          <p v-if="editError" class="error-text">{{ editError }}</p>
          <div class="modal-actions">
            <button type="button" class="primary-button small" @click="editModalOpen = false" :disabled="saving">取消</button>
            <button type="button" class="primary-button small" @click="confirmEdit" :disabled="saving">
              {{ saving ? '保存中...' : '保存' }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </article>
</template>
