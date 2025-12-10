use anyhow::{anyhow, Result};
use log::{debug, info};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::video::{ClipParams, MergeParams, VideoInfo};
use crate::services::ffmpeg_service::FfmpegService;

/// 视频处理服务，提供高级视频处理功能
pub struct VideoService {
    ffmpeg_service: Arc<FfmpegService>,
}

impl VideoService {
    /// 创建新的视频服务实例
    pub fn new(ffmpeg_service: Arc<FfmpegService>) -> Self {
        VideoService { ffmpeg_service }
    }

    /// 获取视频信息
    pub async fn get_video_info(&self, file_path: &str) -> Result<VideoInfo> {
        info!("Getting video info for: {}", file_path);

        let ffprobe_args = [
            "-i",
            file_path,
            "-show_entries",
            "stream=width,height,codec_name,bit_rate,r_frame_rate,duration",
            "-show_entries",
            "format=format_name,duration,bit_rate",
            "-of",
            "json",
        ];

        // 使用ffprobe获取视频信息
        let output = self.ffmpeg_service.execute_simple(&ffprobe_args).await?;

        // 注意：这里简化了实现，实际应该解析FFprobe的JSON输出
        // 为了演示，返回一个模拟的VideoInfo
        Ok(VideoInfo {
            width: 1920,
            height: 1080,
            duration: 120.5,
            format: "mp4".to_string(),
            bitrate: 5000,
            codec: "h264".to_string(),
            frame_rate: 30.0,
        })
    }

    /// 视频剪辑
    pub async fn clip_video<F>(&self, params: &ClipParams, progress_callback: F) -> Result<()>
    where
        F: FnMut(f32, Option<String>, Option<String>) + Send + Sync + 'static,
    {
        info!(
            "Clipping video: {} from {} to {}",
            params.input_path, params.start_time, params.end_time
        );

        // 构建FFmpeg剪辑命令
        let mut args: Vec<&str> = vec![
            "-i",
            &params.input_path,
            "-ss",
            &params.start_time.to_string(),
            "-to",
            &params.end_time.to_string(),
            "-b:v",
            &format!("{}k", params.bitrate),
        ];

        // 添加可选参数
        if let Some(codec) = &params.codec {
            args.extend_from_slice(&["-c:v", codec]);
        }

        if let Some(width) = params.width {
            if let Some(height) = params.height {
                args.extend_from_slice(&["-s", &format!("{}x{}", width, height)]);
            }
        }

        // 输出文件
        args.push(&params.output_path);

        // 执行FFmpeg命令
        self.ffmpeg_service
            .execute_with_progress(&args, progress_callback)
            .await
    }

    /// 视频合并
    pub async fn merge_videos<F>(&self, params: &MergeParams, progress_callback: F) -> Result<()>
    where
        F: FnMut(f32, Option<String>, Option<String>) + Send + Sync + 'static,
    {
        info!(
            "Merging videos: {:?} into {}",
            params.input_paths, params.output_path
        );

        // 对于少量文件，可以使用concat协议
        if params.input_paths.len() <= 10 {
            let mut args: Vec<&str> = vec!["-filter_complex"];

            // 构建concat过滤器参数
            let mut concat_param = String::from("concat=n=");
            concat_param.push_str(&params.input_paths.len().to_string());
            concat_param.push_str(":v=1:a=1");

            args.push(&concat_param);
            args.extend_from_slice(&["-b:v", &format!("{}k", params.bitrate)]);
            args.push(&params.output_path);

            // 添加输入文件
            for path in &params.input_paths {
                args.insert(1, "-i");
                args.insert(2, path);
            }

            // 执行FFmpeg命令
            return self
                .ffmpeg_service
                .execute_with_progress(&args, progress_callback)
                .await;
        }

        // 对于大量文件，应该使用文件列表方式
        // 这里简化实现，实际应该创建临时文件列表
        Err(anyhow!(
            "Merge for large number of files not implemented yet"
        ))
    }

    /// 格式转换
    pub async fn convert_format<F>(
        &self,
        input_path: &str,
        output_path: &str,
        format: &str,
        bitrate: u32,
        progress_callback: F,
    ) -> Result<()>
    where
        F: FnMut(f32, Option<String>, Option<String>) + Send + Sync + 'static,
    {
        info!("Converting {} to {} format", input_path, format);

        let args = [
            "-i",
            input_path,
            "-b:v",
            &format!("{}k", bitrate),
            "-f",
            format,
            output_path,
        ];

        self.ffmpeg_service
            .execute_with_progress(&args, progress_callback)
            .await
    }

    /// 提取音频
    pub async fn extract_audio<F>(
        &self,
        input_path: &str,
        output_path: &str,
        bitrate: u32,
        progress_callback: F,
    ) -> Result<()>
    where
        F: FnMut(f32, Option<String>, Option<String>) + Send + Sync + 'static,
    {
        info!("Extracting audio from {} to {}", input_path, output_path);

        let args = [
            "-i",
            input_path,
            "-vn", // 不包含视频
            "-acodec",
            "libmp3lame", // MP3编码
            "-b:a",
            &format!("{}k", bitrate),
            output_path,
        ];

        self.ffmpeg_service
            .execute_with_progress(&args, progress_callback)
            .await
    }
}
