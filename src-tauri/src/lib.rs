use log::info;
mod command;
mod core;
mod media_info;

// 旧的greet命令保持不变，作为示例
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    env_logger::init();

    info!("Starting Simple Cut Tauri application...");

    tauri::Builder::default()
        // 注册插件
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        // 注册所有命令
        .invoke_handler(tauri::generate_handler![
            // 旧的greet命令
            greet,
            // 执行命令行
            command::execute_command_line,
            core::export::export // 导出视频切片
        ])
        // 运行应用
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
