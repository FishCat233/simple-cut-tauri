/// 滤镜构建器
///
/// 用于构建复杂的滤镜字符串
pub struct FilterBuilder {
    filters: Vec<String>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// 滤镜数量
    #[allow(dead_code)]
    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }

    /// 是否有滤镜
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// 构建滤镜字符串
    ///
    /// # 返回值
    ///
    /// 返回一个字符串，包含所有滤镜的命令行片段。
    ///
    /// 如果没有滤镜，返回空字符串。
    pub fn build_to_string(&self) -> String {
        if self.is_empty() {
            "".to_string()
        } else {
            format!(" -filter_complex \"{}\"", self.filters.join(";"))
        }
    }

    /// 添加合并amix滤镜
    ///
    /// # 参数
    ///
    /// * `input_index` - 输入索引，对应视频切片的索引
    /// * `track_count` - 音频轨道数量
    /// * `output_alias` - 输出别名，用于引用合并后的音频流
    pub fn add_merge_amix_filter(
        &mut self,
        input_index: usize,
        track_count: usize,
        output_alias: &str,
    ) {
        self.filters
            .push(build_amix_filter(input_index, track_count, output_alias));
    }

    /// 添加concat滤镜
    ///
    /// # 参数
    ///
    /// * `inputs` - 输入流列表，包含视频和音频流的索引
    /// * `video_output` - 视频输出别名，用于引用拼接后的视频流
    /// * `audio_output` - 音频输出别名，用于引用拼接后的音频流
    pub fn add_concat_filter(&mut self, inputs: &[String], video_output: &str, audio_output: &str) {
        self.filters
            .push(build_concat_filter(inputs, video_output, audio_output));
    }
}

/// 构建amix音频合并滤镜
///
/// # 参数
///
/// * `input_index` - 输入索引，对应视频切片的索引
/// * `track_count` - 音频轨道数量
/// * `output_alias` - 输出别名，用于引用合并后的音频流
///
/// # 返回值
///
/// 返回一个字符串，包含 amix 滤镜的命令行片段
pub fn build_amix_filter(input_index: usize, track_count: usize, output_alias: &str) -> String {
    let mut inputs = Vec::new();
    for i in 0..track_count {
        inputs.push(format!("[{}:a:{}]", input_index, i));
    }
    format!(
        "{} amix=inputs={}[{}]",
        inputs.join(" "),
        track_count,
        output_alias
    )
}

/// 构建concat拼接滤镜
///
/// # 参数
///
/// * `inputs` - 输入流列表，包含视频和音频流的索引
/// * `video_output` - 视频输出别名，用于引用拼接后的视频流
/// * `audio_output` - 音频输出别名，用于引用拼接后的音频流
///
/// # 返回值
///
/// 返回一个字符串，包含 concat 滤镜的命令行片段
pub fn build_concat_filter(inputs: &[String], video_output: &str, audio_output: &str) -> String {
    let input_count = inputs.len() / 2;
    let inputs_str = inputs
        .iter()
        .map(|s| format!("[{}]", s))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "{} concat=n={}:v=1:a=1 [{}] [{}]",
        inputs_str, input_count, video_output, audio_output
    )
}
