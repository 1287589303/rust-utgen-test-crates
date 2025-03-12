fn new<I, S>(patterns: I) -> Builder
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let mut b = Builder::default();
        b.pats.extend(patterns.into_iter().map(|p| p.as_ref().to_string()));
        b
    }