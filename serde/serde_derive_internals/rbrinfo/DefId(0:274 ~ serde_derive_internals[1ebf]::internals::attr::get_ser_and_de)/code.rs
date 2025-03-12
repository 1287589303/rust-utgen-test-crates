fn get_ser_and_de<'c, T, F, R>(
    cx: &'c Ctxt,
    attr_name: Symbol,
    meta: &ParseNestedMeta,
    f: F,
) -> syn::Result<(VecAttr<'c, T>, VecAttr<'c, T>)>
where
    T: Clone,
    F: Fn(&Ctxt, Symbol, Symbol, &ParseNestedMeta) -> syn::Result<R>,
    R: Into<Option<T>>,
{
    let mut ser_meta = VecAttr::none(cx, attr_name);
    let mut de_meta = VecAttr::none(cx, attr_name);

    let lookahead = meta.input.lookahead1();
    if lookahead.peek(Token![=]) {
        if let Some(both) = f(cx, attr_name, attr_name, meta)?.into() {
            ser_meta.insert(&meta.path, both.clone());
            de_meta.insert(&meta.path, both);
        }
    } else if lookahead.peek(token::Paren) {
        meta.parse_nested_meta(|meta| {
            if meta.path == SERIALIZE {
                if let Some(v) = f(cx, attr_name, SERIALIZE, &meta)?.into() {
                    ser_meta.insert(&meta.path, v);
                }
            } else if meta.path == DESERIALIZE {
                if let Some(v) = f(cx, attr_name, DESERIALIZE, &meta)?.into() {
                    de_meta.insert(&meta.path, v);
                }
            } else {
                return Err(meta.error(format_args!(
                    "malformed {0} attribute, expected `{0}(serialize = ..., deserialize = ...)`",
                    attr_name,
                )));
            }
            Ok(())
        })?;
    } else {
        return Err(lookahead.error());
    }

    Ok((ser_meta, de_meta))
}