use crate::state::{Job, Post};
use askama::Template;

macro_rules! timeago {
    ($timestamp:expr) => {{
        use std::time::{Duration, SystemTime, UNIX_EPOCH};

        let moment = UNIX_EPOCH + Duration::from_secs($timestamp.try_into().unwrap_or(0));
        let delta = SystemTime::now().duration_since(moment).unwrap_or_default();

        let mut formatter = timeago::Formatter::new();
        formatter.num_items(3);

        formatter.convert(delta)
    }};
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub(crate) struct Index {
    pub(crate) post: Post,
    pub(crate) jobs: Vec<Job>,
}

#[derive(Template)]
#[template(path = "email.html", escape = "none")]
pub(crate) struct Email {
    pub(crate) jobs: Vec<Job>,
}

pub(crate) static OUTPUT_CSS: &str = include_str!("../templates/output.css");
