# Habo — 运动 Agent 平台设计文档

> 编写日期：2026-05-08
> 状态：已批准
> 后续步骤：编写实现计划

---

## 一、产品定义

### 1.1 产品形态

| 终端 | 名称 | 技术栈 | 核心定位 |
|------|------|--------|---------|
| **iOS / Android** | Habo App | Flutter + Rust | 运动中实时陪练、设备连接、AI 分析触发 |
| **macOS / Windows** | Habo Platform | Tauri + React | 深度数据回顾、AI 分析报告、训练管理 |
| **后台服务** | Habo Backend | Rust + Axum + PostgreSQL + Redis | 认证、数据存储、业务逻辑 |
| **AI 服务** | Model Router | Rust + Axum (独立部署) | 统一模型路由转发 |

### 1.2 MVP 范围（Phase 1）

- **运动类型**：仅跑步，后续逐步扩展游泳、骑行、徒步
- **设备支持**：
  - Apple Watch：HealthKit 全量实时数据接入
  - 佳明/高驰：MVP 阶段运动期间用手机 GPS + 传感器，运动后通过厂商 API 同步完整数据
- **AI 陪练能力**：
  - **Habo App**：运动后数据分析 + 教练级反馈，运动中实时陪伴
  - **Habo Platform**：消费级深度分析报告、历史趋势、训练管理
- **认证**：手机号 + 验证码登录，支持 Apple ID 关联

### 1.3 用户故事

- 作为跑步爱好者，我打开 Habo App 开始跑步，App 实时显示我的配速、距离和心率
- 作为 Apple Watch 用户，我的手表数据自动同步到 App 获得更精准的分析
- 跑步结束后，Habo 陪练立即给出运动分析和训练建议
- 回家打开电脑，我可以在 Habo Platform 上查看详细的运动报告和长期趋势
- 佳明/高驰用户可以在 Habo Platform 上导入手表数据获得完整的 AI 分析

---

## 二、系统架构总览

```
┌─────────────────────────────────────────────────────────────────┐
│                        Habo 系统总览                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌───────────────────────┐   │
│  │  Habo App    │  │  Habo App   │  │   Habo Platform       │   │
│  │  (iOS)       │  │  (Android)  │  │   (macOS / Windows)   │   │
│  │  Flutter     │  │  Flutter    │  │   Tauri + React       │   │
│  │  + Rust 核心  │  │  + Rust 核心 │  │   + Rust 核心共享     │   │
│  └──────┬───────┘  └──────┬──────┘  └──────────┬────────────┘   │
│         │                 │                     │                │
│         └────────┬────────┘                     │                │
│                  │                              │                │
│         ▼        │                              │                │
│    ┌──────────────────────┐                     │                │
│    │    API Gateway       │ ◄───────────────────┘                │
│    │  (Rust / Axum)       │                                     │
│    └──┬────┬────┬────┬────┘                                     │
│       │    │    │    │                                          │
│       ▼    ▼    ▼    ▼                                          │
│  ┌────┐ ┌────┐ ┌───────┐ ┌─────────────────┐                   │
│  │Auth│ │User│ │Workout│ │  Model Router   │                   │
│  │Svc │ │Svc │ │Svc    │ │  (独立部署)      │                   │
│  └──┬─┘ └──┬─┘ └───┬───┘ └────────┬────────┘                   │
│     │      │        │              │                            │
│     ▼      ▼        ▼              ▼                            │
│  ┌──────┐ ┌──────┐ ┌────────┐ ┌──────────┐                     │
│  │ PG   │ │ Redis│ │ PG     │ │ DeepSeek │ ← 前期               │
│  │      │ │      │ │+Timescale│ 自训练模型│ ← 后期               │
│  └──────┘ └──────┘ └────────┘ └──────────┘                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**核心设计决策：**
1. **Flutter + Rust** 共享核心引擎（GPS 采集、配速计算、AI 上下文管理）跨平台
2. **Tauri + React** 桌面端，共享 Rust 核心库
3. **Rust + Axum** 微服务架构，API Gateway 统一入口
4. **PostgreSQL + TimescaleDB** 主数据库，Redis 缓存/队列
5. **Model Router 独立部署**，实现 DeepSeek → 自训练模型的无缝切换

---

## 三、Habo App（移动端）详细设计

### 3.1 架构分层

```
┌────────────────────────────────────────────┐
│              Flutter UI Layer               │
│  ├─ 跑步主页（实时配速/路线/心率展示）        │
│  ├─ 设备绑定管理                            │
│  ├─ AI 陪练对话界面                         │
│  ├─ 运动历史列表（简要数据）                 │
│  └─ 设置/登录                              │
├────────────────────────────────────────────┤
│           flutter_rust_bridge               │
├────────────────────────────────────────────┤
│              Rust Core Engine               │
│  ├─ GPS 数据采集引擎                        │
│  ├─ HealthKit Bridge (iOS)                 │
│  ├─ 配速/距离实时计算                       │
│  ├─ 运动状态管理（起跑/暂停/结束）           │
│  ├─ BLE 通信层（预留佳明/高驰）             │
│  └─ AI Agent 上下文管理                    │
├────────────────────────────────────────────┤
│        原生桥接层 (Platform Channel)         │
│  ├─ iOS: HealthKit, Watch Connectivity     │
│  └─ Android: 前台服务，GPS                  │
└────────────────────────────────────────────┘
```

### 3.2 功能列表

| 功能 | 描述 |
|------|------|
| 手机号登录/注册 | 验证码登录，支持 Apple ID 关联 |
| 设备绑定 | 绑定 Apple Watch / 佳明 / 高驰账号 |
| 跑步模式 | 起跑/暂停/结束，实时 GPS 追踪 |
| 实时数据显示 | 配速、距离、心率、时长 |
| AI 实时陪练 | 运动中语音/文字交互，状态提醒 |
| 运动后分析 | 数据上传后触发 AI 分析，显示结果 |
| 历史记录 | 简要的运动列表和关键指标 |
| 离线支持 | 运动中网络不佳时本地缓存，恢复后同步 |

### 3.3 数据获取策略

| 设备 | 运动中 | 运动后 |
|------|--------|--------|
| **Apple Watch** | HealthKit Live Workout API | 全量数据分析 |
| **佳明/高驰** | 手机 GPS + 传感器（配速/路线/陪伴） | 厂商 API 同步全量数据（需 Habo Platform） |

---

## 四、Habo Platform（桌面端）详细设计

### 4.1 技术选型

- **框架**：Tauri v2（Rust 后端 + WebView 前端）
- **前端**：React + TypeScript
- **UI 库**：shadcn/ui + Tailwind CSS
- **图表**：ECharts / Recharts
- **本地缓存**：Tauri 内置 SQLite
- **地图**：Mapbox / Leaflet（GPS 轨迹回放）

### 4.2 功能模块

| 模块 | 描述 |
|------|------|
| 仪表盘 | 本周/月训练概览（跑量、次数、平均配速、心率趋势） |
| 运动详情 | GPS 地图回放、每公里分段分析、心率区间分布 |
| AI 分析报告 | 每次运动的陪练分析全文、评分、建议 |
| 趋势分析 | 长期配速/心率/跑量变化趋势、进步曲线 |
| 训练计划 | AI 生成的个性化周训练计划（Phase 2+） |
| 数据导入 | 从佳明/高驰手动触发同步，Apple Watch 数据同步 |

---

## 五、后台服务详细设计

### 5.1 服务拆分

| 服务 | 职责 | 关键依赖 |
|------|------|---------|
| **API Gateway** | 路由转发、JWT 验证、速率限制、请求日志 | Redis |
| **Auth Service** | 手机号验证码登录、JWT 签发、Apple ID 登录 | PG + Redis |
| **User Service** | 用户资料 CRUD、跑步目标设置、统计概览 | PG |
| **Workout Service** | 运动记录 CRUD、GPS 轨迹存储、AI 分析结果关联 | PG + TimescaleDB |
| **Device Service** | 设备绑定/解绑、厂商 API 同步触发、HealthKit 数据接收 | PG |
| **Model Router** | 统一模型接口、上下文管理、多模型路由（独立部署） | PG + Redis |

### 5.2 认证流程

```
1. 用户输入手机号
2. POST /auth/send-code → 验证码存入 Redis（5min TTL）+ PG 留记录
3. 用户输入验证码
4. POST /auth/verify → 校验验证码 → 签发 JWT (access + refresh token)
5. 可选：POST /auth/bind-apple → 关联 Apple ID
6. 后续请求 Authorization: Bearer <access_token>
```

### 5.3 核心数据库表

详见 `docs/superpowers/specs/2026-05-08-habo-database-schema.md`（后续补充详细 DDL）

主要表：`users`, `verification_codes`, `user_profiles`, `device_bindings`, `workouts`, `workout_analysis`, `ai_conversations`

### 5.4 Redis 使用场景

| Key 模式 | 用途 | TTL |
|----------|------|-----|
| `sms:code:{phone}` | 验证码缓存 | 5min |
| `session:token:{jti}` | JWT 黑名单 | 7天 |
| `live:workout:{user_id}` | 运动中实时数据 | 运动期间 |
| `model:rate_limit:{user_id}` | 模型调用频率控制 | 滑动窗口 |

---

## 六、Model Router（模型路由服务）

### 6.1 设计定位

独立部署的模型网关服务，对 Habo App 和 Habo Platform 提供统一的 AI 接口，封装底层模型切换。

### 6.2 路由策略

| 请求类型 | 前期 | 后期 |
|---------|------|------|
| 实时陪练对话 | DeepSeek API | DeepSeek / 端侧模型 |
| 运动后分析 | DeepSeek + 运动数据 Prompt | Habo-Run-V1（专精模型） |
| 平台深度分析 | DeepSeek（长上下文） | Habo-Run-V1（大参数量版本） |

### 6.3 降级策略

- 主模型超时 → 自动切备用模型
- 全部不可用 → 返回离线模板分析
- 频率过高 → 降级到缓存或简化回复

---

## 七、模型训练方案

### 7.1 阶段一：Prompt Engineering（DeepSeek）

- 构建跑步分析 Prompt 模板
- 收集 AI 分析结果作为训练数据种子
- 建立评估体系

### 7.2 阶段二：LoRA 微调

| 项目 | 选择 |
|------|------|
| **底座模型** | Qwen2.5-7B 或 Llama-3.2-8B |
| **微调方式** | LoRA / QLoRA |
| **硬件需求** | 1× RTX 4090 24GB |
| **推理部署** | vLLM 或 Ollama |
| **模型名称** | Habo-Run-V1 |

---

## 八、技术栈汇总

| 层级 | 技术选型 |
|------|---------|
| 移动端 UI | Flutter (Dart) |
| 移动端核心引擎 | Rust（flutter_rust_bridge） |
| 桌面端框架 | Tauri v2 + React + TypeScript |
| 桌面端 UI | shadcn/ui + Tailwind CSS |
| 后台服务 | Rust + Axum + sqlx + Tower |
| 模型路由服务 | Rust + Axum（独立部署） |
| 主数据库 | PostgreSQL（+ TimescaleDB 扩展） |
| 缓存/队列 | Redis |
| 认证 | JWT + 手机号验证码 + Apple ID |
| 前期 AI | DeepSeek API |
| 后期 AI | Qwen/Llama 微调 → vLLM 部署 |
| 容器化 | Docker（后期 K8s） |

---

## 九、Phase 1 开发路线图

| 阶段 | 内容 | 关键里程碑 |
|------|------|-----------|
| **P0 基础设施** | Rust 后台脚手架、PG schema、Redis 集成、API Gateway | 后端能跑通 CRUD |
| **P1 认证系统** | 手机号验证码、JWT、Apple ID 绑定、用户资料 | 用户能注册登录 |
| **P2 运动核心** | Workout 服务、GPS 数据采集、运动记录 CRUD、设备绑定 | App 能记录跑步 |
| **P3 AI 陪练** | Model Router、DeepSeek 集成、分析 Prompt、分析结果保存 | 跑完能看 AI 分析 |
| **P4 移动端 App** | Flutter 项目搭建、跑步模式 UI、实时数据显示、Rust 核心引擎 | Habo App 可用 |
| **P5 桌面端** | Tauri 项目搭建、React UI、数据展示、图表、分析报告 | Habo Platform 可用 |
| **P6 深度集成** | Garmin/Coros API 同步、Apple Watch HealthKit、离线支持 | 设备全接入 |
| **P7 模型训练** | 数据积累 → LoRA 微调 → Habo-Run-V1 部署 | 自训练模型替换 |

---

*本文档由 Habo 项目设计会议产生，已获用户批准。下一步：编写实现计划。*
