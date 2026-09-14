use crate::converter;
use crate::metadata;
use minijinja::{Environment, context};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const JOURNAL_INDEX_TEMPLATE: &str = include_str!("../res/templates/journal-index.html");
const STYLE_JOURNAL_INDEX_CSS: &str = include_str!("../res/templates/styles/journal-index.css");

#[derive(Serialize, Clone)]
pub struct JournalPost {
    pub path: PathBuf,
    pub slug: String,
    pub meta: metadata::Metadata,
    pub body_html: String,
}

fn mdDirToJournals() -> Result<Vec<JournalPost>, Box<dyn Error>> {
    let mut posts = Vec::new();
    let dir_path = "res/md/journal/";

    if !std::path::Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path)?;
    }

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;

            let (meta, body) = metadata::parse_metadata(&content)
                .map_err(|e| format!("Error parsing metadata in '{}': {e}", path.display()))?;

            let body_html = converter::mdToHtml(body)?;

            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("journal")
                .to_string();

            posts.push(JournalPost {
                path,
                slug: stem,
                meta,
                body_html,
            });
        }
    }

    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    Ok(posts)
}

pub fn generate_journal_index() -> Result<String, Box<dyn Error>> {
    let journal_files: Vec<JournalPost> = mdDirToJournals()?;

    let mut env = Environment::new();
    env.add_template("journal_index", JOURNAL_INDEX_TEMPLATE)?;
    let tmpl = env.get_template("journal_index")?;

    let mut tag_map: BTreeMap<String, Vec<&JournalPost>> = BTreeMap::new();
    let mut all_tags: BTreeSet<String> = BTreeSet::new();

    for post in &journal_files {
        for tag in &post.meta.tags {
            all_tags.insert(tag.clone());
            tag_map.entry(tag.clone()).or_default().push(post);
        }
    }

    // Render tag filter views
    for (tag, tagged_posts) in &tag_map {
        let rendered_tag = tmpl.render(context! {
            posts => tagged_posts,
            css => STYLE_JOURNAL_INDEX_CSS,
            active_tag => Some(tag),
            all_tags => &all_tags,
        })?;
        converter::save_html(&format!("journal-tag-{tag}"), &rendered_tag)?;
    }

    // Render main journal.html view
    let rendered = tmpl.render(context! {
        posts => journal_files,
        css => STYLE_JOURNAL_INDEX_CSS,
    })?;
    converter::save_html("journal", &rendered)?;
    Ok(rendered)
}
