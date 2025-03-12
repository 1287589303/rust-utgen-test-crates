pub fn alternation(subs: Vec<Hir>) -> Hir {
        // We rebuild the alternation by simplifying it. We proceed similarly
        // as the concatenation case. But in this case, there's no literal
        // simplification happening. We're just flattening alternations.
        let mut new = Vec::with_capacity(subs.len());
        for sub in subs {
            let (kind, props) = sub.into_parts();
            match kind {
                HirKind::Alternation(subs2) => {
                    new.extend(subs2);
                }
                kind => {
                    new.push(Hir { kind, props });
                }
            }
        }
        if new.is_empty() {
            return Hir::fail();
        } else if new.len() == 1 {
            return new.pop().unwrap();
        }
        // Now that it's completely flattened, look for the special case of
        // 'char1|char2|...|charN' and collapse that into a class. Note that
        // we look for 'char' first and then bytes. The issue here is that if
        // we find both non-ASCII codepoints and non-ASCII singleton bytes,
        // then it isn't actually possible to smush them into a single class.
        // (Because classes are either "all codepoints" or "all bytes." You
        // can have a class that both matches non-ASCII but valid UTF-8 and
        // invalid UTF-8.) So we look for all chars and then all bytes, and
        // don't handle anything else.
        if let Some(singletons) = singleton_chars(&new) {
            let it = singletons
                .into_iter()
                .map(|ch| ClassUnicodeRange { start: ch, end: ch });
            return Hir::class(Class::Unicode(ClassUnicode::new(it)));
        }
        if let Some(singletons) = singleton_bytes(&new) {
            let it = singletons
                .into_iter()
                .map(|b| ClassBytesRange { start: b, end: b });
            return Hir::class(Class::Bytes(ClassBytes::new(it)));
        }
        // Similar to singleton chars, we can also look for alternations of
        // classes. Those can be smushed into a single class.
        if let Some(cls) = class_chars(&new) {
            return Hir::class(cls);
        }
        if let Some(cls) = class_bytes(&new) {
            return Hir::class(cls);
        }
        // Factor out a common prefix if we can, which might potentially
        // simplify the expression and unlock other optimizations downstream.
        // It also might generally make NFA matching and DFA construction
        // faster by reducing the scope of branching in the regex.
        new = match lift_common_prefix(new) {
            Ok(hir) => return hir,
            Err(unchanged) => unchanged,
        };
        let props = Properties::alternation(&new);
        Hir { kind: HirKind::Alternation(new), props }
    }