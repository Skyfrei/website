use katex::Opts;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd, html};
use std::error::Error;
use std::fs;
use syntect::highlighting::{Color, FontStyle, StyleModifier, Theme, ThemeItem, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn save_html(output_file: &str, content: &str) -> std::io::Result<()> {
    let root_dir = "website/";
    fs::write(format!("{root_dir}{output_file}.html"), content)?;
    Ok(())
}

fn gruber_darker_theme() -> Theme {
    // Core Gruber Darker palette
    let bg = Color {
        r: 0x18,
        g: 0x18,
        b: 0x18,
        a: 0xff,
    }; // #181818
    let fg = Color {
        r: 0xe4,
        g: 0xe4,
        b: 0xe4,
        a: 0xff,
    }; // #e4e4e4
    let yellow = Color {
        r: 0xff,
        g: 0xdd,
        b: 0x33,
        a: 0xff,
    }; // #ffdd33 (keywords, operators)
    let green = Color {
        r: 0x73,
        g: 0xc9,
        b: 0x36,
        a: 0xff,
    }; // #73c936 (strings)
    let quartz = Color {
        r: 0x95,
        g: 0xa9,
        b: 0x9f,
        a: 0xff,
    }; // #95a99f (types, properties)
    let brown = Color {
        r: 0xcc,
        g: 0x8c,
        b: 0x3c,
        a: 0xff,
    }; // #cc8c3c (constants, numbers)
    let comment = Color {
        r: 0xcc,
        g: 0x66,
        b: 0x66,
        a: 0xff,
    }; // #cc6666 (comments/red)

    let mut theme = Theme {
        name: Some("Gruber Darker".to_string()),
        author: None,
        scopes: Vec::new(),
        settings: syntect::highlighting::ThemeSettings::default(),
    };

    theme.settings.background = Some(bg);
    theme.settings.foreground = Some(fg);

    let rules = vec![
        // Keywords & Operators -> Yellow
        (
            "keyword, storage, keyword.operator",
            yellow,
            FontStyle::BOLD,
        ),
        // Strings -> Green
        ("string", green, FontStyle::empty()),
        // Types & Classes -> Quartz
        (
            "entity.name.type, storage.type, support.type",
            quartz,
            FontStyle::empty(),
        ),
        // Constants & Numbers -> Brown
        (
            "constant, constant.numeric, constant.language",
            brown,
            FontStyle::empty(),
        ),
        // Functions -> Foreground bold
        (
            "entity.name.function, support.function",
            fg,
            FontStyle::BOLD,
        ),
        // Comments -> Brownish/Red
        ("comment", comment, FontStyle::ITALIC),
        // Variables & Identifiers -> Main FG
        ("variable", fg, FontStyle::empty()),
    ];

    for (selector, color, style) in rules {
        theme.scopes.push(ThemeItem {
            scope: selector.parse().unwrap(),
            style: StyleModifier {
                foreground: Some(color),
                background: None,
                font_style: Some(style),
            },
        });
    }

    theme
}

pub fn highlight_code(code: &str, lang: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let theme = gruber_darker_theme();

    highlighted_html_for_string(code, &ss, syntax, &theme)
        .unwrap_or_else(|_| format!("<pre><code>{}</code></pre>", code))
}

pub fn mdToHtml(content: &str) -> Result<String, Box<dyn Error>> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_MATH);

    let opts_block = Opts::builder().display_mode(true).build()?;
    let opts_inline = Opts::builder().display_mode(false).build()?;

    let parser = Parser::new_ext(content, options);
    let mut in_code = false;
    let mut lang = String::new();
    let mut code_buf = String::new();

    // Intercept math events and compile them with KaTeX
    let events = parser.filter_map(|event| match event {
        Event::DisplayMath(math) => {
            let rendered = katex::render_with_opts(&math, &opts_block)
                .unwrap_or_else(|e| format!("<pre class=\"math-err\">{math} (Error: {e})</pre>"));
            Some(Event::Html(rendered.into()))
        }
        Event::InlineMath(math) => {
            let rendered = katex::render_with_opts(&math, &opts_inline)
                .unwrap_or_else(|e| format!("<code class=\"math-err\">{math} (Error: {e})</code>"));
            Some(Event::Html(rendered.into()))
        }
        Event::Start(Tag::CodeBlock(kind)) => {
            in_code = true;
            code_buf.clear();
            lang = match kind {
                CodeBlockKind::Fenced(l) => l.to_string(),
                CodeBlockKind::Indented => String::new(),
            };
            None
        }
        Event::End(TagEnd::CodeBlock) => {
            in_code = false;
            Some(Event::Html(highlight_code(&code_buf, &lang).into()))
        }
        Event::Text(text) if in_code => {
            code_buf.push_str(&text);
            None
        }
        _ => Some(event),
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, events);
    Ok(html_output)
}
