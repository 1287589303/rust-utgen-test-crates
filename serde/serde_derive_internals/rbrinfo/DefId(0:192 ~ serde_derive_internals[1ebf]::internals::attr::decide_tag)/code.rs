fn decide_tag(
    cx: &Ctxt,
    item: &syn::DeriveInput,
    untagged: BoolAttr,
    internal_tag: Attr<String>,
    content: Attr<String>,
) -> TagType {
    match (
        untagged.0.get_with_tokens(),
        internal_tag.get_with_tokens(),
        content.get_with_tokens(),
    ) {
        (None, None, None) => TagType::External,
        (Some(_), None, None) => TagType::None,
        (None, Some((_, tag)), None) => {
            // Check that there are no tuple variants.
            if let syn::Data::Enum(data) = &item.data {
                for variant in &data.variants {
                    match &variant.fields {
                        syn::Fields::Named(_) | syn::Fields::Unit => {}
                        syn::Fields::Unnamed(fields) => {
                            if fields.unnamed.len() != 1 {
                                let msg =
                                    "#[serde(tag = \"...\")] cannot be used with tuple variants";
                                cx.error_spanned_by(variant, msg);
                                break;
                            }
                        }
                    }
                }
            }
            TagType::Internal { tag }
        }
        (Some((untagged_tokens, ())), Some((tag_tokens, _)), None) => {
            let msg = "enum cannot be both untagged and internally tagged";
            cx.error_spanned_by(untagged_tokens, msg);
            cx.error_spanned_by(tag_tokens, msg);
            TagType::External // doesn't matter, will error
        }
        (None, None, Some((content_tokens, _))) => {
            let msg = "#[serde(tag = \"...\", content = \"...\")] must be used together";
            cx.error_spanned_by(content_tokens, msg);
            TagType::External
        }
        (Some((untagged_tokens, ())), None, Some((content_tokens, _))) => {
            let msg = "untagged enum cannot have #[serde(content = \"...\")]";
            cx.error_spanned_by(untagged_tokens, msg);
            cx.error_spanned_by(content_tokens, msg);
            TagType::External
        }
        (None, Some((_, tag)), Some((_, content))) => TagType::Adjacent { tag, content },
        (Some((untagged_tokens, ())), Some((tag_tokens, _)), Some((content_tokens, _))) => {
            let msg = "untagged enum cannot have #[serde(tag = \"...\", content = \"...\")]";
            cx.error_spanned_by(untagged_tokens, msg);
            cx.error_spanned_by(tag_tokens, msg);
            cx.error_spanned_by(content_tokens, msg);
            TagType::External
        }
    }
}