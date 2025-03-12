fn class_chars(hirs: &[Hir]) -> Option<Class> {
    let mut cls = ClassUnicode::new(vec![]);
    for hir in hirs.iter() {
        match *hir.kind() {
            HirKind::Class(Class::Unicode(ref cls2)) => {
                cls.union(cls2);
            }
            HirKind::Class(Class::Bytes(ref cls2)) => {
                cls.union(&cls2.to_unicode_class()?);
            }
            _ => return None,
        };
    }
    Some(Class::Unicode(cls))
}