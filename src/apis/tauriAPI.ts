import { invoke } from '@tauri-apps/api/core';
import { FileItem, ExportSettings } from '../store';

export interface VideoInfo {
  width: number;
  height: number;
  duration: number; // 秒
  format: string;
  bitrate: number; // kbps
  codec: string;
  frame_rate: number;
}

// 后端任务状态枚举
export enum TaskStatus {
  Pending = 'Pending',
  Running = 'Running',
  Completed = 'Completed',
  Failed = 'Failed',
  Cancelled = 'Cancelled'
}

// 后端任务类型枚举
export enum TaskType {
  ClipVideo = 'ClipVideo',
  MergeVideos = 'MergeVideos',
  ConvertFormat = 'ConvertFormat',
  ExtractAudio = 'ExtractAudio'
}

// 后端任务进度信息
export interface TaskProgress {
  percentage: number;
  current: number;
  total: number;
  message?: string;
  error?: string;
}

// 后端任务信息
export interface TaskInfo {
  id: string;
  task_type: TaskType;
  status: TaskStatus;
  progress: TaskProgress;
  created_at: string;
  started_at?: string;
  completed_at?: string;
  input_paths: string[];
  output_path?: string;
}

// 后端任务摘要
export interface TaskSummary {
  id: string;
  task_type: TaskType;
  status: TaskStatus;
  progress: number;
  created_at: string;
  output_path?: string;
}

// 后端剪辑参数接口
interface ClipParams {
  input_path: string;
  output_path: string;
  start_time: number; // 秒
  end_time: number;   // 秒
  bitrate: number;    // kbps
  format: string;
  codec?: string;
  width?: number;
  height?: number;
}

// 后端合并参数接口
interface MergeParams {
  input_paths: string[];
  output_path: string;
  format: string;
  bitrate: number;
}

// 将时间字符串转换为秒数（支持 mm:ss 格式）
function timeStringToSeconds(timeStr: string): number {
  if (!timeStr) return 0;

  const parts = timeStr.split(':').map(Number);
  if (parts.length === 2) {
    return parts[0] * 60 + parts[1];
  } else if (parts.length === 3) {
    return parts[0] * 3600 + parts[1] * 60 + parts[2];
  }
  return Number(timeStr) || 0;
}

// 获取视频信息
export async function getVideoInfo(filePath: string): Promise<VideoInfo> {
  return invoke<VideoInfo>('get_video_info', { file_path: filePath });
}

// 创建单个剪辑任务
export async function createClipTask(
  file: FileItem,
  exportSettings: ExportSettings
): Promise<string> {
  const params: ClipParams = {
    input_path: file.filePath,
    output_path: file.outputPath || exportSettings.exportPath,
    start_time: timeStringToSeconds(file.startTime || '0'),
    end_time: timeStringToSeconds(file.endTime || '0'),
    bitrate: exportSettings.bitrate * 1000, // 转换为 kbps
    format: 'mp4', // 默认格式
    codec: undefined,
    width: undefined,
    height: undefined
  };

  return invoke<string>('create_clip_task', { params });
}

// 创建合并任务
export async function createMergeTask(
  files: FileItem[],
  exportSettings: ExportSettings
): Promise<string> {
  const params: MergeParams = {
    input_paths: files.map(file => file.outputPath || file.filePath),
    output_path: exportSettings.exportPath,
    format: 'mp4', // 默认格式
    bitrate: exportSettings.bitrate * 1000 // 转换为 kbps
  };

  return invoke<string>('create_merge_task', { params });
}

// 启动任务
export async function startTask(taskId: string): Promise<void> {
  return invoke<void>('start_task', { task_id_str: taskId });
}

// 取消任务
export async function cancelTask(taskId: string): Promise<void> {
  return invoke<void>('cancel_task', { task_id_str: taskId });
}

// 获取任务状态
export async function getTaskStatus(taskId: string): Promise<TaskInfo | null> {
  return invoke<TaskInfo | null>('get_task_status', { task_id_str: taskId });
}

// 获取所有任务
export async function getAllTasks(): Promise<TaskSummary[]> {
  return invoke<TaskSummary[]>('get_all_tasks');
}

// 清理任务
export async function cleanupTasks(days: number): Promise<number> {
  return invoke<number>('cleanup_tasks', { days });
}

// 测试通信
export async function ping(): Promise<string> {
  return invoke<string>('ping');
}
