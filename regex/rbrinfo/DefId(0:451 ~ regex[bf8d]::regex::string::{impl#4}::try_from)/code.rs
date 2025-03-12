fn try_from(s: String) -> Result<Regex, Error> {
        Regex::new(&s)
    }