import { getCurrentWebview } from "@tauri-apps/api/webview";
import { useEffect } from "react";
import { useAppStore } from "../store";
import { message } from "antd";

export default function DragAndDrop() {
  const { appendFileByPaths } = useAppStore();

  useEffect(() => {
    console.log("DragAndDrop mounted");

    const setupDragDrop = async () => {
      const webview = await getCurrentWebview();
      const unlisten = webview.onDragDropEvent((event) => {
        if (event.payload.type == "drop") {
          const paths = event.payload.paths;
          appendFileByPaths(paths);
          message.success(`成功添加 ${paths.length} 个文件`);
        }
      });
      return unlisten;
    };

    const unlistenPromise = setupDragDrop();

    return () => {
      console.log("DragAndDrop unmounted");
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return <></>;
}
