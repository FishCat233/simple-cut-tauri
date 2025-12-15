use serde::Serialize;

#[derive(Serialize)]
pub struct CommandLineOutput {
    signal: i32,
    stdout: String,
    stderr: String,
}

#[tauri::command]
pub fn execute_command_line(command: &str) -> CommandLineOutput {
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    CommandLineOutput {
        signal: output.status.code().unwrap_or(-1),
        stdout: stdout.into(),
        stderr: stderr.into(),
    }
}
