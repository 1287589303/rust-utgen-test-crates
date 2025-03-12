fn compile<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError> {
        if exprs.len() > PatternID::LIMIT {
            return Err(BuildError::too_many_patterns(exprs.len()));
        }
        if self.config.get_reverse()
            && self.config.get_which_captures().is_any()
        {
            return Err(BuildError::unsupported_captures());
        }

        self.builder.borrow_mut().clear();
        self.builder.borrow_mut().set_utf8(self.config.get_utf8());
        self.builder.borrow_mut().set_reverse(self.config.get_reverse());
        self.builder
            .borrow_mut()
            .set_look_matcher(self.config.get_look_matcher());
        self.builder
            .borrow_mut()
            .set_size_limit(self.config.get_nfa_size_limit())?;

        // We always add an unanchored prefix unless we were specifically told
        // not to (for tests only), or if we know that the regex is anchored
        // for all matches. When an unanchored prefix is not added, then the
        // NFA's anchored and unanchored start states are equivalent.
        let all_anchored = exprs.iter().all(|e| {
            let props = e.borrow().properties();
            if self.config.get_reverse() {
                props.look_set_suffix().contains(hir::Look::End)
            } else {
                props.look_set_prefix().contains(hir::Look::Start)
            }
        });
        let anchored = !self.config.get_unanchored_prefix() || all_anchored;
        let unanchored_prefix = if anchored {
            self.c_empty()?
        } else {
            self.c_at_least(&Hir::dot(hir::Dot::AnyByte), false, 0)?
        };

        let compiled = self.c_alt_iter(exprs.iter().map(|e| {
            let _ = self.start_pattern()?;
            let one = self.c_cap(0, None, e.borrow())?;
            let match_state_id = self.add_match()?;
            self.patch(one.end, match_state_id)?;
            let _ = self.finish_pattern(one.start)?;
            Ok(ThompsonRef { start: one.start, end: match_state_id })
        }))?;
        self.patch(unanchored_prefix.end, compiled.start)?;
        let nfa = self
            .builder
            .borrow_mut()
            .build(compiled.start, unanchored_prefix.start)?;

        debug!("HIR-to-NFA compilation complete, config: {:?}", self.config);
        Ok(nfa)
    }