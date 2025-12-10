
// 视频切片切片
export interface VideoSlice {
    key: string;
    order: number;
    fileName: string;
    filePath: string;
    startTime?: string;
    endTime?: string;
    outputPath?: string;
}

// 导出设置接口
export interface ExportSettings {
    fileName: string;
    bitrate: number;
    exportPath: string;
    mergeAudioTracks: boolean;
    useFirstVideoPath: boolean;
}
