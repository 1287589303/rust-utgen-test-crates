fn is_str(ty: &syn::Type) -> bool {
    is_primitive_type(ty, "str")
}