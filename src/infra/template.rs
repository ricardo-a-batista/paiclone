use lazy_static::lazy_static;
use tera::{Context, Tera};

use crate::Result;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("templates/**/*").unwrap();
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

pub fn render(template_name: &str) -> Result<String> {
    Ok(TEMPLATES.render(template_name, &Context::new())?)
}
