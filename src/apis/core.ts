import { invoke } from "@tauri-apps/api/core";
import { ExportSettings, VideoSlice } from "../types/export";

function exportVideoSlices(slices: VideoSlice[], settings: ExportSettings): Promise<boolean> {
    // TODO: 导出视频切片参数验证和调整
    return invoke<boolean>("export", {
        slices,
        settings
    });
}