use serde::{Deserialize, Serialize};

/// 视频信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub duration: f64, // 秒
    pub format: String,
    pub bitrate: u32, // kbps
    pub codec: String,
    pub frame_rate: f64,
}

/// 视频剪辑参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipParams {
    pub input_path: String,
    pub output_path: String,
    pub start_time: f64,       // 秒
    pub end_time: f64,         // 秒
    pub bitrate: u32,          // kbps
    pub format: String,        // 输出格式
    pub codec: Option<String>, // 可选的视频编码器
    pub width: Option<u32>,    // 可选的输出宽度
    pub height: Option<u32>,   // 可选的输出高度
}

/// 视频合并参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeParams {
    pub input_paths: Vec<String>,
    pub output_path: String,
    pub format: String,
    pub bitrate: u32,
}
