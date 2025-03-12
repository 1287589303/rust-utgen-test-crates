fn build(mut self) -> Result<DFA, BuildError> {
        self.nfa.look_set_any().available().map_err(BuildError::word)?;
        for look in self.nfa.look_set_any().iter() {
            // This is a future incompatibility check where if we add any
            // more look-around assertions, then the one-pass DFA either
            // needs to reject them (what we do here) or it needs to have its
            // Transition representation modified to be capable of storing the
            // new assertions.
            if look.as_repr() > Look::WordUnicodeNegate.as_repr() {
                return Err(BuildError::unsupported_look(look));
            }
        }
        if self.nfa.pattern_len().as_u64() > PatternEpsilons::PATTERN_ID_LIMIT
        {
            return Err(BuildError::too_many_patterns(
                PatternEpsilons::PATTERN_ID_LIMIT,
            ));
        }
        if self.nfa.group_info().explicit_slot_len() > Slots::LIMIT {
            return Err(BuildError::not_one_pass(
                "too many explicit capturing groups (max is 16)",
            ));
        }
        assert_eq!(DEAD, self.add_empty_state()?);

        // This is where the explicit slots start. We care about this because
        // we only need to track explicit slots. The implicit slots---two for
        // each pattern---are tracked as part of the search routine itself.
        let explicit_slot_start = self.nfa.pattern_len() * 2;
        self.add_start_state(None, self.nfa.start_anchored())?;
        if self.config.get_starts_for_each_pattern() {
            for pid in self.nfa.patterns() {
                self.add_start_state(
                    Some(pid),
                    self.nfa.start_pattern(pid).unwrap(),
                )?;
            }
        }
        // NOTE: One wonders what the effects of treating 'uncompiled_nfa_ids'
        // as a stack are. It is really an unordered *set* of NFA state IDs.
        // If it, for example, in practice led to discovering whether a regex
        // was or wasn't one-pass later than if we processed NFA state IDs in
        // ascending order, then that would make this routine more costly in
        // the somewhat common case of a regex that isn't one-pass.
        while let Some(nfa_id) = self.uncompiled_nfa_ids.pop() {
            let dfa_id = self.nfa_to_dfa_id[nfa_id];
            // Once we see a match, we keep going, but don't add any new
            // transitions. Normally we'd just stop, but we have to keep
            // going in order to verify that our regex is actually one-pass.
            self.matched = false;
            // The NFA states we've already explored for this DFA state.
            self.seen.clear();
            // The NFA states to explore via epsilon transitions. If we ever
            // try to push an NFA state that we've already seen, then the NFA
            // is not one-pass because it implies there are multiple epsilon
            // transition paths that lead to the same NFA state. In other
            // words, there is ambiguity.
            self.stack_push(nfa_id, Epsilons::empty())?;
            while let Some((id, epsilons)) = self.stack.pop() {
                match *self.nfa.state(id) {
                    thompson::State::ByteRange { ref trans } => {
                        self.compile_transition(dfa_id, trans, epsilons)?;
                    }
                    thompson::State::Sparse(ref sparse) => {
                        for trans in sparse.transitions.iter() {
                            self.compile_transition(dfa_id, trans, epsilons)?;
                        }
                    }
                    thompson::State::Dense(ref dense) => {
                        for trans in dense.iter() {
                            self.compile_transition(dfa_id, &trans, epsilons)?;
                        }
                    }
                    thompson::State::Look { look, next } => {
                        let looks = epsilons.looks().insert(look);
                        self.stack_push(next, epsilons.set_looks(looks))?;
                    }
                    thompson::State::Union { ref alternates } => {
                        for &sid in alternates.iter().rev() {
                            self.stack_push(sid, epsilons)?;
                        }
                    }
                    thompson::State::BinaryUnion { alt1, alt2 } => {
                        self.stack_push(alt2, epsilons)?;
                        self.stack_push(alt1, epsilons)?;
                    }
                    thompson::State::Capture { next, slot, .. } => {
                        let slot = slot.as_usize();
                        let epsilons = if slot < explicit_slot_start {
                            // If this is an implicit slot, we don't care
                            // about it, since we handle implicit slots in
                            // the search routine. We can get away with that
                            // because there are 2 implicit slots for every
                            // pattern.
                            epsilons
                        } else {
                            // Offset our explicit slots so that they start
                            // at index 0.
                            let offset = slot - explicit_slot_start;
                            epsilons.set_slots(epsilons.slots().insert(offset))
                        };
                        self.stack_push(next, epsilons)?;
                    }
                    thompson::State::Fail => {
                        continue;
                    }
                    thompson::State::Match { pattern_id } => {
                        // If we found two different paths to a match state
                        // for the same DFA state, then we have ambiguity.
                        // Thus, it's not one-pass.
                        if self.matched {
                            return Err(BuildError::not_one_pass(
                                "multiple epsilon transitions to match state",
                            ));
                        }
                        self.matched = true;
                        // Shove the matching pattern ID and the 'epsilons'
                        // into the current DFA state's pattern epsilons. The
                        // 'epsilons' includes the slots we need to capture
                        // before reporting the match and also the conditional
                        // epsilon transitions we need to check before we can
                        // report a match.
                        self.dfa.set_pattern_epsilons(
                            dfa_id,
                            PatternEpsilons::empty()
                                .set_pattern_id(pattern_id)
                                .set_epsilons(epsilons),
                        );
                        // N.B. It is tempting to just bail out here when
                        // compiling a leftmost-first DFA, since we will never
                        // compile any more transitions in that case. But we
                        // actually need to keep going in order to verify that
                        // we actually have a one-pass regex. e.g., We might
                        // see more Match states (e.g., for other patterns)
                        // that imply that we don't have a one-pass regex.
                        // So instead, we mark that we've found a match and
                        // continue on. When we go to compile a new DFA state,
                        // we just skip that part. But otherwise check that the
                        // one-pass property is upheld.
                    }
                }
            }
        }
        self.shuffle_states();
        Ok(self.dfa)
    }