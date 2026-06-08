# 简易视频流推荐系统

这是一个“简易版抖音”课程大作业项目，包含 Vue 前端、Rust 轻量后端和 MySQL 数据库。

## 技术栈

- 前端：Vue 3、Vite、Pinia、Vue Router、Axios
- 后端：Rust、Axum、SQLx、Tokio
- 数据库：MySQL
- 鉴权：JWT
- 视频存储：后端本地目录 `backend/uploads/videos`

## 已实现功能

### 用户功能

- 用户注册
- 用户登录
- 用户退出登录
- JWT 登录态校验

### 推荐视频主页

- 按点赞数从高到低推荐视频
- 推荐视频按点赞数形成循环队列，刷完后自动从头继续
- 鼠标滚轮或移动端上下滑动切换视频
- 查看循环队列中的上一个视频
- 查看循环队列中的下一个视频
- 视频点赞和取消点赞

### 我的视频管理

- 发布 mp4 视频
- 单个视频最大 500MB
- 分页查看我的视频
- 删除我的视频
- 删除时校验视频归属权限
- 删除数据库记录并真实删除本地视频文件

### 日志与监控

- 记录每个 API 请求的用户 ID、方法、路径、请求体、响应体、状态码、耗时
- 日志表：`request_logs`
- 自动脱敏 `password` 和 `token`
- 上传视频接口不记录 multipart 文件内容，只记录占位文本

## 目录结构

```text
code/
  backend/
    migrations/001_init.sql
    src/
      main.rs
      config.rs
      db.rs
      error.rs
      middleware/logging.rs
      modules/auth/
      modules/videos/
      utils/
    uploads/videos/
  frontend/
    src/
      api/
      router/
      stores/
      views/
      components/
```

## 数据库初始化

1. 启动 MySQL。
2. 执行初始化 SQL：

```bash
mysql -u root -p < backend/migrations/001_init.sql
```

3. 复制后端环境变量：

```bash
cd backend
cp .env.example .env
```

4. 修改 `backend/.env` 中的数据库密码：

```env
DATABASE_URL=mysql://root:password@localhost:3306/video_flow
JWT_SECRET=please-change-this-secret
SERVER_ADDR=0.0.0.0:8080
FRONTEND_ORIGIN=http://localhost:5173
PUBLIC_BASE_URL=http://localhost:8080
UPLOAD_DIR=uploads/videos
MAX_VIDEO_SIZE_MB=500
```

## 启动后端

当前后端端口为 `8080`。

```bash
cd backend
cargo run
```

后端启动后：

- API 地址：`http://localhost:8080/api`
- 视频静态访问地址：`http://localhost:8080/uploads/videos/...`

## 启动前端

当前前端端口为 `5173`。

```bash
cd frontend
npm install
npm run dev
```

访问：

```text
http://localhost:5173
```

## API 摘要

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| POST | `/api/auth/register` | 注册 |
| POST | `/api/auth/login` | 登录 |
| GET | `/api/videos/recommend/next?current_id=1` | 获取循环队列中的下一个推荐视频 |
| GET | `/api/videos/recommend/prev?current_id=1` | 获取循环队列中的上一个推荐视频 |
| POST | `/api/videos/{id}/like` | 点赞 |
| DELETE | `/api/videos/{id}/like` | 取消点赞 |
| POST | `/api/my/videos` | 发布视频 |
| GET | `/api/my/videos?page=1&page_size=10` | 分页查看我的视频 |
| DELETE | `/api/my/videos/{id}` | 删除我的视频 |

## 演示建议

1. 注册两个用户。
2. 分别上传多个 mp4 视频。
3. 点赞其中几个视频。
4. 重新进入推荐页，观察首个视频按点赞数最高优先展示。
5. 连续向下切换，视频会按照点赞排序循环播放，不会刷到空。
6. 在“我的视频”中删除视频，确认数据库记录和本地文件都被删除。
