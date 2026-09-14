use crate::converter;
use minijinja::{Environment, context};
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;

const WEBRING_TEMPLATE: &str = include_str!("../res/templates/webring.html");
const STYLE_WEBRING_CSS: &str = include_str!("../res/templates/styles/webring.css");

#[derive(Serialize)]
struct WebringMember {
    name: &'static str,
    domain: &'static str,
    url: &'static str,
    badge: &'static str,
}

pub fn generate_webring_page() -> Result<String, Box<dyn Error>> {
    copy_gif_assets()?;
    let sites = vec![
        WebringMember {
            name: "iczelia",
            domain: "iczelia.net",
            url: "https://iczelia.net",
            badge: "gifs/iczelia.gif",
        },
        WebringMember {
            name: "c7.pm",
            domain: "c7.pm",
            url: "https://c7.pm",
            badge: "gifs/c7.gif",
        },
        WebringMember {
            name: "alula",
            domain: "alula.me",
            url: "https://alula.me",
            badge: "gifs/alula.gif",
        },
    ];

    let mut env = Environment::new();
    env.add_template("webring", WEBRING_TEMPLATE)?;

    let tmpl = env.get_template("webring")?;
    let rendered = tmpl.render(context! {
        sites => sites,
        css => STYLE_WEBRING_CSS,
    })?;

    converter::save_html("webring", &rendered)?;
    Ok(rendered)
}

pub fn copy_gif_assets() -> Result<(), Box<dyn Error>> {
    let src = Path::new("res/gifs");
    let dest = Path::new("website/gifs"); // points directly to your site directory

    if src.exists() {
        fs::create_dir_all(dest)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    fs::copy(&path, dest.join(name))?;
                }
            }
        }
    }
    Ok(())
}
