import { invoke } from "@tauri-apps/api/core";
import { ExportSettings, VideoSlice } from "../types/export";

export async function exportVideoSlices(slices: VideoSlice[], settings: ExportSettings): Promise<boolean> {
    // TODO: 导出视频切片参数验证和调整

    // 如果是 both 则进行两次导出
    if (settings.audioMergeType === "both") {
        await invoke<boolean>("export", {
            slices,
            settings: { ...settings, audioMergeType: "merge" }
        });
        await invoke<boolean>("export", {
            slices,
            settings: { ...settings, audioMergeType: "none" }
        });
    } else {
        await invoke<boolean>("export", {
            slices,
            settings
        });
    }

    return true;
}