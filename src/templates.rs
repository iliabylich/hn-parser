use crate::state::{Job, Post};
use askama::Template;

macro_rules! timeago {
    ($timestamp:expr) => {{
        use chrono::Utc;
        use chrono::prelude::DateTime;
        use std::time::{Duration, UNIX_EPOCH};

        let moment = DateTime::<Utc>::from(
            UNIX_EPOCH + Duration::from_secs($timestamp.try_into().unwrap_or(0)),
        );
        let now = Utc::now();
        let delta = (now - moment).to_std().unwrap_or_default();

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
