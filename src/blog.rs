use crate::blog_struct;
use crate::converter;
use crate::metadata;
use minijinja::{Environment, context};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

const POST_TEMPLATE: &str = include_str!("../res/templates/blog-post.html");
const STYLE_POST_CSS: &str = include_str!("../res/templates/styles/blog.css");

const BLOG_INDEX_TEMPLATE: &str = include_str!("../res/templates/blog-index.html");
const STYLE_BLOG_INDEX_CSS: &str = include_str!("../res/templates/styles/blog-index.css");

fn blogFileToHtml(file: &PathBuf) -> Result<String, Box<dyn Error>> {
    let content: String = fs::read_to_string(file)?;
    let (mut meta, body): (metadata::Metadata, &str) = metadata::parse_metadata(&content)?;
    meta.wordNum = body.split_whitespace().count() as u32;
    meta.readTime = (meta.wordNum / 200) as u16;

    let body_html = converter::mdToHtml(body)?;
    let mut env = Environment::new();
    let template_source = POST_TEMPLATE;
    env.add_template("post", &template_source)?;

    let tmpl = env.get_template("post")?;
    let rendered_html = tmpl.render(context! {
        meta => meta,
        body => body_html,
        css => STYLE_POST_CSS,
    })?;

    let filename = file.file_stem().and_then(|s| s.to_str()).unwrap_or("post");
    converter::save_html(filename, &rendered_html)?;
    Ok(rendered_html)
}

fn mdDirToBlogs() -> Result<Vec<blog_struct::Blog>, Box<dyn Error>> {
    let mut posts = Vec::new();
    let dir_path = "res/md/blogs/";

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
            let (meta, body) = metadata::parse_metadata(&content)
                .map_err(|e| format!("Error parsing metadata in '{}': {e}", path.display()))?;

            blogFileToHtml(&path)?;

            posts.push(blog_struct::Blog {
                path,
                meta,
                body: body.to_string(),
            });
        }
    }

    posts.sort_by(|a, b| b.meta.date.cmp(&a.meta.date));
    Ok(posts)
}

pub fn generate_blog_index() -> Result<String, Box<dyn Error>> {
    let blog_files: Vec<blog_struct::Blog> = mdDirToBlogs()?;

    let mut env = Environment::new();
    env.add_template("blog_index", BLOG_INDEX_TEMPLATE)?;

    let tmpl = env.get_template("blog_index")?;
    let rendered = tmpl.render(context! {
        blogs => blog_files,
        css => STYLE_BLOG_INDEX_CSS,
    })?;
    converter::save_html("blog", &rendered)?;
    Ok(rendered)
}
