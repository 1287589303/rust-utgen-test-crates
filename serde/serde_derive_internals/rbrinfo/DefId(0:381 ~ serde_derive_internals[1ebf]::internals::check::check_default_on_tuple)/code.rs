fn check_default_on_tuple(cx: &Ctxt, cont: &Container) {
    if let Default::None = cont.attrs.default() {
        if let Data::Struct(Style::Tuple, fields) = &cont.data {
            let mut first_default_index = None;
            for (i, field) in fields.iter().enumerate() {
                // Skipped fields automatically get the #[serde(default)]
                // attribute. We are interested only on non-skipped fields here.
                if field.attrs.skip_deserializing() {
                    continue;
                }
                if let Default::None = field.attrs.default() {
                    if let Some(first) = first_default_index {
                        cx.error_spanned_by(
                            field.ty,
                            format!("field must have #[serde(default)] because previous field {} has #[serde(default)]", first),
                        );
                    }
                    continue;
                }
                if first_default_index.is_none() {
                    first_default_index = Some(i);
                }
            }
        }
    }
}