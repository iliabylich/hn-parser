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

fn get_template(path: &str) -> String {
    if cfg!(debug_assertions) {
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {}", path))
    } else {
        let content = Asset::get(path)
            .expect("Failed to get template from assets")
            .data;
        std::str::from_utf8(content.as_ref()).unwrap().to_string()
    }
}

impl Template {
    pub(crate) fn new(path: &'static str) -> Self {
        let cached_template = if cfg!(debug_assertions) {
            None
        } else {
            Some(parse_template(&get_template(path)))
        };
        Self {
            path,
            cached_template,
        }
    }

    pub(crate) fn render(&self, globals: &liquid::Object) -> String {
        if cfg!(debug_assertions) {
            let content = get_template(self.path);
            let template = parse_template(&content);
            template.render(globals).expect("Failed to render template")
        } else {
            self.cached_template
                .as_ref()
                .unwrap()
                .render(globals)
                .expect("Failed to render template")
        }
    }
}
