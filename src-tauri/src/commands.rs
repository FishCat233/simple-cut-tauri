use anyhow::{anyhow, Result};
use log::debug;
use std::sync::Arc;
use tauri::{command, State};
use uuid::Uuid;

use crate::models::task::{TaskInfo, TaskSummary};
use crate::models::video::{ClipParams, MergeParams, VideoInfo};
use crate::services::task_service::TaskService;
use crate::services::video_service::VideoService;

/// 获取视频信息
#[command]
pub async fn get_video_info(
    video_service: State<'_, Arc<VideoService>>,
    file_path: String,
) -> Result<VideoInfo, String> {
    debug!("Command called: get_video_info for file: {}", file_path);

    match video_service.get_video_info(&file_path).await {
        Ok(info) => Ok(info),
        Err(e) => {
            debug!("Error getting video info: {}", e);
            Err(e.to_string())
        }
    }
}

/// 创建剪辑任务
#[command]
pub async fn create_clip_task(
    task_service: State<'_, Arc<TaskService>>,
    params: ClipParams,
) -> Result<String, String> {
    debug!("Command called: create_clip_task with params: {:?}", params);

    match task_service.create_clip_task(params) {
        Ok(task_id) => Ok(task_id.to_string()),
        Err(e) => {
            debug!("Error creating clip task: {}", e);
            Err(e.to_string())
        }
    }
}

/// 创建合并任务
#[command]
pub async fn create_merge_task(
    task_service: State<'_, Arc<TaskService>>,
    params: MergeParams,
) -> Result<String, String> {
    debug!(
        "Command called: create_merge_task with params: {:?}",
        params
    );

    match task_service.create_merge_task(params) {
        Ok(task_id) => Ok(task_id.to_string()),
        Err(e) => {
            debug!("Error creating merge task: {}", e);
            Err(e.to_string())
        }
    }
}

/// 启动任务
#[command]
pub async fn start_task(
    task_service: State<'_, Arc<TaskService>>,
    task_id_str: String,
) -> Result<(), String> {
    debug!("Command called: start_task for task_id: {}", task_id_str);

    // 解析任务ID
    let task_id = match Uuid::parse_str(&task_id_str) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid task ID format: {}", e)),
    };

    match task_service.start_task(task_id).await {
        Ok(()) => Ok(()),
        Err(e) => {
            debug!("Error starting task: {}", e);
            Err(e.to_string())
        }
    }
}

/// 取消任务
#[command]
pub async fn cancel_task(
    task_service: State<'_, Arc<TaskService>>,
    task_id_str: String,
) -> Result<(), String> {
    debug!("Command called: cancel_task for task_id: {}", task_id_str);

    // 解析任务ID
    let task_id = match Uuid::parse_str(&task_id_str) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid task ID format: {}", e)),
    };

    match task_service.cancel_task(task_id) {
        Ok(()) => Ok(()),
        Err(e) => {
            debug!("Error canceling task: {}", e);
            Err(e.to_string())
        }
    }
}

/// 获取任务状态
#[command]
pub async fn get_task_status(
    task_service: State<'_, Arc<TaskService>>,
    task_id_str: String,
) -> Result<Option<TaskInfo>, String> {
    debug!(
        "Command called: get_task_status for task_id: {}",
        task_id_str
    );

    // 解析任务ID
    let task_id = match Uuid::parse_str(&task_id_str) {
        Ok(id) => id,
        Err(e) => return Err(format!("Invalid task ID format: {}", e)),
    };

    Ok(task_service.get_task_status(task_id))
}

/// 获取所有任务
#[command]
pub async fn get_all_tasks(
    task_service: State<'_, Arc<TaskService>>,
) -> Result<Vec<TaskSummary>, String> {
    debug!("Command called: get_all_tasks");

    Ok(task_service.get_all_tasks())
}

/// 清理已完成的任务
#[command]
pub async fn cleanup_tasks(
    task_service: State<'_, Arc<TaskService>>,
    days: u64,
) -> Result<usize, String> {
    debug!("Command called: cleanup_tasks with days: {}", days);

    use std::time::Duration;
    let duration = Duration::from_secs(days * 24 * 3600);

    Ok(task_service.cleanup_tasks(duration))
}

/// 测试命令，用于验证前后端通信是否正常
#[command]
pub async fn ping() -> String {
    "pong".to_string()
}
