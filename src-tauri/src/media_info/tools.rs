use anyhow::Context;

/// 获取音频轨道数量
pub fn get_audio_track_count(ffprobe_path: &str, file_path: &str) -> anyhow::Result<usize> {
    let output = std::process::Command::new(ffprobe_path)
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("a")
        .arg("-show_entries")
        .arg("stream=index")
        .arg("-of")
        .arg("csv=p=0")
        .arg(file_path)
        .output()
        .context("执行ffprobe失败")?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "ffprobe返回错误：{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout).context("解析ffprobe输出失败")?;
    let track_count = stdout.lines().count();

    Ok(track_count)
}
