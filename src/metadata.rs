use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub wordNum: u32,
    pub readTime: u16,
}

pub fn parse_metadata(raw: &str) -> Result<(Metadata, &str), Box<dyn Error>> {
    let trimmed = raw.trim_start();
    if !trimmed.starts_with("---") {
        return Err("No metadata in this file".into());
    }

    let after_first_fence = &trimmed[3..].trim_start_matches(|c| c == '\r' || c == '\n');
    let (yaml_str, body_str) = after_first_fence
        .split_once("\n---")
        .or_else(|| after_first_fence.split_once("\r\n---"))
        .ok_or("Unclosed frontmatter fence (missing closing '---')")?;

    let clean_body = body_str.trim_start_matches(|c| c == '\r' || c == '\n');

    let meta: Metadata = serde_yaml::from_str(yaml_str)?;
    Ok((meta, clean_body))
}
