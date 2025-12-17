import { create } from "zustand";
import { ExportSettings, VideoSlice } from "./types/export";

// Store状态接口
interface AppStore {
  // 文件列表状态
  fileList: VideoSlice[];
  setFileList: (files: VideoSlice[]) => void;
  addFile: (file: VideoSlice) => void;
  appendFileByPaths: (paths: string[]) => void;
  removeFile: (key: string) => void;
  updateFile: (key: string, updates: Partial<VideoSlice>) => void;
  clearFiles: () => void;
  moveFile: (key: string, direction: "up" | "down") => void;
  swapFiles: (key1: string, key2: string) => void;

  // 导出设置状态
  exportSettings: ExportSettings;
  setExportSettings: (settings: ExportSettings) => void;
  updateExportSetting: <K extends keyof ExportSettings>(
    key: K,
    value: ExportSettings[K]
  ) => void;
}

// 创建Zustand store
export const useAppStore = create<AppStore>((set) => ({
  // 初始化文件列表
  fileList: [],

  // 文件列表操作方法
  setFileList: (files) => set({ fileList: files }),

  addFile: (file) =>
    set((state) => {
      // 确保file有唯一key
      const hasKey = file.key && file.key.length > 0;
      const keyExists =
        hasKey && state.fileList.some((f) => f.key === file.key);

      if (!hasKey || keyExists) {
        // 生成唯一key
        const uniqueKey = `${
          file.filePath || "video"
        }_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        return {
          fileList: [...state.fileList, { ...file, key: uniqueKey }],
        };
      }

      return {
        fileList: [...state.fileList, file],
      };
    }),

  appendFileByPaths: (paths: string[]) =>
    set((state) => {
      const currentLength = state.fileList.length;
      return {
        fileList: [
          ...state.fileList,
          ...paths.map((filePath, index) => ({
            key: `video_${Date.now()}_${index}_${Math.random()
              .toString(36)
              .substring(2, 11)}`,
            order: currentLength + index + 1,
            fileName: filePath.split(/[\/]/).pop() || "unknown",
            filePath,
            startTime: undefined,
            endTime: undefined,
          })),
        ],
      };
    }),

  removeFile: (key) =>
    set((state) => ({
      fileList: state.fileList.filter((file) => file.key !== key),
    })),

  updateFile: (key, updates) =>
    set((state) => ({
      fileList: state.fileList.map((file) =>
        file.key === key ? { ...file, ...updates } : file
      ),
    })),

  clearFiles: () => set({ fileList: [] }),

  moveFile: (key, direction) =>
    set((state) => {
      const newFileList = [...state.fileList];
      const index = newFileList.findIndex((file) => file.key === key);

      if (index === -1) return { fileList: newFileList };

      const newIndex = direction === "up" ? index - 1 : index + 1;

      if (newIndex < 0 || newIndex >= newFileList.length) {
        return { fileList: newFileList };
      }

      // 交换位置
      [newFileList[index], newFileList[newIndex]] = [
        newFileList[newIndex],
        newFileList[index],
      ];

      // 更新order字段
      return {
        fileList: newFileList.map((file, idx) => ({ ...file, order: idx + 1 })),
      };
    }),

  swapFiles: (key1, key2) =>
    set((state) => {
      const newFileList = [...state.fileList];
      const index1 = newFileList.findIndex((file) => file.key === key1);
      const index2 = newFileList.findIndex((file) => file.key === key2);

      if (index1 === -1 || index2 === -1) return { fileList: newFileList };

      // 交换位置
      [newFileList[index1], newFileList[index2]] = [
        newFileList[index2],
        newFileList[index1],
      ];

      // 更新order字段
      return {
        fileList: newFileList.map((file, idx) => ({ ...file, order: idx + 1 })),
      };
    }),

  // 初始化导出设置
  exportSettings: {
    fileName: "",
    bitrate: 6,
    sizeControlType: "mbps",
    audioMergeType: "none",
    exportPath: "",
    useFirstVideoPath: true,
  },

  // 导出设置操作方法
  setExportSettings: (settings) => set({ exportSettings: settings }),

  updateExportSetting: (key, value) =>
    set((state) => ({
      exportSettings: { ...state.exportSettings, [key]: value },
    })),
}));
