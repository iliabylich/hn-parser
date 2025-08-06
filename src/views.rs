use anyhow::{Context as _, Result};
use askama::Template;

use crate::{
    state::{Job, Post},
    templates::{Email, Index, OUTPUT_CSS},
};

pub(crate) struct Views;

impl Views {
    pub(crate) fn index(post: Post, jobs: Vec<Job>) -> Result<String> {
        Index { post, jobs }
            .render()
            .context("failed to render index")
    }

    pub(crate) fn jobs_email(jobs: Vec<Job>) -> Result<String> {
        Email { jobs }.render().context("failed to render email")
    }

    pub(crate) fn output_css() -> &'static str {
        OUTPUT_CSS
    }
}
