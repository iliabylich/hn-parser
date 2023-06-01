use crate::{
    job::Job,
    post::Post,
    template::{Template, TemplateId},
};
use std::collections::HashMap;

pub(crate) struct Views {
    templates: HashMap<TemplateId, Template>,
}

static INDEX_TEMPLATE: &str = include_str!("../views/index.html.liquid");
static EMAIL_TEMPLATE: &str = include_str!("../views/email.html.liquid");

impl Views {
    pub(crate) fn new() -> Self {
        let mut views = Self {
            templates: HashMap::new(),
        };
        views.register_template(
            TemplateId::Index,
            Template::new("views/index.html.liquid", INDEX_TEMPLATE),
        );
        views.register_template(
            TemplateId::Email,
            Template::new("views/email.html.liquid", EMAIL_TEMPLATE),
        );
        views
    }

    fn register_template(&mut self, template_id: TemplateId, template: Template) {
        self.templates.insert(template_id, template);
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
