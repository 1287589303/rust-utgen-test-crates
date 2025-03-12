pub(crate) fn accelerate(&mut self) {
        // dead and quit states can never be accelerated.
        if self.state_len() <= 2 {
            return;
        }

        // Go through every state and record their accelerator, if possible.
        let mut accels = BTreeMap::new();
        // Count the number of accelerated match, start and non-match/start
        // states.
        let (mut cmatch, mut cstart, mut cnormal) = (0, 0, 0);
        for state in self.states() {
            if let Some(accel) = state.accelerate(self.byte_classes()) {
                debug!(
                    "accelerating full DFA state {}: {:?}",
                    state.id().as_usize(),
                    accel,
                );
                accels.insert(state.id(), accel);
                if self.is_match_state(state.id()) {
                    cmatch += 1;
                } else if self.is_start_state(state.id()) {
                    cstart += 1;
                } else {
                    assert!(!self.is_dead_state(state.id()));
                    assert!(!self.is_quit_state(state.id()));
                    cnormal += 1;
                }
            }
        }
        // If no states were able to be accelerated, then we're done.
        if accels.is_empty() {
            return;
        }
        let original_accels_len = accels.len();

        // A remapper keeps track of state ID changes. Once we're done
        // shuffling, the remapper is used to rewrite all transitions in the
        // DFA based on the new positions of states.
        let mut remapper = Remapper::new(self);

        // As we swap states, if they are match states, we need to swap their
        // pattern ID lists too (for multi-regexes). We do this by converting
        // the lists to an easily swappable map, and then convert back to
        // MatchStates once we're done.
        let mut new_matches = self.ms.to_map(self);

        // There is at least one state that gets accelerated, so these are
        // guaranteed to get set to sensible values below.
        self.special.min_accel = StateID::MAX;
        self.special.max_accel = StateID::ZERO;
        let update_special_accel =
            |special: &mut Special, accel_id: StateID| {
                special.min_accel = cmp::min(special.min_accel, accel_id);
                special.max_accel = cmp::max(special.max_accel, accel_id);
            };

        // Start by shuffling match states. Any match states that are
        // accelerated get moved to the end of the match state range.
        if cmatch > 0 && self.special.matches() {
            // N.B. special.{min,max}_match do not need updating, since the
            // range/number of match states does not change. Only the ordering
            // of match states may change.
            let mut next_id = self.special.max_match;
            let mut cur_id = next_id;
            while cur_id >= self.special.min_match {
                if let Some(accel) = accels.remove(&cur_id) {
                    accels.insert(next_id, accel);
                    update_special_accel(&mut self.special, next_id);

                    // No need to do any actual swapping for equivalent IDs.
                    if cur_id != next_id {
                        remapper.swap(self, cur_id, next_id);

                        // Swap pattern IDs for match states.
                        let cur_pids = new_matches.remove(&cur_id).unwrap();
                        let next_pids = new_matches.remove(&next_id).unwrap();
                        new_matches.insert(cur_id, next_pids);
                        new_matches.insert(next_id, cur_pids);
                    }
                    next_id = self.tt.prev_state_id(next_id);
                }
                cur_id = self.tt.prev_state_id(cur_id);
            }
        }

        // This is where it gets tricky. Without acceleration, start states
        // normally come right after match states. But we want accelerated
        // states to be a single contiguous range (to make it very fast
        // to determine whether a state *is* accelerated), while also keeping
        // match and starting states as contiguous ranges for the same reason.
        // So what we do here is shuffle states such that it looks like this:
        //
        //     DQMMMMAAAAASSSSSSNNNNNNN
        //         |         |
        //         |---------|
        //      accelerated states
        //
        // Where:
        //   D - dead state
        //   Q - quit state
        //   M - match state (may be accelerated)
        //   A - normal state that is accelerated
        //   S - start state (may be accelerated)
        //   N - normal state that is NOT accelerated
        //
        // We implement this by shuffling states, which is done by a sequence
        // of pairwise swaps. We start by looking at all normal states to be
        // accelerated. When we find one, we swap it with the earliest starting
        // state, and then swap that with the earliest normal state. This
        // preserves the contiguous property.
        //
        // Once we're done looking for accelerated normal states, now we look
        // for accelerated starting states by moving them to the beginning
        // of the starting state range (just like we moved accelerated match
        // states to the end of the matching state range).
        //
        // For a more detailed/different perspective on this, see the docs
        // in dfa/special.rs.
        if cnormal > 0 {
            // our next available starting and normal states for swapping.
            let mut next_start_id = self.special.min_start;
            let mut cur_id = self.to_state_id(self.state_len() - 1);
            // This is guaranteed to exist since cnormal > 0.
            let mut next_norm_id =
                self.tt.next_state_id(self.special.max_start);
            while cur_id >= next_norm_id {
                if let Some(accel) = accels.remove(&cur_id) {
                    remapper.swap(self, next_start_id, cur_id);
                    remapper.swap(self, next_norm_id, cur_id);
                    // Keep our accelerator map updated with new IDs if the
                    // states we swapped were also accelerated.
                    if let Some(accel2) = accels.remove(&next_norm_id) {
                        accels.insert(cur_id, accel2);
                    }
                    if let Some(accel2) = accels.remove(&next_start_id) {
                        accels.insert(next_norm_id, accel2);
                    }
                    accels.insert(next_start_id, accel);
                    update_special_accel(&mut self.special, next_start_id);
                    // Our start range shifts one to the right now.
                    self.special.min_start =
                        self.tt.next_state_id(self.special.min_start);
                    self.special.max_start =
                        self.tt.next_state_id(self.special.max_start);
                    next_start_id = self.tt.next_state_id(next_start_id);
                    next_norm_id = self.tt.next_state_id(next_norm_id);
                }
                // This is pretty tricky, but if our 'next_norm_id' state also
                // happened to be accelerated, then the result is that it is
                // now in the position of cur_id, so we need to consider it
                // again. This loop is still guaranteed to terminate though,
                // because when accels contains cur_id, we're guaranteed to
                // increment next_norm_id even if cur_id remains unchanged.
                if !accels.contains_key(&cur_id) {
                    cur_id = self.tt.prev_state_id(cur_id);
                }
            }
        }
        // Just like we did for match states, but we want to move accelerated
        // start states to the beginning of the range instead of the end.
        if cstart > 0 {
            // N.B. special.{min,max}_start do not need updating, since the
            // range/number of start states does not change at this point. Only
            // the ordering of start states may change.
            let mut next_id = self.special.min_start;
            let mut cur_id = next_id;
            while cur_id <= self.special.max_start {
                if let Some(accel) = accels.remove(&cur_id) {
                    remapper.swap(self, cur_id, next_id);
                    accels.insert(next_id, accel);
                    update_special_accel(&mut self.special, next_id);
                    next_id = self.tt.next_state_id(next_id);
                }
                cur_id = self.tt.next_state_id(cur_id);
            }
        }

        // Remap all transitions in our DFA and assert some things.
        remapper.remap(self);
        // This unwrap is OK because acceleration never changes the number of
        // match states or patterns in those match states. Since acceleration
        // runs after the pattern map has been set at least once, we know that
        // our match states cannot error.
        self.set_pattern_map(&new_matches).unwrap();
        self.special.set_max();
        self.special.validate().expect("special state ranges should validate");
        self.special
            .validate_state_len(self.state_len(), self.stride2())
            .expect(
                "special state ranges should be consistent with state length",
            );
        assert_eq!(
            self.special.accel_len(self.stride()),
            // We record the number of accelerated states initially detected
            // since the accels map is itself mutated in the process above.
            // If mutated incorrectly, its size may change, and thus can't be
            // trusted as a source of truth of how many accelerated states we
            // expected there to be.
            original_accels_len,
            "mismatch with expected number of accelerated states",
        );

        // And finally record our accelerators. We kept our accels map updated
        // as we shuffled states above, so the accelerators should now
        // correspond to a contiguous range in the state ID space. (Which we
        // assert.)
        let mut prev: Option<StateID> = None;
        for (id, accel) in accels {
            assert!(prev.map_or(true, |p| self.tt.next_state_id(p) == id));
            prev = Some(id);
            self.accels.add(accel);
        }
    }