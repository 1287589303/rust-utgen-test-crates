fn singleton_chars(hirs: &[Hir]) -> Option<Vec<char>> {
    let mut singletons = vec![];
    for hir in hirs.iter() {
        let literal = match *hir.kind() {
            HirKind::Literal(Literal(ref bytes)) => bytes,
            _ => return None,
        };
        let ch = match crate::debug::utf8_decode(literal) {
            None => return None,
            Some(Err(_)) => return None,
            Some(Ok(ch)) => ch,
        };
        if literal.len() != ch.len_utf8() {
            return None;
        }
        singletons.push(ch);
    }
    Some(singletons)
}