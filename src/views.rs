use crate::{job::Job, post::Post, template::Template};
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
enum TemplateId {
    Index,
    Email,
}

pub(crate) struct Views {
    templates: HashMap<TemplateId, Template>,
}

impl Views {
    pub(crate) fn new() -> Self {
        Self {
            templates: HashMap::from([
                (TemplateId::Index, Template::new("index.html.liquid")),
                (TemplateId::Email, Template::new("email.html.liquid")),
            ]),
        }
    }

    fn render(&self, template_id: TemplateId, globals: &liquid::Object) -> String {
        self.templates.get(&template_id).unwrap().render(globals)
    }

    pub(crate) fn index(&self, last_post: &Post, jobs: &[Job]) -> String {
        let globals = liquid::object!({
            "post": last_post,
            "jobs": jobs
        });
        self.render(TemplateId::Index, &globals)
    }

    pub(crate) fn jobs_email(&self, jobs: &[Job]) -> String {
        let globals = liquid::object!({
            "jobs": jobs
        });
        self.render(TemplateId::Email, &globals)
    }
}
