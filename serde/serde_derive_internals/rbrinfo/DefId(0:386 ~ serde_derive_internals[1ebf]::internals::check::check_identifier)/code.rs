fn check_identifier(cx: &Ctxt, cont: &Container) {
    let variants = match &cont.data {
        Data::Enum(variants) => variants,
        Data::Struct(_, _) => return,
    };

    for (i, variant) in variants.iter().enumerate() {
        match (
            variant.style,
            cont.attrs.identifier(),
            variant.attrs.other(),
            cont.attrs.tag(),
        ) {
            // The `other` attribute may not be used in a variant_identifier.
            (_, Identifier::Variant, true, _) => {
                cx.error_spanned_by(
                    variant.original,
                    "#[serde(other)] may not be used on a variant identifier",
                );
            }

            // Variant with `other` attribute cannot appear in untagged enum
            (_, Identifier::No, true, &TagType::None) => {
                cx.error_spanned_by(
                    variant.original,
                    "#[serde(other)] cannot appear on untagged enum",
                );
            }

            // Variant with `other` attribute must be the last one.
            (Style::Unit, Identifier::Field, true, _) | (Style::Unit, Identifier::No, true, _) => {
                if i < variants.len() - 1 {
                    cx.error_spanned_by(
                        variant.original,
                        "#[serde(other)] must be on the last variant",
                    );
                }
            }

            // Variant with `other` attribute must be a unit variant.
            (_, Identifier::Field, true, _) | (_, Identifier::No, true, _) => {
                cx.error_spanned_by(
                    variant.original,
                    "#[serde(other)] must be on a unit variant",
                );
            }

            // Any sort of variant is allowed if this is not an identifier.
            (_, Identifier::No, false, _) => {}

            // Unit variant without `other` attribute is always fine.
            (Style::Unit, _, false, _) => {}

            // The last field is allowed to be a newtype catch-all.
            (Style::Newtype, Identifier::Field, false, _) => {
                if i < variants.len() - 1 {
                    cx.error_spanned_by(
                        variant.original,
                        format!("`{}` must be the last variant", variant.ident),
                    );
                }
            }

            (_, Identifier::Field, false, _) => {
                cx.error_spanned_by(
                    variant.original,
                    "#[serde(field_identifier)] may only contain unit variants",
                );
            }

            (_, Identifier::Variant, false, _) => {
                cx.error_spanned_by(
                    variant.original,
                    "#[serde(variant_identifier)] may only contain unit variants",
                );
            }
        }
    }
}