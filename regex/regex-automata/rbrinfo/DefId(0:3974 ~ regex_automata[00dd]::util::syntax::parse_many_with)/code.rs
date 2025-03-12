pub fn parse_many_with<P: AsRef<str>>(
    patterns: &[P],
    config: &Config,
) -> Result<Vec<Hir>, Error> {
    let mut builder = ParserBuilder::new();
    config.apply(&mut builder);
    let mut hirs = vec![];
    for p in patterns.iter() {
        hirs.push(builder.build().parse(p.as_ref())?);
    }
    Ok(hirs)
}