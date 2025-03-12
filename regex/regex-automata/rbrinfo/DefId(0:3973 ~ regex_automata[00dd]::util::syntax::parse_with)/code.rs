pub fn parse_with(pattern: &str, config: &Config) -> Result<Hir, Error> {
    let mut builder = ParserBuilder::new();
    config.apply(&mut builder);
    builder.build().parse(pattern)
}