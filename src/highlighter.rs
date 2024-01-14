use anyhow::{Context, Result};

#[derive(Debug, Default)]
pub(crate) struct Highlighter {
    regexes: Vec<regex::Regex>,
}

impl Highlighter {
    pub(crate) fn new(strings: &[String]) -> Result<Self> {
        let regexes = strings
            .iter()
            .map(|string| {
                let regex = format!("\\b{}\\b", string);
                regex::RegexBuilder::new(&regex)
                    .case_insensitive(true)
                    .build()
                    .context("invalid regex")
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { regexes })
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
