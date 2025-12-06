
import { useState } from 'react'
import { Table, Input, Button, Space, message, Flex } from 'antd'
import type { ColumnType } from 'antd/es/table'

interface FileItem {
  key: string
  order: number
  fileName: string
  duration: string
  startTime: string
  endTime: string
}

function IndexPage() {
  // 表格数据状态
  const [fileList, setFileList] = useState<FileItem[]>([
    {
      key: '1',
      order: 1,
      fileName: '示例视频1.mp4',
      duration: '00:10:30',
      startTime: '00:00:00',
      endTime: '00:01:00'
    },
    {
      key: '2',
      order: 2,
      fileName: '示例视频2.mp4',
      duration: '00:05:20',
      startTime: '00:00:30',
      endTime: '00:02:15'
    }
  ])



  // 更新文件的开始时间或结束时间
  const updateFileTime = (key: string, field: 'startTime' | 'endTime', value: string) => {
    setFileList(prevList =>
      prevList.map(file =>
        file.key === key ? { ...file, [field]: value } : file
      )
    )
  }

  // 表格列配置
  const columns: ColumnType<FileItem>[] = [
    {
      title: '序号',
      dataIndex: 'order',
      key: 'order',
      width: 60
    },
    {
      title: '文件名',
      dataIndex: 'fileName',
      key: 'fileName',
      width: 200
    },
    {
      title: '文件时长',
      dataIndex: 'duration',
      key: 'duration',
      width: 120
    },
    {
      title: '开始时间',
      dataIndex: 'startTime',
      key: 'startTime',
      width: 150,
      render: (text, record) => (
        <Input
          placeholder="00:00:00"
          value={text}
          onChange={(e) => updateFileTime(record.key, 'startTime', e.target.value)}
          size="small"
        />
      )
    },
    {
      title: '结束时间',
      dataIndex: 'endTime',
      key: 'endTime',
      width: 150,
      render: (text, record) => (
        <Input
          placeholder="00:00:00"
          value={text}
          onChange={(e) => updateFileTime(record.key, 'endTime', e.target.value)}
          size="small"
        />
      )
    }
  ]

  // 选中的行
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([])

  // 表格选择配置
  const rowSelection = {
    selectedRowKeys,
    onChange: (newSelectedRowKeys: React.Key[]) => {
      setSelectedRowKeys(newSelectedRowKeys)
    }
  }

  // 添加文件
  const handleAddFile = () => {
    // 示例：添加一个新文件，序号自动加1
    const newFile: FileItem = {
      key: Date.now().toString(),
      order: fileList.length + 1,
      fileName: `新视频${fileList.length + 1}.mp4`,
      duration: '00:00:00',
      startTime: '00:00:00',
      endTime: '00:00:00'
    }

    setFileList([...fileList, newFile])
    message.success('已添加新文件')
    // 实际实现时，这里可以打开文件选择对话框
  }

  // 移除文件
  const handleRemoveFile = () => {
    if (selectedRowKeys.length === 0) {
      message.warning('请先选择要移除的文件')
      return
    }

    const newFileList = fileList.filter(file => !selectedRowKeys.includes(file.key))

    // 重新排序序号
    const sortedList = newFileList.map((file, index) => ({
      ...file,
      order: index + 1
    }))

    setFileList(sortedList)
    setSelectedRowKeys([])
    message.success('已移除选中的文件')
  }

  // 向上移动
  const handleMoveUp = () => {
    if (selectedRowKeys.length !== 1) {
      message.warning('请选择一个文件进行移动')
      return
    }

    const key = selectedRowKeys[0] as string
    const index = fileList.findIndex(file => file.key === key)

    if (index === 0) {
      message.warning('已经是第一个文件了')
      return
    }

    const newFileList = [...fileList]
    // 交换位置
    const temp = newFileList[index]
    newFileList[index] = newFileList[index - 1]
    newFileList[index - 1] = temp

    // 更新序号
    newFileList[index].order = index + 1
    newFileList[index - 1].order = index

    setFileList(newFileList)
    message.success('文件已向上移动')
  }

  // 向下移动
  const handleMoveDown = () => {
    if (selectedRowKeys.length !== 1) {
      message.warning('请选择一个文件进行移动')
      return
    }

    const key = selectedRowKeys[0] as string
    const index = fileList.findIndex(file => file.key === key)

    if (index === fileList.length - 1) {
      message.warning('已经是最后一个文件了')
      return
    }

    const newFileList = [...fileList]
    // 交换位置
    const temp = newFileList[index]
    newFileList[index] = newFileList[index + 1]
    newFileList[index + 1] = temp

    // 更新序号
    newFileList[index].order = index + 1
    newFileList[index + 1].order = index + 2

    setFileList(newFileList)
    message.success('文件已向下移动')
  }

  // 清除全部
  const handleClearAll = () => {
    if (fileList.length === 0) {
      message.info('文件列表已经是空的')
      return
    }

    setFileList([])
    setSelectedRowKeys([])
    message.success('已清除所有文件')
  }

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
      />

      {/* 按钮区域 */}
      <div className="mt-4">
        <Space>
          <Button type="primary" onClick={handleAddFile}>添加文件</Button>
          <Button danger onClick={handleRemoveFile}>移除文件</Button>
          <Button onClick={handleMoveUp}>向上移动</Button>
          <Button onClick={handleMoveDown}>向下移动</Button>
          <Button danger onClick={handleClearAll}>清除全部</Button>
        </Space>
      </div>
    </div>
  )
}

export default IndexPage