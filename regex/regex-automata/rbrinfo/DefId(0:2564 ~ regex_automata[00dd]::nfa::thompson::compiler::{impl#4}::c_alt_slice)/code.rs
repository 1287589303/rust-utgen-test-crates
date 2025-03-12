fn c_alt_slice(&self, exprs: &[Hir]) -> Result<ThompsonRef, BuildError> {
        // self.c_alt_iter(exprs.iter().map(|e| self.c(e)))
        let literal_count = exprs
            .iter()
            .filter(|e| {
                matches!(*e.kind(), hir::HirKind::Literal(hir::Literal(_)))
            })
            .count();
        if literal_count <= 1 || literal_count < exprs.len() {
            return self.c_alt_iter(exprs.iter().map(|e| self.c(e)));
        }

        let mut trie = if self.is_reverse() {
            LiteralTrie::reverse()
        } else {
            LiteralTrie::forward()
        };
        for expr in exprs.iter() {
            let literal = match *expr.kind() {
                hir::HirKind::Literal(hir::Literal(ref bytes)) => bytes,
                _ => unreachable!(),
            };
            trie.add(literal)?;
        }
        trie.compile(&mut self.builder.borrow_mut())
    }