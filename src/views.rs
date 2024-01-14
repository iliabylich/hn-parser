use askama::Template;

use crate::{
    job::Job,
    post::Post,
    templates::{Email, Index, OUTPUT_CSS},
};

pub(crate) struct Views;

impl Views {
    pub(crate) fn index(post: &Post, jobs: &[Job]) -> String {
        Index { post, jobs }.render().unwrap()
    }

    pub(crate) fn jobs_email(jobs: &[Job]) -> String {
        Email { jobs }.render().unwrap()
    }

    pub(crate) fn output_css() -> &'static str {
        OUTPUT_CSS
    }
}
