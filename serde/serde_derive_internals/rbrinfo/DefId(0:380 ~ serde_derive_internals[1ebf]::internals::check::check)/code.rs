pub fn check(cx: &Ctxt, cont: &mut Container, derive: Derive) {
    check_default_on_tuple(cx, cont);
    check_remote_generic(cx, cont);
    check_getter(cx, cont);
    check_flatten(cx, cont);
    check_identifier(cx, cont);
    check_variant_skip_attrs(cx, cont);
    check_internal_tag_field_name_conflict(cx, cont);
    check_adjacent_tag_conflict(cx, cont);
    check_transparent(cx, cont, derive);
    check_from_and_try_from(cx, cont);
}