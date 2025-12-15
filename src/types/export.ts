// TODO: 参数验证

type SizeControlType = 'none' | // 不控制码率
    'mbps' | // 按Mbps控制码率
    'x264' // 使用 x264 预设自动调整码率
type audioMergeType = 'none' | // 不合并音频
    'amix' | // 合并音频（使用 amix 滤镜）
    'both' // 生成合并和不合并两个视频

// 视频切片切片
export interface VideoSlice {
    key: string;
    order: number;
    fileName: string;
    filePath: string;
    startTime?: string;
    endTime?: string;
}


// 导出设置接口
export interface ExportSettings {
    // 导出文件名
    fileName: string;
    // 码率，只有在sizeControlType为mbps时生效
    bitrate: number;
    // 码率控制类型
    sizeControlType: SizeControlType;
    // 音频合并类型
    audioMergeType: audioMergeType;
    // 导出路径
    exportPath: string;
    // 是否使用第一个视频路径作为导出路径
    useFirstVideoPath: boolean;
}

