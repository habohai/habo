//! 共享数据类型

use serde::{Deserialize, Serialize};

/// 运动类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SportType {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "swimming")]
    Swimming,
    #[serde(rename = "cycling")]
    Cycling,
    #[serde(rename = "hiking")]
    Hiking,
}

/// 设备类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    #[serde(rename = "apple_watch")]
    AppleWatch,
    #[serde(rename = "garmin")]
    Garmin,
    #[serde(rename = "coros")]
    Coros,
}

/// AI 模型意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelIntent {
    #[serde(rename = "analysis")]
    Analysis,
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "training_plan")]
    TrainingPlan,
}

/// 统一模型请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    pub user_id: uuid::Uuid,
    pub context: Vec<ChatMessage>,
    pub workout_data: Option<serde_json::Value>,
    pub intent: ModelIntent,
}

/// 对话消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// 统一模型响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub content: String,
    pub model_used: String,
    pub tokens_used: Option<u32>,
}
