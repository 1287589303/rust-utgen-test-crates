fn singleton_bytes(hirs: &[Hir]) -> Option<Vec<u8>> {
    let mut singletons = vec![];
    for hir in hirs.iter() {
        let literal = match *hir.kind() {
            HirKind::Literal(Literal(ref bytes)) => bytes,
            _ => return None,
        };
        if literal.len() != 1 {
            return None;
        }
        singletons.push(literal[0]);
    }
    Some(singletons)
}