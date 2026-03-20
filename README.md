# TinyEngine Rust 后端

TinyEngine 低代码平台的 Rust 实现后端。

## 功能特性

- 应用管理（增删改查、Schema、发布）
- 页面管理（含历史版本）
- 区块管理及分类
- Schema 生成代码
- 国际化（i18n）支持
- RESTful API（兼容 TinyEngine 前端）

## 技术栈

- **Web 框架**: Axum 0.7
- **数据库**: MySQL/MariaDB + SQLx
- **异步运行时**: Tokio
- **序列化**: Serde

## 快速启动

### 1. 启动数据库

```bash
cd mariadb
start.bat
```

### 2. 启动后端

```bash
cd tinyengine-backend
cargo run
```

服务启动在 `http://localhost:8080`

## 默认凭证

开发环境下任意登录凭证均可使用：

| 用户名 | 密码 | 角色 |
|--------|------|------|
| `developer` | `123456`（或任意） | admin |

后端会在首次登录时自动创建用户。

## 前端集成

### 开发服务器问题

TinyEngine 开发服务器与 `xss` npm 包存在兼容性问题。**请使用生产构建版本。**

### 构建前端

```bash
cd tiny-engine

# 安装依赖（如需要）
CI=true pnpm install

# 构建插件
CI=true pnpm run build:plugin

# 构建生产版本（可选，用于部署）
CI=true pnpm run build:alpha
```

### 启动前端（开发模式 - 推荐）

**开发服务器会自动代理 API 请求到后端：**

```bash
cd tiny-engine/designer-demo

# 启动开发服务器（端口 8090，有代理配置）
pnpm dev
```

访问：`http://localhost:8090/?type=app&id=1&tenant=1&pageid=1`

### 启动前端（生产模式）

如果需要使用生产构建，必须配置 API 代理：

```bash
cd tiny-engine/designer-demo

# 方式一：使用 serve + 代理插件
npx serve -l 8091 -s dist

# 方式二：使用 Node 服务器手动代理
node -e "
const http = require('http');
const fs = require('fs');
const path = require('path');
const { createServer } = require('http-proxy');

// 创建代理服务器
const proxy = createServer({});
proxy.on('error', (err, req, res) => { res.writeHead(500); res.end('Proxy error'); });

// 创建静态文件服务器
const mime = {'.html':'text/html','.js':'application/javascript','.css':'text/css','.json':'application/json'};
const server = http.createServer((req, res) => {
  // 代理 API 请求
  if (req.url.startsWith('/app-center') || req.url.startsWith('/material-center') || req.url.startsWith('/platform-center')) {
    proxy.web(req, res, { target: 'http://127.0.0.1:8080' });
    return;
  }
  // 静态文件
  let file = path.join('./dist', req.url === '/' ? 'index.html' : req.url);
  if (!fs.existsSync(file)) file = path.join('./dist', 'index.html');
  res.writeHead(200, {'Content-Type': mime[path.extname(file)] || 'application/octet-stream'});
  res.end(fs.readFileSync(file));
});
server.listen(8091, () => console.log('Server on 8091'));
"
```

**重要：生产构建需要额外的代理配置，因为 Vite 代理只在开发模式生效。**

### 配置代理（重要）

前端需要配置 API 代理才能连接到 Rust 后端。

编辑 `designer-demo/vite.config.js`，在 `useTinyEngineBaseConfig()` 返回的配置后添加：

```javascript
// Override proxy to point to Rust backend
baseConfig.server.proxy = {
  '/app-center': {
    target: 'http://127.0.0.1:8080',
    changeOrigin: true
  },
  '/material-center': {
    target: 'http://127.0.0.1:8080',
    changeOrigin: true
  },
  '/platform-center': {
    target: 'http://127.0.0.1:8080',
    changeOrigin: true
  }
}
baseConfig.server.port = 8090
```

**说明**：
- 开发服务器端口改为 `8090`
- API 请求代理到 Rust 后端 `http://127.0.0.1:8080`
- 生产构建无需此配置，直接访问后端

### 访问应用

```
http://localhost:8091/?type=app&id=1&tenant=1&pageid=1
```

登录凭证: `developer` / `123456`

## API 端点

### 应用中心

| 方法 | 端点 | 描述 |
|------|------|------|
| GET | /app-center/api/apps | 获取所有应用 |
| POST | /app-center/api/apps/create | 创建应用 |
| GET | /app-center/api/apps/detail/:id | 获取应用详情 |
| PUT | /app-center/api/apps/:id | 更新应用 |
| DELETE | /app-center/api/apps/delete/:id | 删除应用 |
| GET | /app-center/api/apps/schema/:id | 获取应用 Schema |
| POST | /app-center/api/apps/publish/:id | 发布应用 |

### 页面管理

| 方法 | 端点 | 描述 |
|------|------|------|
| POST | /app-center/api/pages/create | 创建页面 |
| GET | /app-center/api/pages/list/:appId | 获取页面列表 |
| GET | /app-center/api/pages/detail/:id | 获取页面详情 |
| POST | /app-center/api/pages/update/:id | 更新页面 |
| DELETE | /app-center/api/pages/delete/:id | 删除页面 |
| GET | /app-center/api/pages/histories | 获取页面历史 |
| POST | /app-center/api/pages/histories/create | 创建历史版本 |

### 物料中心

| 方法 | 端点 | 描述 |
|------|------|------|
| POST | /material-center/api/block/create | 创建区块 |
| GET | /material-center/api/block/detail/:id | 获取区块详情 |
| PUT | /material-center/api/block/update/:id | 更新区块 |
| DELETE | /material-center/api/block/delete/:id | 删除区块 |
| POST | /material-center/api/block/deploy | 部署区块 |
| GET | /material-center/api/block-categories | 获取分类列表 |

### 代码生成

| 方法 | 端点 | 描述 |
|------|------|------|
| POST | /material-center/api/schema2code | 从 Schema 生成代码 |

### 平台（认证）

| 方法 | 端点 | 描述 |
|------|------|------|
| POST | /platform-center/api/user/login | 用户登录 |
| POST | /platform-center/api/user/register | 用户注册 |

## 配置

创建 `config.yaml` 或使用 `.env`:

```bash
cp .env.example .env
```

默认配置:

```yaml
database:
  host: localhost
  port: 3306
  username: root
  password: ""
  name: tinyengine

app:
  host: 0.0.0.0
  port: 8080
```

## 数据库

数据库 Schema 基于官方 TinyEngine Java 后端。初始化数据库：

```bash
cd mariadb/bin
./mysql -u root tinyengine < ../tinyengine-backend/scripts/init.sql
```

## 项目结构

```
tinyengine-backend/
├── src/
│   ├── api/
│   │   ├── handlers/     # 请求处理器
│   │   ├── models/      # 数据模型
│   │   └── routes/      # 路由定义
│   ├── config/          # 配置模块
│   ├── db/              # 数据库工具
│   ├── error/           # 错误处理
│   └── main.rs          # 入口文件
├── config.yaml          # 配置文件
├── .env.example         # 环境变量模板
├── Cargo.toml           # 依赖管理
└── scripts/
    └── init.sql         # 数据库 Schema
```

## 许可证

MIT
