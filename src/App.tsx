import { useState } from "react";
import Nav, { NavConfig } from "./components/nav";
import "./App.css";
import IndexPage from "./pages";
import ExportPage from "./pages/export";
// import GuidePage from "./pages/guide"; // 暂时未使用
import AboutPage from "./pages/about";
import { Flex } from "antd";
import GuidePage from "./pages/guide";
import DragAndDrop from "./components/dragAndDrop";

function App() {
  const [activeNavId, setActiveNavId] = useState("index");

  // 处理导航项变化的回调函数
  const handleNavChange = (id: string) => {
    setActiveNavId(id);
    console.log("当前激活的导航项ID:", id);
    // 在这里可以根据需要执行其他逻辑
  };

  // 创建导航配置，传递props给页面组件
  const navConfig: NavConfig = [
    {
      title: "素材设置",
      id: "index",
      page: () => <IndexPage />,
    },
    {
      title: "导出设置",
      id: "export",
      page: () => <ExportPage />,
    },
    {
      title: "使用说明",
      id: "guide",
      page: GuidePage,
    },
    {
      title: "关于软件",
      id: "about",
      page: AboutPage,
    },
  ];

  return (
    <>
      <DragAndDrop />
      <Flex vertical className="h-screen w-screen overflow-hidden">
        <Nav
          className={"h-6% p-5"}
          config={navConfig}
          activeId={activeNavId}
          onActiveChange={handleNavChange}
        />
        <div className="flex-1 overflow-auto p-4 h-93%">
          {(() => {
            const Page = navConfig.find(
              (item) => item.id === activeNavId
            )?.page;
            return Page ? <Page /> : null;
          })()}
        </div>
      </Flex>
    </>
  );
}

export default App;
