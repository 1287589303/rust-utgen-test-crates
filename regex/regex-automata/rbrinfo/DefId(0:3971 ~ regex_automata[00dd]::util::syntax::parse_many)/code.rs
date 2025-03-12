pub fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<Vec<Hir>, Error> {
    parse_many_with(patterns, &Config::default())
}