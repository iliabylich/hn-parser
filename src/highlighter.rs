use crate::config::Config;
use anyhow::{Context as _, Result};
use tokio::sync::OnceCell;

#[derive(Debug, Default)]
pub(crate) struct Highlighter {
    regexes: Vec<regex::Regex>,
}

static HIGHLIGHTER: OnceCell<Highlighter> = OnceCell::const_new();

impl Highlighter {
    pub(crate) fn setup() -> Result<()> {
        let regexes = Config::global()
            .keywords
            .iter()
            .map(|string| {
                let regex = format!("\\b{}\\b", string);
                regex::RegexBuilder::new(&regex)
                    .case_insensitive(true)
                    .build()
                    .context("invalid regex")
            })
            .collect::<Result<Vec<_>>>()?;
        HIGHLIGHTER
            .set(Self { regexes })
            .context("failed to set global Highlighter")?;
        Ok(())
    }

    pub(crate) fn global() -> &'static Self {
        HIGHLIGHTER.get().expect("global Highlighter is not set")
    }
}

impl Highlighter {
    pub(crate) fn can_highlight(&self, s: impl AsRef<str>) -> bool {
        self.regexes.iter().any(|regex| regex.is_match(s.as_ref()))
    }

    pub(crate) fn highlight(&self, s: String, f: impl Fn(&str) -> String) -> String {
        self.regexes.iter().fold(s, |s, regex| {
            regex
                .replace_all(&s, |captures: &regex::Captures| f(&captures[0]))
                .into_owned()
        })
    }
}
