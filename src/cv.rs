use crate::converter;
use minijinja::{Environment, context};
use std::error::Error;

const CV_TEMPLATE: &str = include_str!("../res/templates/cv.html");
const STYLE_CV_CSS: &str = include_str!("../res/templates/styles/cv.css");

pub fn generate_cv_page() -> Result<String, Box<dyn Error>> {
    let mut env = Environment::new();
    env.add_template("cv", CV_TEMPLATE)?;

    let tmpl = env.get_template("cv")?;
    let rendered = tmpl.render(context! {
        css => STYLE_CV_CSS,
    })?;

    converter::save_html("cv", &rendered)?;
    Ok(rendered)
}
