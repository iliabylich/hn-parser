use askama::Template;

use crate::{job::Job, post::Post};

mod helpers {
    macro_rules! timeago {
        ($timestamp:expr) => {{
            use chrono::prelude::DateTime;
            use chrono::Utc;
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

    pub(crate) use timeago;
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
pub(crate) struct Index<'a> {
    pub(crate) post: &'a Post,
    pub(crate) jobs: &'a [Job],
}

#[derive(Template)]
#[template(path = "email.html", escape = "none")]
pub(crate) struct Email<'a> {
    pub(crate) jobs: &'a [Job],
}

pub(crate) static OUTPUT_CSS: &'static str = include_str!("../templates/output.css");
