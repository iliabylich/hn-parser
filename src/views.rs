use crate::{job::Job, post::Post};

use liquid::ParserBuilder;

pub(crate) struct Views {
    #[allow(dead_code)]
    index_template: liquid::Template,
}

fn template(content: &str) -> liquid::Template {
    ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse(content)
        .unwrap_or_else(|err| panic!("Failed to compile template:\n{}", err))
}

static INDEX_TEMPLATE: &str = include_str!("../views/index.html.liquid");

impl Views {
    pub(crate) fn new() -> Self {
        Self {
            index_template: template(INDEX_TEMPLATE),
        }
    }

    #[cfg(debug_assertions)]
    fn with_index_template<F>(&self, f: F) -> String
    where
        F: FnOnce(&liquid::Template) -> String,
    {
        let template_src = std::fs::read_to_string("views/index.html.liquid")
            .expect("Failed to read views/index.html.liquid");
        let t = template(&template_src);
        f(&t)
    }

    #[cfg(not(debug_assertions))]
    fn with_index_template<F>(&self, f: F) -> String
    where
        F: FnOnce(&liquid::Template) -> String,
    {
        f(&self.index_template)
    }

    pub(crate) fn index(&self, last_post: &Post, jobs: &[Job]) -> String {
        let globals = liquid::object!({
            "post": last_post,
            "jobs": jobs
        });
        self.with_index_template(|template| template.render(&globals).unwrap())
    }
}
