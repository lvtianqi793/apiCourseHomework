# 简易视频流推荐系统

这是一个"简易版抖音"课程大作业项目，包含 Vue 前端、Spring Boot 后端和 MySQL 数据库。

## 技术栈

- 前端：Vue 3、Vite、Pinia、Vue Router、Axios
- 后端：Java 21、Spring Boot 3、Spring Web MVC、JdbcTemplate、JJWT、Argon2
- 数据库：MySQL
- 鉴权：JWT
- 视频存储：后端本地目录 `backend/uploads/videos`

---

## 原有功能

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

---

## 新增功能（第一次迭代）

### 视频侧边栏交互按钮

在推荐视频页面右侧新增仿抖音风格竖向操作栏，自上而下共五个按钮：

| 按钮 | 功能 |
| --- | --- |
| 头像 | 点击跳转到该视频发布者的主页 |
| 点赞 | 对当前视频点赞 / 取消点赞，实时更新数量 |
| 评论 | 展开 / 收起评论侧滑面板 |
| 收藏 | 将当前视频加入 / 取消收藏夹 |
| 分享 | 将当前页面链接复制到剪贴板 |

### 评论功能

- 任意用户可查看视频评论列表（按时间升序）
- 登录用户可发表评论（1–500 字符）
- 评论面板以侧滑动画展开，无需离开推荐页
- 评论数实时同步到侧边栏计数

### 收藏功能

- 登录用户可收藏 / 取消收藏任意视频
- 重复收藏幂等处理（`INSERT IGNORE`）
- 收藏状态在视频切换后从服务端实时读取

### 弹幕功能

- 发送弹幕：在当前视频播放时间点发送，内容 1–200 字符
- 渲染弹幕：视频播放时，对应时间戳附近的弹幕从右向左滚动飘过画面
- 全部登录用户可见；弹幕数据持久化存储
- 底部输入栏内嵌于视频播放器，回车或点击"发送"提交

### 发布者主页

- 路由：`/users/:id`
- 展示用户名、头像和其全部公开视频列表
- 从侧边栏头像按钮一键跳转

---

## 新增功能（第二次迭代）

### 收藏数与分享数展示

侧边栏"收藏"和"分享"按钮下方现在同样显示实时数字：

- **收藏数**：乐观更新，点击后立刻在本地 ±1，无需等待服务器响应
- **分享数**：点击"分享"将链接复制到剪贴板的同时，向服务端发送 `POST /api/videos/{id}/share`，服务端将 `share_count +1` 并返回最新值，前端随即更新

### "我的"页面重构

"我的"页面拆分为三级路由：

| 路由 | 说明 |
| --- | --- |
| `/my` | 个人中心（仪表盘），显示"我的作品"前 3 条和"收藏"前 3 条 |
| `/my/works` | 我的作品子页，完整分页列表，支持编辑与删除 |
| `/my/favorites` | 收藏子页，完整分页列表，只读展示 |

旧路由 `/my/videos` 自动重定向到 `/my`，不影响已有书签。

### 视频编辑功能

在"我的作品"子页的每张视频卡片右上角新增 ⋯ 下拉菜单，包含两个操作：

**编辑（编辑弹窗）**

- 可修改视频标题（必填，最多 128 字符）
- 可修改视频简介（可选，最多 500 字符）
- 点击空白区域或"取消"关闭弹窗
- 保存成功后本地卡片内容立即更新，无需刷新页面

**删除（删除确认弹窗）**

- 弹窗展示视频标题，明确提示操作不可恢复
- 确认后调用后端接口删除数据库记录及本地视频文件
- 删除成功后列表实时移除该卡片，页面总数同步减 1

---

## 新增功能（第三次迭代）

### 浏览量统计

- `videos` 表新增 `view_count BIGINT NOT NULL DEFAULT 0` 字段
- 每次用户进入视频观看页（包括通过推荐页切换、直接访问 UUID 路由），后端自动执行 `view_count + 1`，每次点击均计数，不去重

### 基于综合分数的视频推荐排序

推荐队列改为按综合分数降序排列，分数公式：

```
score = (view_count × 1 + like_count × 3 + favorite_count × 5)
        ÷ (自己发布的视频 ? 2 : 1)
```

- 浏览量权重最低，收藏权重最高，避免单纯刷量影响排序
- 对登录用户自己发布的视频分数减半，保证推荐多样性
- 循环队列中的上一个 / 下一个切换逻辑同步使用该分数排序，保证顺序一致

### "我的"页面数量展示优化

"我的"页面及所有子页（`/my`、`/my/works`、`/my/favorites`）的视频卡片统计栏由原来的点赞 / 收藏 / 转发改为：

| 原来 | 现在 |
| --- | --- |
| 点赞量 | 浏览量 |
| 收藏量 | 点赞量 |
| 转发量 | 收藏量 |

### 数量格式化（千 / 万）

全局引入 `formatCount` 工具函数，统一格式化所有数量展示：

| 数值范围 | 显示示例 |
| --- | --- |
| < 1000 | 原始数字，如 `999` |
| ≥ 1000，< 10000 | `xx千`（1 位小数四舍五入），如 `1451 → 1.5千` |
| ≥ 10000 | `xx万`（1 位小数四舍五入），如 `1234000 → 123.4万` |

### 推荐页 UUID 路由

- 推荐页路由调整为 `/recommend/:uuid`，支持直接通过视频 UUID（即文件名去掉 `.mp4` 后缀）访问指定视频
- 切换视频时 URL 自动同步为 `/recommend/<uuid>`，便于收藏和分享固定链接
- 未携带 UUID 时自动加载推荐队列首个视频，向后兼容旧链接

### 全局时间统一为 UTC+8

- 后端所有时间字段（视频发布时间、评论发布时间等）统一转换为 UTC+8（北京时间）后返回前端
- 涉及 `VideoDto` 和 `CommentDto` 的 `created_at` 字段，均在序列化前加 8 小时

### "我的"页面视频点击跳转

- `/my`（个人中心仪表盘）、`/my/works`（我的作品）、`/my/favorites`（收藏）三个页面的所有视频卡片均支持点击跳转
- 点击卡片后跳转至 `/recommend/<uuid>`，在推荐页直接播放对应视频
- 卡片内视频预览的原生控件点击不触发跳转（`@click.stop`），仅点击卡片其他区域跳转

---

## 新增功能（第四次迭代）

### 视频切换滑动动画

推荐页切换视频时新增上下滑入动画：

- 向下翻（下一个）：新视频从底部滑入，旧视频向上滑出
- 向上翻（上一个）：新视频从顶部滑入，旧视频向下滑出
- 使用 Vue `<Transition>` 动态名称（`slide-up` / `slide-down`），配合 `position: absolute; inset: 0` 实现两帧叠加的无缝切换
- 动画时长 380 ms，缓动曲线 `cubic-bezier(0.42, 0, 0.18, 1)`

### 搜索功能

上边栏搜索框现已完全可用，支持用户名和视频标题 / 简介的模糊匹配：

**前端**

- 在任意页面的顶部输入框键入关键词，按 Enter 或点击"⌕ 搜索"跳转至 `/search?q=...`
- 搜索结果页分为"用户"和"视频"两区
- 用户卡片点击跳转至 `/users/:id`
- 视频卡片点击跳转至 `/recommend/:uuid`，直接进入推荐播放页

**后端**

- 接口：`GET /api/search?q=<关键词>`
- 用户搜索：`WHERE username LIKE '%q%'`，按视频数量降序，最多返回 10 条
- 视频搜索：`WHERE title LIKE '%q%' OR description LIKE '%q%'`，按综合分数（`like_count×3 + view_count + favorite_count×5`）降序，最多返回 20 条
- 关键词为空时直接返回两个空数组，不查库

### "我的"页面时段问候

"我的"页面的标题由固定"我的"改为根据本地时间动态显示：

| 时间范围 | 显示文字 |
| --- | --- |
| 00:00 – 11:59 | 上午好，`用户名` |
| 12:00 – 17:59 | 下午好，`用户名` |
| 18:00 – 23:59 | 晚上好，`用户名` |

---

## 数据库变更

| 迁移文件 | 变更内容 |
| --- | --- |
| `backend/migrations/001_init.sql` | 初始化：users / videos / video_likes / video_views / request_logs，并条件性添加 share_count、view_count |
| `backend/migrations/002_add_view_share_count.sql` | 为旧库补充：`videos.share_count`、`videos.view_count` |
| `backend/migrations/003_social_tables.sql` | 新增：video_comments / video_favorites / video_danmaku |

---

## 目录结构

```text
code/
  backend/
    pom.xml
    .env.example
    migrations/
      001_init.sql
      002_add_view_share_count.sql
      003_social_tables.sql
    uploads/videos/
    src/main/java/com/videoflow/
      VideoFlowApplication.java
      config/
        AppProperties.java
        DotEnvLoader.java
        WebConfig.java
      controller/
        AuthController.java
        VideoController.java
        MyController.java
        UserController.java
        SearchController.java
      service/
        AuthService.java
        VideoService.java
      security/
        JwtService.java
        PasswordService.java
      filter/
        JwtAuthFilter.java
        RequestLoggingFilter.java
      dto/
        VideoDto.java
        CommentDto.java
        DanmakuDto.java
        AuthDtos.java
      mapper/
        RowMappers.java
      exception/
        AppException.java
        GlobalExceptionHandler.java
      util/
        ApiResponse.java
    src/main/resources/
      application.yml
  frontend/
    src/
      api/
        auth.js
        videos.js
        http.js
      router/
        index.js
      utils/
        format.js
      views/
        MyVideosView.vue
        MyWorksPage.vue
        MyFavoritesPage.vue
        RecommendView.vue
        SearchView.vue
        PublishVideoView.vue
        LoginView.vue
        RegisterView.vue
        UserProfileView.vue
      components/
        VideoCard.vue
        VideoPlayer.vue
        CommentPanel.vue
        LikeButton.vue
      styles.css
```

---

## 数据库初始化

1. 启动 MySQL。
2. 依次执行迁移文件：

```bash
mysql -u root -p < backend/migrations/001_init.sql
mysql -u root -p < backend/migrations/002_add_view_share_count.sql
mysql -u root -p < backend/migrations/003_social_tables.sql
```

> 若是全新安装，执行 `001_init.sql` 后通常已包含全部表结构；`002`、`003` 用于兼容旧版数据库补全字段和表。

3. 复制后端环境变量：

```bash
cd backend
cp .env.example .env
```

4. 修改 `backend/.env` 中的数据库配置：

```env
DATABASE_URL=jdbc:mysql://localhost:3306/video_flow?useUnicode=true&characterEncoding=utf8&serverTimezone=Asia/Shanghai
DATABASE_USERNAME=root
DATABASE_PASSWORD=你的密码
JWT_SECRET=video-flow-dev-secret-key-2026
SERVER_PORT=8080
SERVER_ADDR=0.0.0.0
FRONTEND_ORIGIN=http://localhost:5173
PUBLIC_BASE_URL=http://localhost:8080
UPLOAD_DIR=uploads/videos
MAX_VIDEO_SIZE_MB=500
```

## 启动后端

前置条件：Java 21、Maven。

```bash
cd backend
mvn spring-boot:run
```

后端启动时会自动读取 `backend/.env` 中的环境变量。

后端启动后：

- API 地址：`http://localhost:8080/api`
- 视频静态访问地址：`http://localhost:8080/uploads/videos/...`

## 启动前端

```bash
cd frontend
npm install
cp .env.example .env
npm run dev
```

访问：`http://localhost:5173`

---

## API 摘要

### 原有接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| POST | `/api/auth/register` | 注册 |
| POST | `/api/auth/login` | 登录 |
| GET | `/api/videos/recommend/next?current_id=1` | 获取下一个推荐视频 |
| GET | `/api/videos/recommend/prev?current_id=1` | 获取上一个推荐视频 |
| POST | `/api/videos/{id}/like` | 点赞 |
| DELETE | `/api/videos/{id}/like` | 取消点赞 |
| POST | `/api/my/videos` | 发布视频 |
| GET | `/api/my/videos?page=1&page_size=10` | 分页查看我的视频 |
| DELETE | `/api/my/videos/{id}` | 删除我的视频 |

### 第一次迭代新增接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/api/videos/{id}/comments` | 获取视频评论列表 |
| POST | `/api/videos/{id}/comments` | 发表评论 |
| POST | `/api/videos/{id}/favorite` | 收藏视频 |
| DELETE | `/api/videos/{id}/favorite` | 取消收藏 |
| GET | `/api/videos/{id}/danmaku` | 获取弹幕列表 |
| POST | `/api/videos/{id}/danmaku` | 发送弹幕 |
| GET | `/api/users/{id}` | 获取用户主页（用户信息 + 视频列表） |

### 第二次迭代新增接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| PATCH | `/api/my/videos/{id}` | 编辑我的视频（标题 / 简介） |
| GET | `/api/my/favorites?page=1&page_size=9` | 分页获取我的收藏列表 |
| POST | `/api/videos/{id}/share` | 分享计数 +1，返回最新 share_count |

### 第三次迭代新增接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/api/videos/by-uuid/:uuid` | 按 UUID 获取视频 |
| POST | `/api/videos/:id/view` | 浏览量 +1（已内嵌于 recommend/by-uuid） |

### 第四次迭代新增接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/api/search?q=关键词` | 搜索用户（最多 10 条）和视频（最多 20 条），按相关性排序 |

---

## 演示建议

1. 注册两个用户，分别上传多个 mp4 视频。
2. 点赞其中几个视频，观察推荐页首个视频按点赞数最高优先展示。
3. 连续向下切换，视频会按点赞排序循环播放，不会刷到空。
4. 点击评论按钮，在侧滑面板中发表评论，观察评论数实时更新。
5. 点击收藏按钮，收藏当前视频，观察收藏数 +1；再次点击取消收藏，数量 -1。
6. 点击分享按钮，链接复制到剪贴板，同时分享数 +1。
7. 播放视频时在底部输入框发送弹幕，观察弹幕从右向左飘过画面。
8. 点击侧边栏头像按钮，跳转到该视频发布者的主页，查看其全部视频。
9. 进入"我的"页面，查看作品和收藏各 3 条预览，点击"查看全部"进入子页。
10. 在"我的作品"子页，点击视频卡片右上角 ⋯，选择"编辑"修改标题和简介，保存后立即生效。
11. 选择"删除"，确认弹窗后视频从列表中消失，后端文件同步删除。
12. 在推荐页上下滑动切换视频，观察滑入动画效果。
13. 在顶部搜索框输入关键词，回车或点击搜索按钮，跳转至搜索结果页。
14. 点击搜索结果中的用户卡片跳转至用户主页，点击视频卡片跳转至推荐播放页。
15. 进入"我的"页面，观察标题根据当前时间显示"上午好 / 下午好 / 晚上好，用户名"。
