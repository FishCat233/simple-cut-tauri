// 统一的类型定义文件，确保前后端数据结构一致

// 任务状态枚举
export enum TaskStatus {
  Pending = 'Pending',
  Running = 'Running',
  Completed = 'Completed',
  Failed = 'Failed',
  Cancelled = 'Cancelled'
}

// 任务类型枚举
export enum TaskType {
  ClipVideo = 'ClipVideo',
  MergeVideos = 'MergeVideos',
  ConvertFormat = 'ConvertFormat',
  ExtractAudio = 'ExtractAudio'
}

// 视频信息
export interface VideoInfo {
  width: number;
  height: number;
  duration: number; // 秒
  format: string;
  bitrate: number; // kbps
  codec: string;
  frame_rate: number;
}

// 任务进度信息
export interface TaskProgress {
  percentage: number;
  current: number;
  total: number;
  message?: string;
  error?: string;
}

// 任务信息
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

// 任务摘要
export interface TaskSummary {
  id: string;
  task_type: TaskType;
  status: TaskStatus;
  progress: number;
  created_at: string;
  output_path?: string;
}
