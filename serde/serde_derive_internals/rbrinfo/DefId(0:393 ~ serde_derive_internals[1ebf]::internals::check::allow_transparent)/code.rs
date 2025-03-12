fn allow_transparent(field: &Field, derive: Derive) -> bool {
    if let Type::Path(ty) = ungroup(field.ty) {
        if let Some(seg) = ty.path.segments.last() {
            if seg.ident == "PhantomData" {
                return false;
            }
        }
    }

    match derive {
        Derive::Serialize => !field.attrs.skip_serializing(),
        Derive::Deserialize => !field.attrs.skip_deserializing() && field.attrs.default().is_none(),
    }
}