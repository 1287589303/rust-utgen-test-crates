fn check_flatten(cx: &Ctxt, cont: &Container) {
    match &cont.data {
        Data::Enum(variants) => {
            for variant in variants {
                for field in &variant.fields {
                    check_flatten_field(cx, variant.style, field);
                }
            }
        }
        Data::Struct(style, fields) => {
            for field in fields {
                check_flatten_field(cx, *style, field);
            }
        }
    }
}