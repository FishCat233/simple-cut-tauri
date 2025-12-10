use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

/// 任务状态枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    /// 判断任务是否处于终端状态（Completed、Failed、Cancelled）
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }
}

/// 任务类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    ClipVideo,
    MergeVideos,
    ConvertFormat,
    ExtractAudio,
}

/// 任务进度信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub percentage: f32, // 0-100
    pub current: u64,
    pub total: u64,
    pub message: Option<String>, // 进度信息
    pub error: Option<String>,   // 错误信息
}

/// 任务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: Uuid,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub progress: TaskProgress,
    pub created_at: SystemTime,
    pub started_at: Option<SystemTime>,
    pub completed_at: Option<SystemTime>,
    pub input_paths: Vec<String>,
    pub output_path: Option<String>,
}

/// 任务摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSummary {
    pub id: Uuid,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub progress: f32,
    pub created_at: SystemTime,
    pub output_path: Option<String>,
}
