fn into_class_item_range(hir: Hir) -> Result<char, Error> {
    match hir.kind {
        HirKind::Char(ch) => Ok(ch),
        _ => Err(Error::new(ERR_CLASS_INVALID_RANGE_ITEM)),
    }
}