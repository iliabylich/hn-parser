#[derive(Debug, Default)]
pub(crate) struct Highlighter {
    regexes: Vec<regex::Regex>,
}

impl Highlighter {
    pub(crate) fn new(strings: &[String]) -> Self {
        let regexes = strings
            .iter()
            .map(|s| {
                let regex = format!("\\b{}\\b", s);
                regex::RegexBuilder::new(&regex)
                    .case_insensitive(true)
                    .build()
                    .expect("Invalid regex")
            })
            .collect();
        Self { regexes }
    }
}

impl Highlighter {
    pub(crate) fn can_highlight(&self, s: impl AsRef<str>) -> bool {
        self.regexes.iter().any(|regex| regex.is_match(s.as_ref()))
    }

    pub(crate) fn highlight(&self, mut s: String, f: impl Fn(&str) -> String) -> String {
        for regex in &self.regexes {
            s = regex
                .replace_all(&s, |captures: &regex::Captures| f(&captures[0]))
                .into_owned();
        }
        s
    }
}
