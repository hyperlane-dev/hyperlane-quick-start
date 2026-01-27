---
name: gomoku-websocket-theme-toast
overview: 为五子棋单独定义全局WebSocket与相关全局变量，并将五子棋前端主题调整为项目通用风格，同时在游戏开始/结束时弹出toast提示。
design:
  architecture:
    framework: html
  styleKeywords:
    - 渐变背景
    - 白色卡片
    - 轻盈阴影
    - 现代清爽
    - 柔和动效
  fontSystem:
    fontFamily: Roboto
    heading:
      size: 28px
      weight: 600
    subheading:
      size: 18px
      weight: 500
    body:
      size: 14px
      weight: 400
  colorSystem:
    primary:
      - "#667EEA"
      - "#764BA2"
    background:
      - "#F5F7FB"
      - "#FFFFFF"
    text:
      - "#2C3E50"
      - "#6C757D"
    functional:
      - "#22C55E"
      - "#F59E0B"
      - "#EF4444"
todos:
  - id: add-gomoku-global-websocket
    content: 新增五子棋独立全局 WebSocket 与获取方法
    status: completed
  - id: update-gomoku-ws-usage
    content: 替换控制器与服务中 WebSocket 引用为 Gomoku 专用实例
    status: completed
    dependencies:
      - add-gomoku-global-websocket
  - id: emit-start-event
    content: 在游戏开始时广播 Start 事件到房间成员
    status: completed
    dependencies:
      - update-gomoku-ws-usage
  - id: update-gomoku-theme
    content: 调整五子棋页面为渐变背景与白色卡片主题
    status: completed
  - id: add-toast-ui
    content: 新增 toast 容器与基础样式动画
    status: completed
    dependencies:
      - update-gomoku-theme
  - id: toast-status-logic
    content: 根据开始事件与结束状态展示开始/胜者提示
    status: completed
    dependencies:
      - emit-start-event
      - add-toast-ui
---

## Product Overview

为五子棋页面提供独立的全局 WebSocket 与游戏状态提示能力，页面整体改为渐变背景与白色卡片的统一视觉风格，并在游戏开始/结束时出现轻量提示弹窗。

## Core Features

- 独立维护五子棋全局连接与状态数据，避免复用其他模块的连接
- 游戏开始与结束触发提示，结束提示包含胜者（黑/白）
- 页面主题统一为渐变背景 + 白色卡片布局，提升一致性与清晰度

## Tech Stack

- 后端：Rust（Hyperlane WebSocket）
- 前端：原生 HTML/CSS/JavaScript

## Tech Architecture

### Module Division

- **Gomoku WebSocket 模块**：独立全局 WebSocket 实例与广播逻辑
- **Gomoku 领域/映射模块**：房间状态与玩家映射存储
- **Gomoku 前端页面**：统一主题样式与 toast 提示交互

### Data Flow

- 客户端发送加入/落子请求 → 服务端更新房间状态 → 广播房间数据与开始/结束事件 → 前端更新 UI 与 toast 提示

## Implementation Details

### Core Directory Structure (modified)

```
app/
├── mapper/
│   └── gomoku/
│       ├── static.rs          # 新增 Gomoku 独立 WebSocket 全局变量
│       └── fn.rs              # 新增 Gomoku WebSocket 获取方法
├── service/
│   └── gomoku_websocket/
│       └── impl.rs            # 使用 Gomoku WebSocket + 广播开始事件
├── controller/
│   └── gomoku/
│       └── impl.rs            # 使用 Gomoku WebSocket
resources/
└── static/
    └── gomoku/
        └── index.html         # 统一主题与 toast 逻辑
```

### Key Code Structures

- **全局 WebSocket Getter**

```rust
pub fn get_global_gomoku_websocket() -> &'static WebSocket
```

用于 Gomoku 专用连接池的初始化与复用。

- **开始事件广播**
在游戏状态切换为进行中时，构建 `Start` 类型响应并广播到房间用户。

### Technical Implementation Plan

1. **独立 Gomoku 全局 WebSocket**

- 方案：在 gomoku mapper 的 static/fn 中新增 WebSocket 全局变量与 getter
- 步骤：定义 OnceLock → getter 返回 → 替换现有引用

2. **开始/结束事件触发**

- 方案：当第二位玩家加入并触发开始时广播 Start；结束由 RoomState 的 winner 信息判断
- 步骤：start_game 后构建 Start 响应 → 发送给房间成员

3. **前端主题与 toast**

- 方案：替换为渐变背景+白卡样式，新增 toast 容器与状态切换提示
- 步骤：更新样式变量 → 添加 toast DOM → 监听 Start/Finished 状态展示提示

### Testing Strategy

- 创建/加入房间验证开始 toast
- 完成对局验证结束 toast 与胜者显示
- 断线重连确保 UI 状态与房间数据同步

## Design Style

采用现代渐变背景与白色卡片的统一风格，整体清爽明亮，强调层次与可读性；交互元素使用柔和阴影与轻微动效，toast 以顶部浮层方式提示状态变化。

## Page Plan

- 单页应用：Gomoku 游戏页

## Block Design

1. **顶部导航栏**：居中品牌标题与连接状态，白色文字与轻阴影，保持页面辨识度。  
2. **房间控制卡片**：白色圆角卡片，输入与按钮整齐对齐，突出操作区。  
3. **棋盘区域**：白卡容器内展示棋盘，棋子对比清晰，边框轻柔。  
4. **比赛信息卡片**：玩家/观战/走子列表使用浅灰底块，易于扫读。  
5. **Toast 提示层**：顶部居中浮层，浅色背景与强调色边框，淡入淡出。  
6. **底部导航栏**：简洁版权信息，与顶部呼应。