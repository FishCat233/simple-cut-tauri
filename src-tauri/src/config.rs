use anyhow::Result;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 应用配置结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// FFmpeg配置
    pub ffmpeg: FfmpegConfig,
    /// 视频处理默认配置
    pub video: VideoConfig,
    /// 任务管理配置
    pub task: TaskConfig,
    /// 日志配置
    pub logging: LoggingConfig,
}

/// FFmpeg相关配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FfmpegConfig {
    /// FFmpeg可执行文件路径
    pub path: Option<String>,
    /// FFmpeg默认参数
    pub default_args: Vec<String>,
    /// 超时时间（秒）
    pub timeout: u32,
}

/// 视频处理默认配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VideoConfig {
    /// 默认比特率（kbps）
    pub default_bitrate: u32,
    /// 默认编码器
    pub default_codec: String,
    /// 默认分辨率宽度
    pub default_width: Option<u32>,
    /// 默认分辨率高度
    pub default_height: Option<u32>,
}

/// 任务管理配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskConfig {
    /// 最大并发任务数
    pub max_concurrent_tasks: usize,
    /// 任务自动清理时间（天）
    pub auto_cleanup_days: u32,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// 日志级别
    pub log_level: String,
    /// 是否启用文件日志
    pub enable_file_log: bool,
    /// 日志文件路径
    pub log_file: Option<String>,
}

/// 配置管理服务
pub struct ConfigService {
    config: AppConfig,
    config_path: Option<String>,
}

impl ConfigService {
    /// 创建新的配置服务实例
    pub fn new() -> Self {
        ConfigService {
            config: AppConfig::default(),
            config_path: None,
        }
    }

    /// 加载配置文件
    pub fn load_config(&mut self) -> Result<()> {
        // 使用程序根目录作为配置目录
        let mut dir = std::env::current_dir()?;

        // 配置文件路径
        dir.push("config.toml");
        let config_path = dir.to_string_lossy().to_string();
        self.config_path = Some(config_path.clone());

        // 如果配置文件存在，加载它
        if Path::new(&config_path).exists() {
            info!("Loading config from file: {}", config_path);

            let content = fs::read_to_string(&config_path)?;
            self.config = toml::from_str(&content)?;

            debug!("Config loaded: {:?}", self.config);
        } else {
            // 配置文件不存在，使用默认配置
            info!("No config file found, using default config");
            self.save_config()?;
        }

        Ok(())
    }

    /// 保存配置到文件
    pub fn save_config(&self) -> Result<()> {
        if let Some(config_path) = &self.config_path {
            info!("Saving config to file: {}", config_path);

            let content = toml::to_string_pretty(&self.config)?;
            fs::write(config_path, content)?;

            debug!("Config saved successfully");
        }

        Ok(())
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    /// 更新配置
    pub fn update_config(&mut self, config: AppConfig) -> Result<()> {
        self.config = config;
        self.save_config()
    }

    /// 更新FFmpeg配置
    pub fn update_ffmpeg_config(&mut self, ffmpeg_config: FfmpegConfig) -> Result<()> {
        self.config.ffmpeg = ffmpeg_config;
        self.save_config()
    }

    /// 更新视频处理配置
    pub fn update_video_config(&mut self, video_config: VideoConfig) -> Result<()> {
        self.config.video = video_config;
        self.save_config()
    }

    /// 更新任务管理配置
    pub fn update_task_config(&mut self, task_config: TaskConfig) -> Result<()> {
        self.config.task = task_config;
        self.save_config()
    }

    /// 更新日志配置
    pub fn update_logging_config(&mut self, logging_config: LoggingConfig) -> Result<()> {
        self.config.logging = logging_config;
        self.save_config()
    }
}
