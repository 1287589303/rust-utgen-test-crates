fn run(mut self) -> Result<(), BuildError> {
        if self.nfa.look_set_any().contains_word_unicode()
            && !self.config.quit.contains_range(0x80, 0xFF)
        {
            return Err(BuildError::unsupported_dfa_word_boundary_unicode());
        }

        // A sequence of "representative" bytes drawn from each equivalence
        // class. These representative bytes are fed to the NFA to compute
        // state transitions. This allows us to avoid re-computing state
        // transitions for bytes that are guaranteed to produce identical
        // results. Since computing the representatives needs to do a little
        // work, we do it once here because we'll be iterating over them a lot.
        let representatives: Vec<alphabet::Unit> =
            self.dfa.byte_classes().representatives(..).collect();
        // The set of all DFA state IDs that still need to have their
        // transitions set. We start by seeding this with all starting states.
        let mut uncompiled = alloc::vec![];
        self.add_all_starts(&mut uncompiled)?;
        while let Some(dfa_id) = uncompiled.pop() {
            for &unit in &representatives {
                if unit.as_u8().map_or(false, |b| self.config.quit.contains(b))
                {
                    continue;
                }
                // In many cases, the state we transition to has already been
                // computed. 'cached_state' will do the minimal amount of work
                // to check this, and if it exists, immediately return an
                // already existing state ID.
                let (next_dfa_id, is_new) = self.cached_state(dfa_id, unit)?;
                self.dfa.set_transition(dfa_id, unit, next_dfa_id);
                // If the state ID we got back is newly created, then we need
                // to compile it, so add it to our uncompiled frontier.
                if is_new {
                    uncompiled.push(next_dfa_id);
                }
            }
        }
        debug!(
            "determinization complete, memory usage: {}, \
             dense DFA size: {}, \
             is reverse? {}",
            self.memory_usage(),
            self.dfa.memory_usage(),
            self.nfa.is_reverse(),
        );

        // A map from DFA state ID to one or more NFA match IDs. Each NFA match
        // ID corresponds to a distinct regex pattern that matches in the state
        // corresponding to the key.
        let mut matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
        self.cache.clear();
        #[cfg(feature = "logging")]
        let mut total_pat_len = 0;
        for (i, state) in self.builder_states.into_iter().enumerate() {
            if let Some(pat_ids) = state.match_pattern_ids() {
                let id = self.dfa.to_state_id(i);
                log! {
                    total_pat_len += pat_ids.len();
                }
                matches.insert(id, pat_ids);
            }
        }
        log! {
            use core::mem::size_of;
            let per_elem = size_of::<StateID>() + size_of::<Vec<PatternID>>();
            let pats = total_pat_len * size_of::<PatternID>();
            let mem = (matches.len() * per_elem) + pats;
            log::debug!("matches map built, memory usage: {}", mem);
        }
        // At this point, we shuffle the "special" states in the final DFA.
        // This permits a DFA's match loop to detect a match condition (among
        // other things) by merely inspecting the current state's identifier,
        // and avoids the need for any additional auxiliary storage.
        self.dfa.shuffle(matches)?;
        Ok(())
    }