use crate::converter;
use minijinja::{Environment, context};
use std::error::Error;

const ABOUT_TEMPLATE: &str = include_str!("../res/templates/about.html");
const STYLE_ABOUT_CSS: &str = include_str!("../res/templates/styles/about.css");

pub fn generate_about_page() -> Result<String, Box<dyn Error>> {
    let mut env = Environment::new();
    env.add_template("about", ABOUT_TEMPLATE)?;

    let tmpl = env.get_template("about")?;
    let rendered = tmpl.render(context! {
        css => STYLE_ABOUT_CSS,
    })?;

    converter::save_html("about", &rendered)?;
    Ok(rendered)
}
