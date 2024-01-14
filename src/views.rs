use anyhow::{Context, Result};
use askama::Template;

use crate::{
    job::Job,
    post::Post,
    templates::{Email, Index, OUTPUT_CSS},
};

pub(crate) struct Views;

impl Views {
    pub(crate) fn index(post: &Post, jobs: &[Job]) -> Result<String> {
        Index { post, jobs }
            .render()
            .context("failed to render index")
    }

    pub(crate) fn jobs_email(jobs: &[Job]) -> Result<String> {
        Email { jobs }.render().context("failed to render email")
    }

    pub(crate) fn output_css() -> &'static str {
        OUTPUT_CSS
    }
}
