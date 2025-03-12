fn decide_identifier(
    cx: &Ctxt,
    item: &syn::DeriveInput,
    field_identifier: BoolAttr,
    variant_identifier: BoolAttr,
) -> Identifier {
    match (
        &item.data,
        field_identifier.0.get_with_tokens(),
        variant_identifier.0.get_with_tokens(),
    ) {
        (_, None, None) => Identifier::No,
        (_, Some((field_identifier_tokens, ())), Some((variant_identifier_tokens, ()))) => {
            let msg =
                "#[serde(field_identifier)] and #[serde(variant_identifier)] cannot both be set";
            cx.error_spanned_by(field_identifier_tokens, msg);
            cx.error_spanned_by(variant_identifier_tokens, msg);
            Identifier::No
        }
        (syn::Data::Enum(_), Some(_), None) => Identifier::Field,
        (syn::Data::Enum(_), None, Some(_)) => Identifier::Variant,
        (syn::Data::Struct(syn::DataStruct { struct_token, .. }), Some(_), None) => {
            let msg = "#[serde(field_identifier)] can only be used on an enum";
            cx.error_spanned_by(struct_token, msg);
            Identifier::No
        }
        (syn::Data::Union(syn::DataUnion { union_token, .. }), Some(_), None) => {
            let msg = "#[serde(field_identifier)] can only be used on an enum";
            cx.error_spanned_by(union_token, msg);
            Identifier::No
        }
        (syn::Data::Struct(syn::DataStruct { struct_token, .. }), None, Some(_)) => {
            let msg = "#[serde(variant_identifier)] can only be used on an enum";
            cx.error_spanned_by(struct_token, msg);
            Identifier::No
        }
        (syn::Data::Union(syn::DataUnion { union_token, .. }), None, Some(_)) => {
            let msg = "#[serde(variant_identifier)] can only be used on an enum";
            cx.error_spanned_by(union_token, msg);
            Identifier::No
        }
    }
}