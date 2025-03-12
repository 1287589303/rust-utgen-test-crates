fn c_at_least(
        &self,
        hir: &Hir,
        greedy: bool,
        n: u32,
    ) -> Result<ThompsonRef, Error> {
        if n == 0 {
            // When the expression cannot match the empty string, then we
            // can get away with something much simpler: just one 'alt'
            // instruction that optionally repeats itself. But if the expr
            // can match the empty string... see below.
            if !hir.is_match_empty() {
                let splits = self.add(State::Splits {
                    targets: vec![],
                    reverse: !greedy,
                })?;
                let compiled = self.c(hir)?;
                self.patch(splits, compiled.start)?;
                self.patch(compiled.end, splits)?;
                return Ok(ThompsonRef { start: splits, end: splits });
            }

            // What's going on here? Shouldn't x* be simpler than this? It
            // turns out that when implementing leftmost-first (Perl-like)
            // match semantics, x* results in an incorrect preference order
            // when computing the transitive closure of states if and only if
            // 'x' can match the empty string. So instead, we compile x* as
            // (x+)?, which preserves the correct preference order.
            //
            // See: https://github.com/rust-lang/regex/issues/779
            let compiled = self.c(hir)?;
            let plus =
                self.add(State::Splits { targets: vec![], reverse: !greedy })?;
            self.patch(compiled.end, plus)?;
            self.patch(plus, compiled.start)?;

            let question =
                self.add(State::Splits { targets: vec![], reverse: !greedy })?;
            let empty = self.add_empty()?;
            self.patch(question, compiled.start)?;
            self.patch(question, empty)?;
            self.patch(plus, empty)?;
            Ok(ThompsonRef { start: question, end: empty })
        } else if n == 1 {
            let compiled = self.c(hir)?;
            let splits =
                self.add(State::Splits { targets: vec![], reverse: !greedy })?;
            self.patch(compiled.end, splits)?;
            self.patch(splits, compiled.start)?;
            Ok(ThompsonRef { start: compiled.start, end: splits })
        } else {
            let prefix = self.c_exactly(hir, n - 1)?;
            let last = self.c(hir)?;
            let splits =
                self.add(State::Splits { targets: vec![], reverse: !greedy })?;
            self.patch(prefix.end, last.start)?;
            self.patch(last.end, splits)?;
            self.patch(splits, last.start)?;
            Ok(ThompsonRef { start: prefix.start, end: splits })
        }
    }