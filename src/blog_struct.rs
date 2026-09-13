use crate::metadata;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Blog {
    pub path: PathBuf,
    pub slug: String,
    pub meta: metadata::Metadata,
    pub body: String,
}
