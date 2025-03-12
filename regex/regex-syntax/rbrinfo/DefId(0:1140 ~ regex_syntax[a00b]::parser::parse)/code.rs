pub fn parse(pattern: &str) -> Result<hir::Hir, Error> {
    Parser::new().parse(pattern)
}