pub fn concat(subs: Vec<Hir>) -> Hir {
        // We rebuild the concatenation by simplifying it. Would be nice to do
        // it in place, but that seems a little tricky?
        let mut new = vec![];
        // This gobbles up any adjacent literals in a concatenation and smushes
        // them together. Basically, when we see a literal, we add its bytes
        // to 'prior_lit', and whenever we see anything else, we first take
        // any bytes in 'prior_lit' and add it to the 'new' concatenation.
        let mut prior_lit: Option<Vec<u8>> = None;
        for sub in subs {
            let (kind, props) = sub.into_parts();
            match kind {
                HirKind::Literal(Literal(bytes)) => {
                    if let Some(ref mut prior_bytes) = prior_lit {
                        prior_bytes.extend_from_slice(&bytes);
                    } else {
                        prior_lit = Some(bytes.to_vec());
                    }
                }
                // We also flatten concats that are direct children of another
                // concat. We only need to do this one level deep since
                // Hir::concat is the only way to build concatenations, and so
                // flattening happens inductively.
                HirKind::Concat(subs2) => {
                    for sub2 in subs2 {
                        let (kind2, props2) = sub2.into_parts();
                        match kind2 {
                            HirKind::Literal(Literal(bytes)) => {
                                if let Some(ref mut prior_bytes) = prior_lit {
                                    prior_bytes.extend_from_slice(&bytes);
                                } else {
                                    prior_lit = Some(bytes.to_vec());
                                }
                            }
                            kind2 => {
                                if let Some(prior_bytes) = prior_lit.take() {
                                    new.push(Hir::literal(prior_bytes));
                                }
                                new.push(Hir { kind: kind2, props: props2 });
                            }
                        }
                    }
                }
                // We can just skip empty HIRs.
                HirKind::Empty => {}
                kind => {
                    if let Some(prior_bytes) = prior_lit.take() {
                        new.push(Hir::literal(prior_bytes));
                    }
                    new.push(Hir { kind, props });
                }
            }
        }
        if let Some(prior_bytes) = prior_lit.take() {
            new.push(Hir::literal(prior_bytes));
        }
        if new.is_empty() {
            return Hir::empty();
        } else if new.len() == 1 {
            return new.pop().unwrap();
        }
        let props = Properties::concat(&new);
        Hir { kind: HirKind::Concat(new), props }
    }