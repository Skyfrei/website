use crate::converter;
use minijinja::{Environment, context};
use std::error::Error;

const GUESTBOOK_TEMPLATE: &str = include_str!("../res/templates/guestbook.html");
const STYLE_GUESTBOOK_CSS: &str = include_str!("../res/templates/styles/guestbook.css");

#[derive(serde::Serialize)]
struct GuestbookEntry {
    name: &'static str,
    date: &'static str,
    message: &'static str,
}

pub fn generate_guestbook_page() -> Result<String, Box<dyn Error>> {
    // Initial entries or read from a local TOML/JSON cache
    let mut env = Environment::new();
    env.add_template("guestbook", GUESTBOOK_TEMPLATE)?;

    let tmpl = env.get_template("guestbook")?;
    let rendered = tmpl.render(context! {
        css => STYLE_GUESTBOOK_CSS,
    })?;

    converter::save_html("guestbook", &rendered)?;
    Ok(rendered)
}
