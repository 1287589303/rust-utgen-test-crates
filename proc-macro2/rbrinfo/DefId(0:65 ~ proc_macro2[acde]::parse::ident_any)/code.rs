fn ident_any(input: Cursor) -> PResult<crate::Ident> {
    let raw = input.starts_with("r#");
    let rest = input.advance((raw as usize) << 1);

    let (rest, sym) = ident_not_raw(rest)?;

    if !raw {
        let ident =
            crate::Ident::_new_fallback(Ident::new_unchecked(sym, fallback::Span::call_site()));
        return Ok((rest, ident));
    }

    match sym {
        "_" | "super" | "self" | "Self" | "crate" => return Err(Reject),
        _ => {}
    }

    let ident =
        crate::Ident::_new_fallback(Ident::new_raw_unchecked(sym, fallback::Span::call_site()));
    Ok((rest, ident))
}