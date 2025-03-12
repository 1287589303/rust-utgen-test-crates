pub(crate) fn is_ident_start(c: char) -> bool {
    c == '_' || unicode_ident::is_xid_start(c)
}