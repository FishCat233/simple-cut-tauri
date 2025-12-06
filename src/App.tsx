import { useState } from "react";
import Nav, { NavConfig } from "./components/nav";
import "./App.css";
import IndexPage from "./pages";
import ExportPage from "./pages/export";
import GuidePage from "./pages/guide";
import AboutPage from "./pages/about";
import { Flex } from "antd";

const navConfig: NavConfig = [
  {
    title: '素材设置',
    id: 'index',
    page: IndexPage,
  },
  {
    title: '导出设置',
    id: 'export',
    page: ExportPage,
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
]

function App() {
  const [activeNavId, setActiveNavId] = useState('index');

  // 处理导航项变化的回调函数
  const handleNavChange = (id: string) => {
    setActiveNavId(id);
    console.log('当前激活的导航项ID:', id);
    // 在这里可以根据需要执行其他逻辑
  };

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
