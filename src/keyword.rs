#[derive(Debug)]
pub(crate) struct Keyword {
    regex: regex::Regex,
}

impl<T> From<T> for Keyword
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        let regex = format!("\\b{}\\b", s.as_ref());
        let regex = regex::RegexBuilder::new(&regex)
            .case_insensitive(true)
            .build()
            .expect("Invalid regex");
        Self { regex }
    }
}

impl Keyword {
    pub(crate) fn is_match<T>(&self, s: T) -> bool
    where
        T: AsRef<str>,
    {
        self.regex.is_match(s.as_ref())
    }

    pub(crate) fn replace_all<T, F>(&self, s: T, f: F) -> String
    where
        T: AsRef<str>,
        F: Fn(&str) -> String,
    {
        self.regex
            .replace_all(s.as_ref(), |captures: &regex::Captures| f(&captures[0]))
            .into_owned()
    }
}
