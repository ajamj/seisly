pub struct SegyMetadata {
    pub inline_range: (i32, i32),
    pub crossline_range: (i32, i32),
    pub sample_count: usize,
    pub sample_interval: f32,
}

pub fn parse_metadata(_path: &std::path::Path) -> anyhow::Result<SegyMetadata> {
    // Placeholder: In reality, read binary/text headers
    Ok(SegyMetadata {
        inline_range: (1, 100),
        crossline_range: (1, 100),
        sample_count: 500,
        sample_interval: 4.0,
    })
}
