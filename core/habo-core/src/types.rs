//! 共享数据类型和 API 请求/响应结构

use serde::{Deserialize, Serialize};

/// 标准 API 响应包装
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self { success: false, data: None, error: Some(msg.into()) }
    }
}

/// 分页响应
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub success: bool,
    pub data: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
}

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

// ---- Auth DTOs ----

/// 发送验证码请求
#[derive(Debug, Deserialize)]
pub struct SendCodeRequest {
    pub phone: String,
}

/// 验证码校验请求
#[derive(Debug, Deserialize)]
pub struct VerifyCodeRequest {
    pub phone: String,
    pub code: String,
}

/// 验证码校验响应
#[derive(Debug, Serialize)]
pub struct VerifyCodeResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: uuid::Uuid,
}

/// Token 刷新请求
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Apple ID 绑定请求
#[derive(Debug, Deserialize)]
pub struct BindAppleRequest {
    pub apple_id: String,
    pub identity_token: String,
}

// ---- User DTOs ----

/// 更新用户资料请求
#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub gender: Option<i16>,
    pub height_cm: Option<i32>,
    pub weight_kg: Option<rust_decimal::Decimal>,
    pub running_goal: Option<String>,
    pub weekly_goal_km: Option<rust_decimal::Decimal>,
}

/// 用户资料响应
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UserProfileResponse {
    pub user_id: uuid::Uuid,
    pub phone: String,
    pub nickname: String,
    pub avatar_url: String,
    pub gender: i16,
    pub height_cm: Option<i32>,
    pub weight_kg: Option<rust_decimal::Decimal>,
    pub running_goal: String,
    pub weekly_goal_km: Option<rust_decimal::Decimal>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ---- Device DTOs ----

/// 绑定设备请求
#[derive(Debug, Deserialize)]
pub struct BindDeviceRequest {
    pub device_type: String,
    pub device_name: String,
    pub config_json: Option<serde_json::Value>,
}

/// 设备绑定响应
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DeviceBindingResponse {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub device_type: String,
    pub device_name: String,
    pub is_active: bool,
    pub config_json: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ---- Workout DTOs ----

/// 创建运动请求
#[derive(Debug, Deserialize)]
pub struct CreateWorkoutRequest {
    pub device_id: Option<uuid::Uuid>,
    pub sport_type: Option<String>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_secs: Option<i32>,
    pub distance_meters: Option<rust_decimal::Decimal>,
    pub avg_heart_rate: Option<i16>,
    pub max_heart_rate: Option<i16>,
    pub avg_pace_km: Option<rust_decimal::Decimal>,
    pub calories_kcal: Option<i32>,
    pub elevation_gain_m: Option<rust_decimal::Decimal>,
    pub route_data: Option<serde_json::Value>,
    pub splits_data: Option<serde_json::Value>,
    pub sensor_data: Option<serde_json::Value>,
    pub notes: Option<String>,
}

/// 更新运动请求
#[derive(Debug, Deserialize)]
pub struct UpdateWorkoutRequest {
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_secs: Option<i32>,
    pub distance_meters: Option<rust_decimal::Decimal>,
    pub avg_heart_rate: Option<i16>,
    pub max_heart_rate: Option<i16>,
    pub avg_pace_km: Option<rust_decimal::Decimal>,
    pub calories_kcal: Option<i32>,
    pub elevation_gain_m: Option<rust_decimal::Decimal>,
    pub route_data: Option<serde_json::Value>,
    pub splits_data: Option<serde_json::Value>,
    pub notes: Option<String>,
}

/// 运动记录响应（含所有详细数据）
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WorkoutResponse {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub device_id: Option<uuid::Uuid>,
    pub sport_type: String,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_secs: Option<i32>,
    pub distance_meters: Option<rust_decimal::Decimal>,
    pub avg_heart_rate: Option<i16>,
    pub max_heart_rate: Option<i16>,
    pub avg_pace_km: Option<rust_decimal::Decimal>,
    pub calories_kcal: Option<i32>,
    pub elevation_gain_m: Option<rust_decimal::Decimal>,
    pub route_data: serde_json::Value,
    pub splits_data: serde_json::Value,
    pub sensor_data: serde_json::Value,
    pub source: String,
    pub notes: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// 运动列表项（不含详细传感器数据）
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct WorkoutListItem {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub sport_type: String,
    pub status: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub duration_secs: Option<i32>,
    pub distance_meters: Option<rust_decimal::Decimal>,
    pub avg_heart_rate: Option<i16>,
    pub max_heart_rate: Option<i16>,
    pub avg_pace_km: Option<rust_decimal::Decimal>,
    pub calories_kcal: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
