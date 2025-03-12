fn check_variant_skip_attrs(cx: &Ctxt, cont: &Container) {
    let variants = match &cont.data {
        Data::Enum(variants) => variants,
        Data::Struct(_, _) => return,
    };

    for variant in variants {
        if variant.attrs.serialize_with().is_some() {
            if variant.attrs.skip_serializing() {
                cx.error_spanned_by(
                    variant.original,
                    format!(
                        "variant `{}` cannot have both #[serde(serialize_with)] and #[serde(skip_serializing)]",
                        variant.ident
                    ),
                );
            }

            for field in &variant.fields {
                let member = member_message(&field.member);

                if field.attrs.skip_serializing() {
                    cx.error_spanned_by(
                        variant.original,
                        format!(
                            "variant `{}` cannot have both #[serde(serialize_with)] and a field {} marked with #[serde(skip_serializing)]",
                            variant.ident, member
                        ),
                    );
                }

                if field.attrs.skip_serializing_if().is_some() {
                    cx.error_spanned_by(
                        variant.original,
                        format!(
                            "variant `{}` cannot have both #[serde(serialize_with)] and a field {} marked with #[serde(skip_serializing_if)]",
                            variant.ident, member
                        ),
                    );
                }
            }
        }

        if variant.attrs.deserialize_with().is_some() {
            if variant.attrs.skip_deserializing() {
                cx.error_spanned_by(
                    variant.original,
                    format!(
                        "variant `{}` cannot have both #[serde(deserialize_with)] and #[serde(skip_deserializing)]",
                        variant.ident
                    ),
                );
            }

            for field in &variant.fields {
                if field.attrs.skip_deserializing() {
                    let member = member_message(&field.member);

                    cx.error_spanned_by(
                        variant.original,
                        format!(
                            "variant `{}` cannot have both #[serde(deserialize_with)] and a field {} marked with #[serde(skip_deserializing)]",
                            variant.ident, member
                        ),
                    );
                }
            }
        }
    }
}