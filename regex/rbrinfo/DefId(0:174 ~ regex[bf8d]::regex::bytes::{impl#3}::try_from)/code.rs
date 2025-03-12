fn try_from(s: &str) -> Result<Regex, Error> {
        Regex::new(s)
    }