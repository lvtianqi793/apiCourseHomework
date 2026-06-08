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
    headers: { 'Content-Type': 'multipart/form-data' },
    onUploadProgress
  })
}

export function listMyVideos(params) {
  return http.get('/my/videos', { params })
}

export function deleteMyVideo(videoId) {
  return http.delete(`/my/videos/${videoId}`)
}
