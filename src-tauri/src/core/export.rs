use std::process::Command;

use serde::{Deserialize, Serialize};
use tauri::ipc::InvokeError;

use crate::core::filter_builder::FilterBuilder;
use crate::core::path::{get_ffmpeg_path, get_ffprobe_path};
use crate::media_info::tools::get_audio_track_count;
use shlex;

// 大小控制类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SizeControlType {
    None,
    Mbps,
    X264,
}

// 音频合并类型
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AudioMergeType {
    None,
    Amix,
    Both,
}

// 视频切片信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoSlice {
    pub key: String,
    pub order: u32,
    pub file_name: String,
    pub file_path: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

// 导出设置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportSettings {
    pub file_name: String,
    pub bitrate: u32,
    pub size_control_type: SizeControlType,
    pub audio_merge_type: AudioMergeType,
    pub export_path: String,
    pub use_first_video_path: bool,
}

/// 导出视频切片
///
/// # 参数
///
/// * `handle` - Tauri应用句柄
/// * `slices` - 视频切片信息列表
/// * `settings` - 导出设置
#[tauri::command]
pub fn export(
    handle: tauri::AppHandle,
    slices: Vec<VideoSlice>,
    settings: ExportSettings,
) -> Result<(), InvokeError> {
    // TODO: 测试

    let ffmpeg_path = get_ffmpeg_path(&handle);
    let ffprobe_path = get_ffprobe_path(&handle);

    // 命令头
    let mut command = build_command_header(&ffmpeg_path);

    // 导入视频切片
    for video_slice in &slices {
        command.push_str(&build_video_input(video_slice));
    }

    let filter_complex = build_filter_complex(&ffprobe_path, &slices, &settings.audio_merge_type);

    match filter_complex {
        Ok(filter) => command.push_str(&filter),
        Err(e) => {
            log::error!("build filter complex error: {:?}", e);
            return Err(InvokeError::from_anyhow(e));
        }
    }

    // 输出尾
    command.push_str(&build_command_tail(&settings.export_path, &settings));

    log::info!("export one with command: {}", command);

    // 解析命令
    let args = match shlex::split(&command) {
        None => {
            return Err(InvokeError::from_anyhow(anyhow::anyhow!(
                "split command failed"
            )));
        }
        Some(args) => args,
    };

    let (program, args) = args.split_first().unwrap();

    // 执行命令
    let exit_status = Command::new(program)
        .args(args)
        .status()
        .map_err(|e| InvokeError::from_anyhow(e.into()))?;

    if exit_status.success() {
        log::info!("export one success");
        Ok(())
    } else {
        log::error!("export one failed with exit status: {:?}", exit_status);
        Err(InvokeError::from_anyhow(anyhow::anyhow!(
            "export one failed with exit status: {:?}",
            exit_status
        )))
    }
}

/// 构建命令头
///
/// # 参数
///
/// * `ffmpeg_path` - ffmpeg可执行文件路径
fn build_command_header(ffmpeg_path: &str) -> String {
    ffmpeg_path.to_string() + " -y"
}

/// 构建参数头 (不带执行文件路径的命令头)
///
/// # 返回值
///
/// 不带执行文件路径的命令头字符串
#[allow(dead_code)]
fn build_command_header_without_executeable() -> String {
    " -y".to_string()
}

/// 构建视频输入参数
///
/// # 参数
///
/// * `video_slice` - 视频切片信息
///
/// # 返回值
///
/// 视频输入参数字符串
fn build_video_input(video_slice: &VideoSlice) -> String {
    let start_time_string: String = match &video_slice.start_time {
        Some(time) => format!(" -ss {}", time),
        None => "".to_string(),
    };

    let end_time_string: String = match &video_slice.end_time {
        Some(time) => format!(" -to {}", time),
        None => "".to_string(),
    };

    format!(
        "{}{} -i {}",
        start_time_string, end_time_string, video_slice.file_path
    )
}

/// 构建完整的filter_complex滤镜链
///
/// # 参数
///
/// * `ffprobe_path` - ffprobe可执行文件路径
/// * `slices` - 视频切片信息列表
/// * `audio_merge_type` - 音频合并类型
fn build_filter_complex(
    ffprobe_path: &str,
    slices: &[VideoSlice],
    audio_merge_type: &AudioMergeType,
) -> anyhow::Result<String> {
    let mut concat_inputs = Vec::new();

    // 构建滤镜
    let mut filter_builder = FilterBuilder::new();

    // 遍历处理音频滤镜
    for (index, slice) in slices.iter().enumerate() {
        // 添加视频流到concat输入
        concat_inputs.push(format!("{}:v", index));

        // 添加音频流到concat输入
        let audio_track_count = get_audio_track_count(ffprobe_path, &slice.file_path)?;
        if audio_track_count > 0 && *audio_merge_type == AudioMergeType::Amix {
            // 多音轨且需要合并
            let output_alias = format!("{}a", index); // 为每个音频流创建唯一别名

            // 添加滤镜
            filter_builder.add_merge_amix_filter(index, audio_track_count, &output_alias);

            // 添加合并后的音频流到concat输入
            concat_inputs.push(output_alias);
        } else {
            // 单音轨或不需要合并
            concat_inputs.push(format!("{}:a", index));
        }
    }

    filter_builder.add_concat_filter(&concat_inputs, "v", "a");

    let mut filter_complex = filter_builder.build_to_string();

    if !filter_complex.is_empty() {
        filter_complex.push_str(" -map [v] -map [a]"); // 和上面的对应，映射滤镜输出到输出文件
    }

    Ok(filter_complex)
}

/// 构建命令尾
///
/// # 参数
///
/// * `output_path` - 导出路径
/// * `settings` - 导出设置
fn build_command_tail(output_path: &str, settings: &ExportSettings) -> String {
    match &settings.size_control_type {
        SizeControlType::None => format!(" \"{}\"", output_path),
        SizeControlType::Mbps => format!(" -b:v {}M \"{}\"", settings.bitrate, output_path),
        SizeControlType::X264 => format!(" -c:v libx264 -crf 23.5 -preset veryslow -keyint_min 600 -g 600 -refs 4 -bf 3 -me_method umh -sc_threshold 60 -b_strategy 1 -qcomp 0.5 -psy-rd 0.3:0 -aq-mode 2 -aq-strength 0.8 -c:a aac -b:a 128k -movflags faststart \"{}\"", output_path),
    }
}
