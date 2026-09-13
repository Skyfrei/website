use crate::metadata;
use katex::Opts;
use minijinja::{Environment, context};
use pulldown_cmark::{Event, Options, Parser, html};
use std::error::Error;
use std::fs;

const POST_TEMPLATE: &str = include_str!("../res/templates/post.html");
const STYLE_CSS: &str = include_str!("../res/templates/style.css");

pub fn mdFileToHtml(file: &str) -> Result<String, Box<dyn Error>> {
    let content: String = fs::read_to_string(file)?;
    let (mut meta, body): (metadata::Metadata, &str) = metadata::parse_metadata(&content)?;
    meta.wordNum = body.split_whitespace().count() as u32;
    meta.readTime = (meta.wordNum / 200) as u16;

    let body_html = mdToHtml(body)?;
    let mut env = Environment::new();
    let template_source = POST_TEMPLATE;
    env.add_template("post", &template_source)?;

    // 3. Render into the template
    let tmpl = env.get_template("post")?;
    let rendered_html = tmpl.render(context! {
        meta => meta,
        body => body_html,
        css => STYLE_CSS,
    })?;
    save_html(&rendered_html)?;
    Ok(rendered_html)
}

fn save_html(str: &str) -> std::io::Result<()> {
    let file_name = "main.html";
    fs::write(file_name, str)?;
    Ok(())
}

pub fn mdToHtml(content: &str) -> Result<String, Box<dyn Error>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_MATH);

    let opts_block = Opts::builder().display_mode(true).build()?;
    let opts_inline = Opts::builder().display_mode(false).build()?;

    let parser = Parser::new_ext(content, options);

    // Intercept math events and compile them with KaTeX
    let events = parser.map(|event| match event {
        Event::DisplayMath(math) => {
            let rendered = katex::render_with_opts(&math, &opts_block)
                .unwrap_or_else(|e| format!("<pre class=\"math-err\">{math} (Error: {e})</pre>"));
            Event::Html(rendered.into())
        }
        Event::InlineMath(math) => {
            let rendered = katex::render_with_opts(&math, &opts_inline)
                .unwrap_or_else(|e| format!("<code class=\"math-err\">{math} (Error: {e})</code>"));
            Event::Html(rendered.into())
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, events);
    Ok(html_output)
}
