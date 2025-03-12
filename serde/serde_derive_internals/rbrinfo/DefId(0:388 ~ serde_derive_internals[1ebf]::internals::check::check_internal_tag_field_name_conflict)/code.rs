fn check_internal_tag_field_name_conflict(cx: &Ctxt, cont: &Container) {
    let variants = match &cont.data {
        Data::Enum(variants) => variants,
        Data::Struct(_, _) => return,
    };

    let tag = match cont.attrs.tag() {
        TagType::Internal { tag } => tag.as_str(),
        TagType::External | TagType::Adjacent { .. } | TagType::None => return,
    };

    let diagnose_conflict = || {
        cx.error_spanned_by(
            cont.original,
            format!("variant field name `{}` conflicts with internal tag", tag),
        );
    };

    for variant in variants {
        match variant.style {
            Style::Struct => {
                if variant.attrs.untagged() {
                    continue;
                }
                for field in &variant.fields {
                    let check_ser =
                        !(field.attrs.skip_serializing() || variant.attrs.skip_serializing());
                    let check_de =
                        !(field.attrs.skip_deserializing() || variant.attrs.skip_deserializing());
                    let name = field.attrs.name();
                    let ser_name = name.serialize_name();

                    if check_ser && ser_name.value == tag {
                        diagnose_conflict();
                        return;
                    }

                    for de_name in field.attrs.aliases() {
                        if check_de && de_name.value == tag {
                            diagnose_conflict();
                            return;
                        }
                    }
                }
            }
            Style::Unit | Style::Newtype | Style::Tuple => {}
        }
    }
}