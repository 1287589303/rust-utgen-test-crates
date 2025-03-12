fn check_getter(cx: &Ctxt, cont: &Container) {
    match cont.data {
        Data::Enum(_) => {
            if cont.data.has_getter() {
                cx.error_spanned_by(
                    cont.original,
                    "#[serde(getter = \"...\")] is not allowed in an enum",
                );
            }
        }
        Data::Struct(_, _) => {
            if cont.data.has_getter() && cont.attrs.remote().is_none() {
                cx.error_spanned_by(
                    cont.original,
                    "#[serde(getter = \"...\")] can only be used in structs that have #[serde(remote = \"...\")]",
                );
            }
        }
    }
}