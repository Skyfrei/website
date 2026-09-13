use crate::blog;
use crate::blog_struct;
use crate::converter;
use minijinja::{Environment, context};
use std::error::Error;

const HOME_TEMPLATE: &str = include_str!("../res/templates/home.html");
const STYLE_HOME_CSS: &str = include_str!("../res/templates/styles/home.css");

pub fn generate_home_page() -> Result<String, Box<dyn Error>> {
    blog::generate_blog_index()?;

    // Grab up to the 5 most recent posts for the homepage feed
    let mut env = Environment::new();
    env.add_template("home", HOME_TEMPLATE)?;

    let tmpl = env.get_template("home")?;
    let rendered = tmpl.render(context! {
        css => STYLE_HOME_CSS,
    })?;

    converter::save_html("index", &rendered)?;
    Ok(rendered)
}
