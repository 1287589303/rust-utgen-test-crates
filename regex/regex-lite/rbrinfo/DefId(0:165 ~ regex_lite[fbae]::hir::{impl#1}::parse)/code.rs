pub(crate) fn parse(config: Config, pattern: &str) -> Result<Hir, Error> {
        self::parse::Parser::new(config, pattern).parse()
    }