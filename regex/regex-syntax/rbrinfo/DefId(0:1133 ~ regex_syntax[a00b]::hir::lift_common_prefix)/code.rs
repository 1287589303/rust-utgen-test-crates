fn lift_common_prefix(hirs: Vec<Hir>) -> Result<Hir, Vec<Hir>> {
    if hirs.len() <= 1 {
        return Err(hirs);
    }
    let mut prefix = match hirs[0].kind() {
        HirKind::Concat(ref xs) => &**xs,
        _ => return Err(hirs),
    };
    if prefix.is_empty() {
        return Err(hirs);
    }
    for h in hirs.iter().skip(1) {
        let concat = match h.kind() {
            HirKind::Concat(ref xs) => xs,
            _ => return Err(hirs),
        };
        let common_len = prefix
            .iter()
            .zip(concat.iter())
            .take_while(|(x, y)| x == y)
            .count();
        prefix = &prefix[..common_len];
        if prefix.is_empty() {
            return Err(hirs);
        }
    }
    let len = prefix.len();
    assert_ne!(0, len);
    let mut prefix_concat = vec![];
    let mut suffix_alts = vec![];
    for h in hirs {
        let mut concat = match h.into_kind() {
            HirKind::Concat(xs) => xs,
            // We required all sub-expressions to be
            // concats above, so we're only here if we
            // have a concat.
            _ => unreachable!(),
        };
        suffix_alts.push(Hir::concat(concat.split_off(len)));
        if prefix_concat.is_empty() {
            prefix_concat = concat;
        }
    }
    let mut concat = prefix_concat;
    concat.push(Hir::alternation(suffix_alts));
    Ok(Hir::concat(concat))
}