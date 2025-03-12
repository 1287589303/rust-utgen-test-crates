pub(crate) fn is_ident_continue(c: char) -> bool {
    unicode_ident::is_xid_continue(c)
}