import { create } from 'zustand';

// 文件项接口
export interface FileItem {
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

// Store状态接口
interface AppStore {
  // 文件列表状态
  fileList: FileItem[];
  setFileList: (files: FileItem[]) => void;
  addFile: (file: FileItem) => void;
  removeFile: (key: string) => void;
  updateFile: (key: string, updates: Partial<FileItem>) => void;
  clearFiles: () => void;
  moveFile: (key: string, direction: 'up' | 'down') => void;
  swapFiles: (key1: string, key2: string) => void;

  // 导出设置状态
  exportSettings: ExportSettings;
  setExportSettings: (settings: ExportSettings) => void;
  updateExportSetting: <K extends keyof ExportSettings>(key: K, value: ExportSettings[K]) => void;
}

// 创建Zustand store
export const useAppStore = create<AppStore>((set) => ({
  // 初始化文件列表
  fileList: [],

  // 文件列表操作方法
  setFileList: (files) => set({ fileList: files }),

  addFile: (file) => set((state) => ({
    fileList: [...state.fileList, file]
  })),

  removeFile: (key) => set((state) => ({
    fileList: state.fileList.filter(file => file.key !== key)
  })),

  updateFile: (key, updates) => set((state) => ({
    fileList: state.fileList.map(file =>
      file.key === key ? { ...file, ...updates } : file
    )
  })),

  clearFiles: () => set({ fileList: [] }),

  moveFile: (key, direction) => set((state) => {
    const newFileList = [...state.fileList];
    const index = newFileList.findIndex(file => file.key === key);

    if (index === -1) return { fileList: newFileList };

    const newIndex = direction === 'up' ? index - 1 : index + 1;

    if (newIndex < 0 || newIndex >= newFileList.length) {
      return { fileList: newFileList };
    }

    // 交换位置
    [newFileList[index], newFileList[newIndex]] = [newFileList[newIndex], newFileList[index]];

    // 更新order字段
    return {
      fileList: newFileList.map((file, idx) => ({ ...file, order: idx + 1 }))
    };
  }),

  swapFiles: (key1, key2) => set((state) => {
    const newFileList = [...state.fileList];
    const index1 = newFileList.findIndex(file => file.key === key1);
    const index2 = newFileList.findIndex(file => file.key === key2);

    if (index1 === -1 || index2 === -1) return { fileList: newFileList };

    // 交换位置
    [newFileList[index1], newFileList[index2]] = [newFileList[index2], newFileList[index1]];

    // 更新order字段
    return {
      fileList: newFileList.map((file, idx) => ({ ...file, order: idx + 1 }))
    };
  }),

  // 初始化导出设置
  exportSettings: {
    fileName: 'output',
    bitrate: 6,
    exportPath: '',
    mergeAudioTracks: false,
    useFirstVideoPath: true
  },

  // 导出设置操作方法
  setExportSettings: (settings) => set({ exportSettings: settings }),

  updateExportSetting: (key, value) => set((state) => ({
    exportSettings: { ...state.exportSettings, [key]: value }
  }))
}));
