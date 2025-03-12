fn check_transparent(cx: &Ctxt, cont: &mut Container, derive: Derive) {
    if !cont.attrs.transparent() {
        return;
    }

    if cont.attrs.type_from().is_some() {
        cx.error_spanned_by(
            cont.original,
            "#[serde(transparent)] is not allowed with #[serde(from = \"...\")]",
        );
    }

    if cont.attrs.type_try_from().is_some() {
        cx.error_spanned_by(
            cont.original,
            "#[serde(transparent)] is not allowed with #[serde(try_from = \"...\")]",
        );
    }

    if cont.attrs.type_into().is_some() {
        cx.error_spanned_by(
            cont.original,
            "#[serde(transparent)] is not allowed with #[serde(into = \"...\")]",
        );
    }

    let fields = match &mut cont.data {
        Data::Enum(_) => {
            cx.error_spanned_by(
                cont.original,
                "#[serde(transparent)] is not allowed on an enum",
            );
            return;
        }
        Data::Struct(Style::Unit, _) => {
            cx.error_spanned_by(
                cont.original,
                "#[serde(transparent)] is not allowed on a unit struct",
            );
            return;
        }
        Data::Struct(_, fields) => fields,
    };

    let mut transparent_field = None;

    for field in fields {
        if allow_transparent(field, derive) {
            if transparent_field.is_some() {
                cx.error_spanned_by(
                    cont.original,
                    "#[serde(transparent)] requires struct to have at most one transparent field",
                );
                return;
            }
            transparent_field = Some(field);
        }
    }

    match transparent_field {
        Some(transparent_field) => transparent_field.attrs.mark_transparent(),
        None => match derive {
            Derive::Serialize => {
                cx.error_spanned_by(
                    cont.original,
                    "#[serde(transparent)] requires at least one field that is not skipped",
                );
            }
            Derive::Deserialize => {
                cx.error_spanned_by(
                    cont.original,
                    "#[serde(transparent)] requires at least one field that is neither skipped nor has a default",
                );
            }
        },
    }
}