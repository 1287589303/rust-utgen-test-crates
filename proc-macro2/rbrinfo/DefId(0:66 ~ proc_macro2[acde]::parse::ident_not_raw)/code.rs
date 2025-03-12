fn ident_not_raw(input: Cursor) -> PResult<&str> {
    let mut chars = input.char_indices();

    match chars.next() {
        Some((_, ch)) if is_ident_start(ch) => {}
        _ => return Err(Reject),
    }

    let mut end = input.len();
    for (i, ch) in chars {
        if !is_ident_continue(ch) {
            end = i;
            break;
        }
    }

    Ok((input.advance(end), &input.rest[..end]))
}