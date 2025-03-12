fn get_renames(
    cx: &Ctxt,
    attr_name: Symbol,
    meta: &ParseNestedMeta,
) -> syn::Result<SerAndDe<syn::LitStr>> {
    let (ser, de) = get_ser_and_de(cx, attr_name, meta, get_lit_str2)?;
    Ok((ser.at_most_one(), de.at_most_one()))
}