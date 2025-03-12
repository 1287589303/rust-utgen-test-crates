pub fn new<I, S>(patterns: I) -> RegexSetBuilder
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            RegexSetBuilder { builder: Builder::new(patterns) }
        }