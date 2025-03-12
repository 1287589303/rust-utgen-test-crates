fn float(input: Cursor) -> Result<Cursor, Reject> {
    let mut rest = float_digits(input)?;
    if let Some(ch) = rest.chars().next() {
        if is_ident_start(ch) {
            rest = ident_not_raw(rest)?.0;
        }
    }
    word_break(rest)
}