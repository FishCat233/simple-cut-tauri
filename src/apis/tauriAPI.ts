import { invoke } from "@tauri-apps/api/core";
import { ExportSettings, VideoSlice } from "./types";

function exportVideoClips(clips: VideoSlice[], settings: ExportSettings): Promise<boolean> {
    return invoke<boolean>("export_video_clips", {
        clips,
        settings
    });
}