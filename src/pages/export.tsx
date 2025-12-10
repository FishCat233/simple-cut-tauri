
import { useState } from 'react';
import { Flex, Form, Input, Button, Checkbox, Alert, Space, Card, InputNumber } from 'antd';
import { MessageOutlined, DownloadOutlined } from '@ant-design/icons';
import { useAppStore } from '../store';

function ExportPage() {
  // 从Zustand store获取导出设置和操作方法
  const { fileList, exportSettings, setExportSettings } = useAppStore();

  // 表单状态
  const [form] = Form.useForm();

  // 导出状态
  const [exporting, setExporting] = useState(false);

  // 当表单值变化时更新store
  const handleValuesChange = (_: any, values: any) => {
    setExportSettings(values as any);
  };



  // 导出处理函数
  const handleExport = () => {
    form.validateFields()
      .then(values => {
        console.log('导出参数:', values);
        setExporting(true);

        // 模拟导出过程
        setTimeout(() => {
          setExporting(false);
          alert('导出成功！');
        }, 2000);
      })
      .catch(info => {
        console.log('表单验证失败:', info);
      });
  };

  // 选择文件路径函数（模拟）
  const handleSelectPath = () => {
    // 实际应用中，这里应该使用Tauri的文件选择API
    form.setFieldsValue({ exportPath: 'D:\\导出目录' });
  };

  // 处理默认路径选项变化
  const handleUseFirstVideoPath = (checked: boolean) => {
    if (checked && fileList.length > 0 && fileList[0].filePath) {
      // 提取第一个视频的目录路径作为导出路径
      const firstFilePath = fileList[0].filePath;
      const exportPath = firstFilePath.substring(0, firstFilePath.lastIndexOf('\\'));
      form.setFieldsValue({ exportPath });
    }
  };

  return (
    <Flex vertical className="p-4 w-full">
      <h1 className="text-xl font-bold mb-4">导出设置</h1>

      {/* 提示框 */}
      <Alert
        title="导出提示"
        description="请确保已完成素材设置，导出过程中请勿关闭应用程序。"
        type="info"
        showIcon
        icon={<MessageOutlined />}
        className="mb-4"
      />

      <Card className="mb-4">
        <Form
          form={form}
          layout="vertical"
          initialValues={exportSettings}
          onValuesChange={handleValuesChange}
        >
          {/* 导出文件名 */}
          <Form.Item
            name="fileName"
            label="导出文件名"
            rules={[{ required: true, message: '请输入导出文件名' }]}
          >
            <Input placeholder="请输入导出文件名" />
          </Form.Item>

          {/* 导出码率 */}
          <Form.Item
            name="bitrate"
            label="导出码率"
            rules={[
              { required: true, message: '请输入导出码率' },
              { type: 'number', min: 0.5, message: '码率不能低于0.5 mbps' },
              { validator: (_, value) => value && value > 1000 ? Promise.reject(new Error('码率不能超过1000 mbps')) : Promise.resolve() }
            ]}
          >
            <InputNumber
              placeholder="请输入导出码率 (mbps)"
              suffix="mbps"
              style={{ width: '100%' }}
              min={0.5}
              max={1000}
              step={1}
              changeOnWheel
            />
          </Form.Item>

          {/* 导出路径 */}
          <Form.Item
            name="exportPath"
            label="导出路径"
            rules={[{ required: true, message: '请选择导出路径' }]}
          >
            <Space.Compact style={{ width: '100%' }}>
              <Input placeholder="请选择导出路径" readOnly />
              <Button type="primary" onClick={handleSelectPath}>浏览</Button>
            </Space.Compact>
          </Form.Item>

          {/* 默认第一个视频路径为导出路径 */}
          <Form.Item
            name="useFirstVideoPath"
            valuePropName="checked"
          >
            <Checkbox onChange={(e) => handleUseFirstVideoPath(e.target.checked)}
              disabled={fileList.length === 0}
            >
              默认第一个视频路径为导出路径
            </Checkbox>
          </Form.Item>

          {/* 合并多音轨 */}
          <Form.Item
            name="mergeAudioTracks"
            valuePropName="checked"
          >
            <Checkbox>合并多音轨</Checkbox>
          </Form.Item>
        </Form>
      </Card>

      {/* 导出按钮 */}
      <Button
        type="primary"
        size="large"
        icon={<DownloadOutlined />}
        onClick={handleExport}
        loading={exporting}
        block
      >
        {exporting ? '导出中...' : '导出'}
      </Button>
    </Flex>
  )
}

export default ExportPage