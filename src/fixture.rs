use crate::{job::Job, post::Post};

pub(crate) trait Fixture {
    fn fixture() -> Self
    where
        Self: Sized;
}

impl Fixture for Post {
    fn fixture() -> Self {
        Self {
            hn_id: 12345,
            name: String::from("Who's hiring now"),
        }
    }
}

static LOREM_IPSUM_WITH_RUST: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing Rust elit, sed RUST do eiusmod tempor incididunt ut labore et rust dolore magna aliqua.";

impl Fixture for Job {
    fn fixture() -> Self {
        Self {
            hn_id: Default::default(),
            text: LOREM_IPSUM_WITH_RUST.repeat(15),
            by: String::from("Username"),
            post_hn_id: 12345,
            time: 1298888434,
            interesting: true,
            email_sent: false,
        }
    }
}
