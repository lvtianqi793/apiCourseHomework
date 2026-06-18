import http from './http'

export function getNextVideo(currentId) {
  return http.get('/videos/recommend/next', {
    params: currentId ? { current_id: currentId } : {}
  })
}

export function getPrevVideo(currentId) {
  return http.get('/videos/recommend/prev', {
    params: currentId ? { current_id: currentId } : {}
  })
}

export function likeVideo(videoId) {
  return http.post(`/videos/${videoId}/like`)
}

export function unlikeVideo(videoId) {
  return http.delete(`/videos/${videoId}/like`)
}

export function publishVideo(formData, onUploadProgress) {
  return http.post('/my/videos', formData, {
    timeout: 0,
    onUploadProgress
  })
}

export function listMyVideos(params) {
  return http.get('/my/videos', { params })
}

export function deleteMyVideo(videoId) {
  return http.delete(`/my/videos/${videoId}`)
}

export function listComments(videoId) {
  return http.get(`/videos/${videoId}/comments`)
}

export function postComment(videoId, content) {
  return http.post(`/videos/${videoId}/comments`, { content })
}

export function favoriteVideo(videoId) {
  return http.post(`/videos/${videoId}/favorite`)
}

export function unfavoriteVideo(videoId) {
  return http.delete(`/videos/${videoId}/favorite`)
}

export function listDanmaku(videoId) {
  return http.get(`/videos/${videoId}/danmaku`)
}

export function sendDanmaku(videoId, content, timestamp_sec) {
  return http.post(`/videos/${videoId}/danmaku`, { content, timestamp_sec })
}

export function getUserProfile(userId) {
  return http.get(`/users/${userId}`)
}

export function updateMyVideo(videoId, data) {
  return http.patch(`/my/videos/${videoId}`, data)
}

export function listMyFavorites(params) {
  return http.get('/my/favorites', { params })
}

export function incrementShare(videoId) {
  return http.post(`/videos/${videoId}/share`)
}

export function getVideoByUuid(uuid) {
  return http.get(`/videos/by-uuid/${uuid}`)
}

export function searchContent(q) {
  return http.get('/search', { params: { q } })
}
