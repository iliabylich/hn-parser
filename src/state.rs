use crate::{highlighter::Highlighter, non_empty_vec::NonEmptyVec, scraper::Item};
use std::collections::HashMap;

#[derive(Default)]
pub(crate) struct State {
    pub(crate) post: Post,
    pub(crate) jobs: HashMap<u32, Job>,
}

impl State {
    pub(crate) fn set(
        &mut self,
        root: Item,
        items: Vec<Item>,
        highlighter: &Highlighter,
    ) -> Option<NonEmptyVec<Job>> {
        self.post = Post::from(root);
        let mut new_jobs = vec![];
        for item in items {
            let job = Job::from(item);
            if !highlighter.can_highlight(&job.text) {
                continue;
            }

            if !self.jobs.contains_key(&job.id) {
                new_jobs.push(job.clone());
            }
            self.jobs.insert(job.id, job);
        }

        NonEmptyVec::try_new(new_jobs)
    }

    pub(crate) fn get(&self) -> (Post, Vec<Job>) {
        let mut jobs = self.jobs.values().cloned().collect::<Vec<_>>();
        jobs.sort_unstable_by_key(|job| job.time);
        (self.post.clone(), jobs)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Post {
    pub(crate) id: u32,
    pub(crate) title: String,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::from("???"),
        }
    }
}

impl From<Item> for Post {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            title: item.title.unwrap_or_else(|| String::from("???")),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Job {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) time: i64,
}

impl From<Item> for Job {
    fn from(item: Item) -> Self {
        Self {
            id: item.id,
            text: item.text.unwrap_or_default(),
            by: item.by.unwrap_or_default(),
            time: item.time,
        }
    }
}

impl Job {
    pub(crate) fn fixture() -> Self {
        const LOREM_IPSUM_WITH_RUST: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing Rust elit, sed RUST do eiusmod tempor incididunt ut labore et rust dolore magna aliqua.";

        Self {
            id: 12345,
            text: LOREM_IPSUM_WITH_RUST.repeat(15),
            by: "Username".to_string(),
            time: 1298888434,
        }
    }
}
