use liquid::ParserBuilder;

#[derive(Debug, Hash, PartialEq, Eq)]
pub(crate) enum TemplateId {
    Index,
    Email,
}

pub(crate) struct Template {
    path: String,
    pre_compiled: liquid::Template,
}

fn parse_template(content: &str) -> liquid::Template {
    let mut builder = ParserBuilder::with_stdlib();
    builder = crate::liquid::add_filters(builder);
    builder
        .build()
        .expect("Failed to build liquid parser")
        .parse(content)
        .unwrap_or_else(|err| panic!("Failed to compile template:\n{}", err))
}

impl Template {
    pub(crate) fn new(path: &str, embedded_src: &str) -> Self {
        Self {
            path: path.to_string(),
            pre_compiled: parse_template(embedded_src),
        }
    }

    fn render_debug(&self, globals: &liquid::Object) -> String {
        let fresh_src =
            std::fs::read_to_string(&self.path).expect(&format!("Failed to read {}", self.path));
        let template = parse_template(&fresh_src);
        template.render(globals).expect("Failed to render template")
    }

    fn render_release(&self, globals: &liquid::Object) -> String {
        self.pre_compiled
            .render(globals)
            .expect("Failed to render template")
    }

    pub(crate) fn render(&self, globals: &liquid::Object) -> String {
        if cfg!(debug_assertions) {
            self.render_debug(globals)
        } else {
            self.render_release(globals)
        }
    }
}
