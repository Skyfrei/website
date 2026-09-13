use crate::metadata;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Blog {
    pub path: PathBuf,
    pub meta: metadata::Metadata,
    pub body: String,
}
