use liquid::ParserBuilder;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "views/"]
struct Asset;

pub(crate) struct Template {
    path: &'static str,
    cached_template: Option<liquid::Template>,
}

fn parse_template(content: &str) -> liquid::Template {
    ParserBuilder::with_stdlib()
        .build()
        .expect("Failed to build liquid parser")
        .parse(content)
        .unwrap_or_else(|err| panic!("Failed to compile template:\n{}", err))
}

impl Template {
    pub(crate) fn new(path: &'static str) -> Self {
        if cfg!(debug_assertions) {
            Self {
                path,
                cached_template: None,
            }
        } else {
            let content =
                std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", path));
            let template = parse_template(&content);
            Self {
                path,
                cached_template: Some(template),
            }
        }
    }

    pub(crate) fn render(&self, globals: &liquid::Object) -> String {
        if let Some(template) = &self.cached_template {
            template.render(globals).expect("Failed to render template")
        } else {
            let content = Asset::get(self.path)
                .expect("Failed to get template from assets")
                .data;
            let template = parse_template(std::str::from_utf8(content.as_ref()).unwrap());
            template.render(globals).expect("Failed to render template")
        }
    }
}
