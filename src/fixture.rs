use crate::{job::Job, post::Post};

pub(crate) trait Fixture {
    fn fixture() -> Self
    where
        Self: Sized;
}

impl Fixture for Post {
    fn fixture() -> Self {
        Self {
            id: 12345,
            title: "Who's hiring now".to_string(),
        }
    }
}

static LOREM_IPSUM_WITH_RUST: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing Rust elit, sed RUST do eiusmod tempor incididunt ut labore et rust dolore magna aliqua.";

impl Fixture for Job {
    fn fixture() -> Self {
        Self {
            id: 12345,
            text: LOREM_IPSUM_WITH_RUST.repeat(15),
            by: "Username".to_string(),
            time: 1298888434,
        }
    }
}
