fn parse(s: &str) -> Option<Mime> {
    let trimmed = s.trim_matches(http_whitespace);

    let (type_, rest) = split2(trimmed, '/');
    require!(only_http_token_code_points(type_) && !type_.is_empty());

    let (subtype, rest) = split2(rest?, ';');
    let subtype = subtype.trim_end_matches(http_whitespace);
    require!(only_http_token_code_points(subtype) && !subtype.is_empty());

    let mut parameters = Vec::new();
    if let Some(rest) = rest {
        parse_parameters(rest, &mut parameters)
    }

    Some(Mime {
        type_: type_.to_ascii_lowercase(),
        subtype: subtype.to_ascii_lowercase(),
        parameters,
    })
}