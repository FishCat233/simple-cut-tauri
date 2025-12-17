import { z } from "zod";

export type SizeControlType =
  | "none" // 不控制码率
  | "mbps" // 按Mbps控制码率
  | "x264"; // 使用 x264 预设自动调整码率
export type audioMergeType =
  | "none" // 不合并音频
  | "amix" // 合并音频（使用 amix 滤镜）
  | "both"; // 生成合并和不合并两个视频

// 视频切片切片
export interface VideoSlice {
  key: string;
  order: number;
  fileName: string;
  filePath: string;
  startTime?: string;
  endTime?: string;
}

// 导出设置Zod Schema
export const exportSettingsSchema = z
  .object({
    // 导出文件名
    fileName: z.string().min(1, "请输入导出文件名"),
    // 码率，只有在sizeControlType为mbps时生效
    bitrate: z.number().optional(),
    // 码率控制类型
    sizeControlType: z.enum(["none", "mbps", "x264"]),
    // 音频合并类型
    audioMergeType: z.enum(["none", "amix", "both"]),
    // 导出路径
    exportPath: z.string().optional(),
    // 是否使用第一个视频路径作为导出路径
    useFirstVideoPath: z.boolean(),
  })
  .refine(
    (data) => {
      // 当码率控制类型为mbps时，码率必须存在且大于0
      if (data.sizeControlType === "mbps") {
        return data.bitrate !== undefined && data.bitrate > 0;
      }
      return true;
    },
    { message: "按Mbps控制码率时，必须输入有效的码率值", path: ["bitrate"] }
  )
  .refine(
    (data) => {
      // 当不使用第一个视频路径时，必须指定导出路径
      if (!data.useFirstVideoPath) {
        return data.exportPath !== undefined && data.exportPath.length > 0;
      }
      return true;
    },
    { message: "请选择导出路径", path: ["exportPath"] }
  );

// 导出设置接口
export type ExportSettings = z.infer<typeof exportSettingsSchema>;
