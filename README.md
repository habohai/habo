# Habo — AI 跑步陪练平台

## 快速开始

### 环境要求
- Rust 1.91+
- Flutter (待安装)
- PostgreSQL 16+
- Redis 7+

### 项目结构
```
habo/
├── backend/          # Rust 后台服务 (workspace)
│   ├── Cargo.toml
│   ├── gateway/      # API Gateway
│   ├── auth/         # Auth Service
│   ├── user/         # User Service
│   ├── workout/      # Workout Service
│   ├── device/       # Device Service
│   └── model-router/ # Model Router (独立部署)
├── mobile/           # Flutter App
├── platform/         # Tauri 桌面端
├── core/             # 共享 Rust 核心库
│   ├── habo-core/    # 核心类型、数据模型
│   └── habo-macros/  # 过程宏
└── docs/             # 设计文档
```
