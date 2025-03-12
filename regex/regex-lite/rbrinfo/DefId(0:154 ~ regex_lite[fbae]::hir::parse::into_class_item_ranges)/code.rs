fn into_class_item_ranges(
    mut hir: Hir,
) -> Result<Vec<hir::ClassRange>, Error> {
    match core::mem::replace(&mut hir.kind, HirKind::Empty) {
        HirKind::Char(ch) => Ok(vec![hir::ClassRange { start: ch, end: ch }]),
        HirKind::Class(hir::Class { ranges }) => Ok(ranges),
        _ => Err(Error::new(ERR_CLASS_INVALID_ITEM)),
    }
}