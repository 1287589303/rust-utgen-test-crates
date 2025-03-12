fn struct_from_ast<'a>(
    cx: &Ctxt,
    fields: &'a syn::Fields,
    attrs: Option<&attr::Variant>,
    container_default: &attr::Default,
) -> (Style, Vec<Field<'a>>) {
    match fields {
        syn::Fields::Named(fields) => (
            Style::Struct,
            fields_from_ast(cx, &fields.named, attrs, container_default),
        ),
        syn::Fields::Unnamed(fields) if fields.unnamed.len() == 1 => (
            Style::Newtype,
            fields_from_ast(cx, &fields.unnamed, attrs, container_default),
        ),
        syn::Fields::Unnamed(fields) => (
            Style::Tuple,
            fields_from_ast(cx, &fields.unnamed, attrs, container_default),
        ),
        syn::Fields::Unit => (Style::Unit, Vec::new()),
    }
}