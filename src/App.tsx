import { useState } from "react";
import Nav, { NavConfig } from "./components/nav";
import "./App.css";
import IndexPage from "./pages";
import ExportPage from "./pages/export";
import GuidePage from "./pages/guide";
import AboutPage from "./pages/about";
import { Flex } from "antd";

// 定义文件项接口
export interface FileItem {
  key: string
  order: number
  fileName: string
  filePath?: string
  duration: string
  startTime: string
  endTime: string
}

function App() {
  const [activeNavId, setActiveNavId] = useState('index');
  // 提升文件列表状态到App组件
  const [fileList, setFileList] = useState<FileItem[]>([
    {
      key: '1',
      order: 1,
      fileName: '示例视频1.mp4',
      filePath: 'D:\\Videos\\示例视频1.mp4',
      duration: '00:10:30',
      startTime: '00:00:00',
      endTime: '00:01:00'
    },
    {
      key: '2',
      order: 2,
      fileName: '示例视频2.mp4',
      filePath: 'D:\\Videos\\示例视频2.mp4',
      duration: '00:05:20',
      startTime: '00:00:30',
      endTime: '00:02:15'
    }
  ]);

  // 处理导航项变化的回调函数
  const handleNavChange = (id: string) => {
    setActiveNavId(id);
    console.log('当前激活的导航项ID:', id);
    // 在这里可以根据需要执行其他逻辑
  };

  // 创建导航配置，传递props给页面组件
  const navConfig: NavConfig = [
    {
      title: '素材设置',
      id: 'index',
      page: () => <IndexPage fileList={fileList} setFileList={setFileList} />,
    },
    {
      title: '导出设置',
      id: 'export',
      page: () => <ExportPage fileList={fileList} />,
    },
    {
      title: "使用说明",
      id: 'guide',
      page: GuidePage,
    },
    {
      title: '关于',
      id: 'about',
      page: AboutPage,
    },
  ];

  return (
    <Flex vertical className="h-screen w-screen overflow-hidden">
      <Nav config={navConfig} onActiveChange={handleNavChange} />
      <div className="flex-1 overflow-auto p-4">
        {(() => {
          const Page = navConfig.find(item => item.id === activeNavId)?.page;
          return Page ? <Page /> : null;
        })()}
      </div>
    </Flex>
  );
}

export default App;
