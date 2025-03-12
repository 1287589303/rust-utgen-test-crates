pub fn parse(pattern: &str) -> Result<Hir, Error> {
    parse_with(pattern, &Config::default())
}