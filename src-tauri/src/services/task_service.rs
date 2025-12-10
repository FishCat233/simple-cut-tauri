use anyhow::{anyhow, Result};
use log::{info, warn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::models::task::{TaskInfo, TaskProgress, TaskStatus, TaskSummary, TaskType};
use crate::models::video::{ClipParams, MergeParams};
use crate::services::video_service::VideoService;

/// 任务执行上下文，用于控制任务执行
struct TaskContext {
    cancel_tx: Option<oneshot::Sender<()>>,
    progress: Mutex<TaskProgress>,
}

/// 任务管理服务
pub struct TaskService {
    video_service: Arc<VideoService>,
    tasks: RwLock<HashMap<Uuid, TaskInfo>>,
    task_contexts: RwLock<HashMap<Uuid, TaskContext>>,
    max_concurrent_tasks: usize,
}

impl TaskService {
    /// 创建新的任务管理服务实例
    pub fn new(video_service: Arc<VideoService>) -> Self {
        TaskService {
            video_service,
            tasks: RwLock::new(HashMap::new()),
            task_contexts: RwLock::new(HashMap::new()),
            max_concurrent_tasks: 2, // 默认最大并发任务数
        }
    }

    /// 设置最大并发任务数
    pub fn set_max_concurrent_tasks(&mut self, max: usize) {
        self.max_concurrent_tasks = max;
    }

    /// 获取当前运行中的任务数
    fn get_running_tasks_count(&self) -> usize {
        let tasks = self.tasks.read().unwrap();
        tasks
            .values()
            .filter(|task| task.status == TaskStatus::Running)
            .count()
    }

    /// 创建新的剪辑任务
    pub fn create_clip_task(&self, params: ClipParams) -> Result<Uuid> {
        let task_id = Uuid::new_v4();
        let now = SystemTime::now();

        // 创建任务信息
        let task_info = TaskInfo {
            id: task_id,
            task_type: TaskType::Clip,
            status: TaskStatus::Pending,
            params: serde_json::to_string(&params)?,
            progress: TaskProgress {
                percentage: 0.0,
                current: 0,
                total: 100,
                message: Some("任务已创建，等待执行".to_string()),
                error: None,
            },
            created_at: now,
            started_at: None,
            completed_at: None,
        };

        // 保存任务信息
        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(task_id, task_info);

        info!("Created clip task: {}", task_id);
        Ok(task_id)
    }

    /// 创建新的合并任务
    pub fn create_merge_task(&self, params: MergeParams) -> Result<Uuid> {
        let task_id = Uuid::new_v4();
        let now = SystemTime::now();

        // 创建任务信息
        let task_info = TaskInfo {
            id: task_id,
            task_type: TaskType::Merge,
            status: TaskStatus::Pending,
            params: serde_json::to_string(&params)?,
            progress: TaskProgress {
                percentage: 0.0,
                current: 0,
                total: 100,
                message: Some("任务已创建，等待执行".to_string()),
                error: None,
            },
            created_at: now,
            started_at: None,
            completed_at: None,
        };

        // 保存任务信息
        let mut tasks = self.tasks.write().unwrap();
        tasks.insert(task_id, task_info);

        info!("Created merge task: {}", task_id);
        Ok(task_id)
    }

    /// 启动任务
    pub async fn start_task(&self, task_id: Uuid) -> Result<()> {
        // 检查任务是否存在
        let mut tasks = self.tasks.write().unwrap();
        let task = tasks.get_mut(&task_id);

        if let Some(task) = task {
            // 检查任务状态
            if task.status != TaskStatus::Pending {
                return Err(anyhow!("Task is not in pending state: {:?}", task.status));
            }

            // 检查并发任务数限制
            if self.get_running_tasks_count() >= self.max_concurrent_tasks {
                return Err(anyhow!("Too many concurrent tasks running"));
            }

            // 更新任务状态
            task.status = TaskStatus::Running;
            task.started_at = Some(SystemTime::now());
            task.progress.message = Some("任务开始执行".to_string());

            // 释放锁，避免死锁
            drop(tasks);

            // 执行任务
            self.execute_task(task_id).await;

            Ok(())
        } else {
            Err(anyhow!("Task not found: {}", task_id))
        }
    }

    /// 执行任务的内部方法
    async fn execute_task(&self, task_id: Uuid) {
        let task = {
            let tasks = self.tasks.read().unwrap();
            tasks.get(&task_id).cloned()
        };

        if let Some(mut task) = task {
            // 创建取消通道
            let (cancel_tx, cancel_rx) = oneshot::channel();

            // 保存任务上下文
            {
                let mut contexts = self.task_contexts.write().unwrap();
                contexts.insert(
                    task_id,
                    TaskContext {
                        cancel_tx: Some(cancel_tx),
                        progress: Mutex::new(task.progress.clone()),
                    },
                );
            }

            // 执行具体任务
            let result = match task.task_type {
                TaskType::Clip => {
                    let params: ClipParams = serde_json::from_str(&task.params).unwrap();
                    self.execute_clip_task(task_id, &params, cancel_rx).await
                }
                TaskType::Merge => {
                    let params: MergeParams = serde_json::from_str(&task.params).unwrap();
                    self.execute_merge_task(task_id, &params, cancel_rx).await
                }
                TaskType::Convert => {
                    // 格式转换任务
                    Err(anyhow!("Convert task not implemented yet"))
                }
                TaskType::ExtractAudio => {
                    // 提取音频任务
                    Err(anyhow!("Extract audio task not implemented yet"))
                }
            };

            // 更新任务状态
            let mut tasks = self.tasks.write().unwrap();
            let task = tasks.get_mut(&task_id);

            if let Some(task) = task {
                match result {
                    Ok(_) => {
                        task.status = TaskStatus::Completed;
                        task.completed_at = Some(SystemTime::now());
                        task.progress.percentage = 100.0;
                        task.progress.message = Some("任务执行完成".to_string());
                        task.progress.error = None;
                        info!("Task completed successfully: {}", task_id);
                    }
                    Err(e) => {
                        task.status = TaskStatus::Failed;
                        task.completed_at = Some(SystemTime::now());
                        task.progress.message = Some("任务执行失败".to_string());
                        task.progress.error = Some(e.to_string());
                        warn!("Task failed: {} - {}", task_id, e);
                    }
                }
            }

            // 清理任务上下文
            let mut contexts = self.task_contexts.write().unwrap();
            contexts.remove(&task_id);
        }
    }

    /// 执行剪辑任务
    async fn execute_clip_task<F>(
        &self,
        task_id: Uuid,
        params: &ClipParams,
        cancel_rx: oneshot::Receiver<()>,
    ) -> Result<()> {
        // 进度回调函数
        let progress_callback =
            |percentage: f32, message: Option<String>, error: Option<String>| {
                // 更新任务进度
                let tasks = self.tasks.write().unwrap();
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.progress.percentage = percentage;
                    if let Some(msg) = message {
                        task.progress.message = Some(msg);
                    }
                    if let Some(err) = error {
                        task.progress.error = Some(err);
                    }
                }
            };

        // 执行视频剪辑
        let result = self
            .video_service
            .clip_video(params, progress_callback)
            .await;

        // 检查是否被取消
        if cancel_rx.try_recv().is_ok() {
            return Err(anyhow!("Task cancelled"));
        }

        result
    }

    /// 执行合并任务
    async fn execute_merge_task<F>(
        &self,
        task_id: Uuid,
        params: &MergeParams,
        cancel_rx: oneshot::Receiver<()>,
    ) -> Result<()> {
        // 进度回调函数
        let progress_callback =
            |percentage: f32, message: Option<String>, error: Option<String>| {
                // 更新任务进度
                let tasks = self.tasks.write().unwrap();
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.progress.percentage = percentage;
                    if let Some(msg) = message {
                        task.progress.message = Some(msg);
                    }
                    if let Some(err) = error {
                        task.progress.error = Some(err);
                    }
                }
            };

        // 执行视频合并
        let result = self
            .video_service
            .merge_videos(params, progress_callback)
            .await;

        // 检查是否被取消
        if cancel_rx.try_recv().is_ok() {
            return Err(anyhow!("Task cancelled"));
        }

        result
    }

    /// 取消任务
    pub fn cancel_task(&self, task_id: Uuid) -> Result<()> {
        let mut contexts = self.task_contexts.write().unwrap();

        if let Some(context) = contexts.get_mut(&task_id) {
            if let Some(cancel_tx) = context.cancel_tx.take() {
                // 发送取消信号
                if cancel_tx.send(()).is_err() {
                    warn!("Failed to send cancel signal for task: {}", task_id);
                }

                // 更新任务状态
                let mut tasks = self.tasks.write().unwrap();
                if let Some(task) = tasks.get_mut(&task_id) {
                    task.status = TaskStatus::Cancelled;
                    task.completed_at = Some(SystemTime::now());
                    task.progress.message = Some("任务已取消".to_string());
                }

                info!("Task cancelled: {}", task_id);
                Ok(())
            } else {
                Err(anyhow!("Task already cancelled or completed: {}", task_id))
            }
        } else {
            Err(anyhow!("Task not found or not running: {}", task_id))
        }
    }

    /// 获取任务状态
    pub fn get_task_status(&self, task_id: Uuid) -> Option<TaskInfo> {
        let tasks = self.tasks.read().unwrap();
        tasks.get(&task_id).cloned()
    }

    /// 获取所有任务
    pub fn get_all_tasks(&self) -> Vec<TaskSummary> {
        let tasks = self.tasks.read().unwrap();
        tasks
            .values()
            .map(|task| TaskSummary {
                id: task.id,
                task_type: task.task_type.clone(),
                status: task.status.clone(),
                progress: task.progress.percentage,
                created_at: task.created_at,
                output_path: task.output_path.clone(),
            })
            .collect()
    }

    /// 清理已完成的任务
    pub fn cleanup_tasks(&self, older_than: Duration) -> usize {
        let now = SystemTime::now();
        let mut tasks = self.tasks.write().unwrap();
        let mut contexts = self.task_contexts.write().unwrap();

        let before_time = now - older_than;
        let mut removed_count = 0;

        // 过滤需要保留的任务
        let remaining_tasks: HashMap<_, _> = tasks
            .drain()
            .filter(|(id, task)| {
                if task.status.is_terminal() {
                    if let Some(completed_at) = task.completed_at {
                        if completed_at < before_time {
                            // 清理对应的上下文
                            contexts.remove(id);
                            removed_count += 1;
                            info!("Cleaned up completed task: {}", id);
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                } else {
                    true
                }
            })
            .collect();

        // 重新插入保留的任务
        *tasks = remaining_tasks;

        removed_count
    }
}
