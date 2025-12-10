use log::info;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

// 导入服务
mod commands;
mod config;
mod models;
mod services;

// 导入具体服务
use crate::config::ConfigService;
use crate::services::ffmpeg_service::FfmpegService;
use crate::services::task_service::TaskService;
use crate::services::video_service::VideoService;

// 导入命令
use crate::commands::{
    cancel_task, cleanup_tasks, create_clip_task, create_merge_task, get_all_tasks,
    get_task_status, get_video_info, ping, start_task,
};

// 旧的greet命令保持不变，作为示例
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 初始化服务的函数
fn init_services(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing backend services...");

    // 1. 加载配置
    let mut config_service = ConfigService::new();
    config_service.load_config()?;
    let config = config_service.get_config().clone();

    // 2. 创建FFmpeg服务
    let ffmpeg_service = Arc::new(FfmpegService::new(app.clone(), config.ffmpeg.path.clone()));

    // 3. 创建视频服务
    let video_service = Arc::new(VideoService::new(ffmpeg_service.clone()));

    // 4. 创建任务管理服务
    let mut task_service = TaskService::new(video_service.clone());

    task_service.set_max_concurrent_tasks(config.task.max_concurrent_tasks); // 设置最大并发任务数

    let task_service = Arc::new(task_service);

    // 5. 将服务注册为应用状态
    app.manage(Arc::new(config_service));
    app.manage(ffmpeg_service);
    app.manage(video_service);
    app.manage(task_service);

    info!("Backend services initialized successfully");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    env_logger::init();

    info!("Starting Simple Cut Tauri application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // 注册插件
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        // 初始化服务
        .setup(|app| {
            init_services(&app.handle())?;
            Ok(())
        })
        // 注册所有命令
        .invoke_handler(tauri::generate_handler![
            // 旧的greet命令
            greet,
            // 新的视频处理命令
            get_video_info,
            create_clip_task,
            create_merge_task,
            start_task,
            cancel_task,
            get_task_status,
            get_all_tasks,
            cleanup_tasks,
            ping
        ])
        // 运行应用
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
