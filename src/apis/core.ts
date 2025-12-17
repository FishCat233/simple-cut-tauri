import { invoke } from "@tauri-apps/api/core";
import { ExportSettings, VideoSlice } from "../types/export";

export function exportVideoSlices(slices: VideoSlice[], settings: ExportSettings): Promise<boolean> {
    // TODO: 导出视频切片参数验证和调整

    // 如果是 both 则进行两次导出
    const promises: Promise<boolean>[] = [];
    if (settings.audioMergeType === "both") {
        promises.push(invoke<boolean>("export", {
            slices,
            settings: { ...settings, audioMergeType: "merge" }
        }));
        promises.push(invoke<boolean>("export", {
            slices,
            settings: { ...settings, audioMergeType: "none" }
        }));
    } else {
        promises.push(invoke<boolean>("export", {
            slices,
            settings
        }));
    }

    return Promise.all(promises).then(results => results.every(result => result));
}