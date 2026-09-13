use katex::Opts;
use pulldown_cmark::{Event, Options, Parser, html};
use std::error::Error;
use std::fs;

pub fn save_html(output_file: &str, content: &str) -> std::io::Result<()> {
    let root_dir = "website/";
    fs::write(format!("{root_dir}{output_file}.html"), content)?;
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
