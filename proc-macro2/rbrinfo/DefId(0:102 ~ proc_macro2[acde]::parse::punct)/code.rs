fn punct(input: Cursor) -> PResult<Punct> {
    let (rest, ch) = punct_char(input)?;
    if ch == '\'' {
        if ident_any(rest)?.0.starts_with_char('\'') {
            Err(Reject)
        } else {
            Ok((rest, Punct::new('\'', Spacing::Joint)))
        }
    } else {
        let kind = match punct_char(rest) {
            Ok(_) => Spacing::Joint,
            Err(Reject) => Spacing::Alone,
        };
        Ok((rest, Punct::new(ch, kind)))
    }
}