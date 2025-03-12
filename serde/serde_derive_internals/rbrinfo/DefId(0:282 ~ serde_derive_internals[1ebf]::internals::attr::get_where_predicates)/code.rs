fn get_where_predicates(
    cx: &Ctxt,
    meta: &ParseNestedMeta,
) -> syn::Result<SerAndDe<Vec<syn::WherePredicate>>> {
    let (ser, de) = get_ser_and_de(cx, BOUND, meta, parse_lit_into_where)?;
    Ok((ser.at_most_one(), de.at_most_one()))
}