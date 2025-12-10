use anyhow::{anyhow, Result};
use log::{debug, error};
use std::process::Stdio;
use tauri::AppHandle;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// FFmpeg服务，负责调用FFmpeg命令行工具
pub struct FfmpegService {
    ffmpeg_path: String,
    app_handle: AppHandle,
}

impl FfmpegService {
    /// 创建新的FFmpeg服务实例
    pub fn new(app_handle: AppHandle, ffmpeg_path: Option<String>) -> Self {
        // 如果没有提供FFmpeg路径，使用默认名称（假设已在PATH中或作为sidecar）
        let path = ffmpeg_path.unwrap_or_else(|| String::from("ffmpeg"));
        FfmpegService {
            ffmpeg_path: path,
            app_handle: app_handle,
        }
    }

    /// 构建FFmpeg命令参数
    pub fn build_command(&self, args: &[&str]) -> Result<Vec<String>> {
        let mut cmd_args: Vec<String> = vec![
            "-hide_banner".to_string(), // 隐藏横幅
            "-y".to_string(),           // 覆盖输出文件
        ];

        cmd_args.extend(args.iter().map(|s| s.to_string()));
        Ok(cmd_args)
    }

    /// 执行FFmpeg命令（简单版本）
    pub async fn execute_simple(&self, args: &[&str]) -> Result<()> {
        let cmd_args = self.build_command(args)?;
        debug!(
            "Executing FFmpeg command: {} {:?}",
            self.ffmpeg_path, cmd_args
        );

        let output = Command::new(&self.ffmpeg_path)
            .args(&cmd_args)
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("FFmpeg command failed: {}", stderr);
            return Err(anyhow!("FFmpeg error: {}", stderr));
        }

        Ok(())
    }

    /// 执行FFmpeg命令并实时获取进度
    pub async fn execute_with_progress<F>(&self, args: &[&str], progress_callback: F) -> Result<()>
    where
        F: FnMut(f64, Option<String>, Option<String>) + Send + Sync + 'static,
    {
        let cmd_args = self.build_command(args)?;
        debug!(
            "Executing FFmpeg command with progress: {} {:?}",
            self.ffmpeg_path, cmd_args
        );

        // 启动FFmpeg进程
        let mut child = Command::new(&self.ffmpeg_path)
            .args(&cmd_args)
            .stderr(Stdio::piped())
            .spawn()?;

        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow!("Failed to capture stderr"))?;
        let mut reader = BufReader::new(stderr).lines();

        // 进度回调闭包
        let mut callback = progress_callback;

        // 读取FFmpeg输出
        while let Some(line) = reader.next_line().await? {
            debug!("FFmpeg output: {}", line);

            // 解析进度信息
            if let Some((progress, speed, eta)) = self.parse_progress(&line) {
                callback(progress, speed, eta);
            }

            // 检查错误信息
            if line.contains("error") || line.contains("Error") {
                error!("FFmpeg error: {}", line);
                return Err(anyhow!("FFmpeg error: {}", line));
            }
        }

        // 等待进程结束
        let status = child.wait().await?;
        if !status.success() {
            return Err(anyhow!(
                "FFmpeg process exited with code: {}",
                status.code().unwrap_or(-1)
            ));
        }

        // 任务完成，设置100%进度
        callback(100.0, None, None);

        Ok(())
    }

    /// 解析FFmpeg输出中的进度信息
    fn parse_progress(&self, line: &str) -> Option<(f64, Option<String>, Option<String>)> {
        // FFmpeg进度格式示例: frame= 1000 fps= 25 q=-1.0 size=   10240kB time=00:00:40.00 bitrate=2048.0kbits/s speed=1.0x
        if !line.starts_with("frame=") {
            return None;
        }

        let mut progress = 0.0;
        let mut speed: Option<String> = None;
        let mut eta: Option<String> = None;
        let mut total_duration = 0.0;
        let mut current_time = 0.0;

        // 解析输出行中的各个字段
        let parts: Vec<&str> = line.split_whitespace().collect();
        for i in 0..parts.len() {
            if parts[i].starts_with("time=") {
                // 解析时间，格式为 HH:MM:SS.ms
                let time_str = &parts[i][5..];
                if let Ok(t) = Self::parse_time(time_str) {
                    current_time = t;
                }
            } else if parts[i].starts_with("speed=") {
                speed = Some(parts[i][6..].to_string());
            } else if parts[i].starts_with("eta=") {
                eta = Some(parts[i][4..].to_string());
            }
        }

        // 如果知道总时长，可以计算进度百分比
        // 注意：这里需要额外的逻辑来获取总时长
        // 简单实现：假设总时长已知，这里简化处理
        // TODO:
        if total_duration > 0.0 {
            progress = (current_time / total_duration) * 100.0;
            Some((progress, speed, eta))
        } else {
            // 如果不知道总时长，返回0%进度
            Some((0.0, speed, eta))
        }
    }

    /// 解析FFmpeg时间格式（HH:MM:SS.ms）
    fn parse_time(time_str: &str) -> Result<f64> {
        let parts: Vec<&str> = time_str.split(":").collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid time format: {}", time_str));
        }

        let hours = parts[0].parse::<f64>()?;
        let minutes = parts[1].parse::<f64>()?;
        let seconds = parts[2].parse::<f64>()?;

        Ok(hours * 3600.0 + minutes * 60.0 + seconds)
    }

    /// 获取视频信息
    pub async fn get_video_info(&self, file_path: &str) -> Result<serde_json::Value> {
        let cmd_args = vec![
            "-i",
            file_path,
            "-show_entries",
            "stream=width,height,codec_name,bit_rate,r_frame_rate,duration",
            "-show_entries",
            "format=format_name,duration,size,bit_rate",
            "-of",
            "json",
        ];

        debug!("Getting video info: {} {:?}", self.ffmpeg_path, cmd_args);

        let output = Command::new(&self.ffmpeg_path)
            .args(cmd_args)
            .stderr(Stdio::null())
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to get video info"));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let info: serde_json::Value = serde_json::from_str(&stdout)?;

        Ok(info)
    }
}
