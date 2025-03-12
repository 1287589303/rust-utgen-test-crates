fn class_bytes(hirs: &[Hir]) -> Option<Class> {
    let mut cls = ClassBytes::new(vec![]);
    for hir in hirs.iter() {
        match *hir.kind() {
            HirKind::Class(Class::Unicode(ref cls2)) => {
                cls.union(&cls2.to_byte_class()?);
            }
            HirKind::Class(Class::Bytes(ref cls2)) => {
                cls.union(cls2);
            }
            _ => return None,
        };
    }
    Some(Class::Bytes(cls))
}