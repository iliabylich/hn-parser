use askama::Template;

use crate::{
    job::{Job, JobToRender},
    post::Post,
    templates::{Email, Index, OUTPUT_CSS},
};

pub(crate) struct Views;

impl Views {
    pub(crate) fn index(last_post: &Post, jobs: &[Job]) -> String {
        let jobs = jobs
            .iter()
            .map(|j| JobToRender::from(j))
            .collect::<Vec<_>>();

        Index {
            post: last_post,
            jobs: &jobs,
        }
        .render()
        .unwrap()
    }

    pub(crate) fn jobs_email(jobs: &[Job]) -> String {
        let jobs = jobs
            .iter()
            .map(|j| JobToRender::from(j))
            .collect::<Vec<_>>();
        Email { jobs: &jobs }.render().unwrap()
    }

    pub(crate) fn output_css() -> &'static str {
        OUTPUT_CSS
    }
}
