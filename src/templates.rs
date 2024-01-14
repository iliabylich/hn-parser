use askama::Template;

use crate::{job::JobToRender, post::Post};

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub(crate) struct Index<'a> {
    pub(crate) post: &'a Post,
    pub(crate) jobs: &'a [JobToRender],
}

#[derive(Template)]
#[template(path = "email.html", escape = "none")]
pub(crate) struct Email<'a> {
    pub(crate) jobs: &'a [JobToRender],
}

pub(crate) static OUTPUT_CSS: &'static str = include_str!("../templates/output.css");
