fn compile(self, hir: &Hir) -> Result<NFA, Error> {
        self.nfa.borrow_mut().is_start_anchored = hir.is_start_anchored();
        self.nfa.borrow_mut().is_match_empty = hir.is_match_empty();
        self.nfa.borrow_mut().static_explicit_captures_len =
            hir.static_explicit_captures_len();
        let compiled = self.c_capture(0, None, hir)?;
        let mat = self.add(State::Match)?;
        self.patch(compiled.end, mat)?;
        self.nfa.borrow_mut().start = compiled.start;
        Ok(self.nfa.into_inner())
    }