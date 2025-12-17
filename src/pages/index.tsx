import { useState } from "react";
import { Table, Input, Button, Space, message } from "antd";
import type { ColumnType } from "antd/es/table";
import { useAppStore } from "../store";
import { VideoSlice } from "../types/export";
import { open } from "@tauri-apps/plugin-dialog";

function IndexPage() {
  // 从Zustand store获取文件列表和操作方法
  const {
    fileList,
    setFileList,
    updateFile,
    removeFile,
    clearFiles,
    moveFile,
    swapFiles,
  } = useAppStore();

  // 更新文件的开始时间或结束时间
  const updateFileTime = (
    key: string,
    field: "startTime" | "endTime",
    value: string
  ) => {
    updateFile(key, { [field]: value });
  };

  // 表格列配置
  const columns: ColumnType<VideoSlice>[] = [
    {
      title: "序号",
      dataIndex: "order",
      key: "order",
      width: 60,
    },
    {
      title: "文件名",
      dataIndex: "fileName",
      key: "fileName",
      width: 200,
    },
    {
      title: "文件时长",
      dataIndex: "duration",
      key: "duration",
      width: 120,
    },
    {
      title: "开始时间",
      dataIndex: "startTime",
      key: "startTime",
      width: 150,
      render: (text, record) => (
        <Input
          placeholder="00:00:00"
          value={text}
          onChange={(e) =>
            updateFileTime(record.key, "startTime", e.target.value)
          }
          size="small"
        />
      ),
    },
    {
      title: "结束时间",
      dataIndex: "endTime",
      key: "endTime",
      width: 150,
      render: (text, record) => (
        <Input
          placeholder="00:00:00"
          value={text}
          onChange={(e) =>
            updateFileTime(record.key, "endTime", e.target.value)
          }
          size="small"
        />
      ),
    },
  ];

  // 选中的行
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);

  // 表格选择配置
  const rowSelection = {
    selectedRowKeys,
    onChange: (newSelectedRowKeys: React.Key[]) => {
      setSelectedRowKeys(newSelectedRowKeys);
    },
  };

  // 添加文件
  const handleAddFile = async () => {
    // 打开文件选择对话框，允许选择多个文件
    let selected = await open({
      title: "选择视频文件",
      multiple: true,
    });

    if (!selected) {
      message.warning("请选择文件");
      return;
    }

    if (!Array.isArray(selected)) {
      selected = [selected];
    }

    // 添加选中的文件到文件列表
    selected.forEach((filePath) => {
      const newFile: VideoSlice = {
        key: Date.now().toString(),
        order: fileList.length + 1,
        fileName: filePath.split("\\").pop() || "新视频.mp4",
        filePath: filePath,
        startTime: "00:00:00",
        endTime: "00:00:00",
      };

      setFileList([...fileList, newFile]);
    });

    message.success("已添加新文件");
  };

  // 移除文件
  const handleRemoveFile = () => {
    if (selectedRowKeys.length === 0) {
      message.warning("请先选择要移除的文件");
      return;
    }

    selectedRowKeys.forEach((key) => removeFile(key.toString()));
    setSelectedRowKeys([]);
    message.success("已移除选中的文件");
  };

  // 向上移动
  const handleMoveUp = () => {
    if (selectedRowKeys.length !== 1) {
      message.warning("请选择一个文件进行移动");
      return;
    }

    const key = selectedRowKeys[0] as string;
    const index = fileList.findIndex((file) => file.key === key);

    if (index === 0) {
      message.warning("已经是第一个文件了");
      return;
    }

    moveFile(key, "up");
    message.success("文件已向上移动");
  };

  // 向下移动
  const handleMoveDown = () => {
    if (selectedRowKeys.length !== 1) {
      message.warning("请选择一个文件进行移动");
      return;
    }

    const key = selectedRowKeys[0] as string;
    const index = fileList.findIndex((file) => file.key === key);

    if (index === fileList.length - 1) {
      message.warning("已经是最后一个文件了");
      return;
    }

    moveFile(key, "down");
    message.success("文件已向下移动");
  };

  // 清除全部
  const handleClearAll = () => {
    if (fileList.length === 0) {
      message.info("文件列表已经是空的");
      return;
    }

    clearFiles();
    setSelectedRowKeys([]);
    message.success("已清除所有文件");
  };

  // 拖拽排序相关功能
  const [draggedKey, setDraggedKey] = useState<string | null>(null);
  const [dragOverKey, setDragOverKey] = useState<string | null>(null);

  const handleDragStart = (e: React.DragEvent, key: string) => {
    setDraggedKey(key);
    e.dataTransfer.effectAllowed = "move";
    e.dataTransfer.setData("text/plain", key); // 设置拖拽数据
  };

  // 拖拽结束
  const handleDragEnd = () => {
    setDraggedKey(null);
    setDragOverKey(null);
  };

  // 拖拽进入
  const handleDragEnter = (e: React.DragEvent, key: string) => {
    e.preventDefault();
    setDragOverKey(key);
  };

  // 允许放置
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  };

  // 拖拽离开
  const handleDragLeave = () => {
    setDragOverKey(null);
  };

  // 放置处理
  const handleDrop = (e: React.DragEvent, dropKey: string) => {
    e.preventDefault();
    setDragOverKey(null);

    if (!draggedKey || draggedKey === dropKey) {
      return;
    }

    // 使用store的swapFiles方法交换文件
    swapFiles(draggedKey, dropKey);
    setDraggedKey(null);
    message.success("文件顺序已更新");
  };

  // 表格行属性设置
  const rowProps = (record: VideoSlice) => ({
    draggable: true,
    onDragStart: (e: React.DragEvent) => handleDragStart(e, record.key),
    onDragEnd: handleDragEnd,
    onDragOver: handleDragOver,
    onDragEnter: (e: React.DragEvent) => handleDragEnter(e, record.key),
    onDragLeave: handleDragLeave,
    onDrop: (e: React.DragEvent) => handleDrop(e, record.key),
    style: {
      cursor: "move",
      opacity: draggedKey === record.key ? 0.5 : 1,
      backgroundColor: dragOverKey === record.key ? "#f0f0f0" : "transparent",
      transition: "all 0.2s ease",
    },
  });

  return (
    <div className="p-4">
      <h1 className="text-xl font-bold mb-4">素材设置</h1>

      {/* 表格 */}
      <Table
        rowSelection={rowSelection}
        columns={columns}
        dataSource={fileList}
        pagination={false}
        bordered
        className="mb-4 h-400px"
        scroll={{ y: 300 }}
        onRow={rowProps}
      />

      {/* 按钮区域 */}
      <div className="mt-4">
        <Space>
          <Button type="primary" onClick={handleAddFile}>
            添加文件
          </Button>
          <Button danger onClick={handleRemoveFile}>
            移除文件
          </Button>
          <Button onClick={handleMoveUp}>向上移动</Button>
          <Button onClick={handleMoveDown}>向下移动</Button>
          <Button danger onClick={handleClearAll}>
            清除全部
          </Button>
        </Space>
      </div>
    </div>
  );
}

export default IndexPage;
