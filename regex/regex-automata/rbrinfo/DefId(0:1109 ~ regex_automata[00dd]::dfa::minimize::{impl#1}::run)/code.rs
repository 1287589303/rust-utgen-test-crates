pub fn run(mut self) {
        let stride2 = self.dfa.stride2();
        let as_state_id = |index: usize| -> StateID {
            StateID::new(index << stride2).unwrap()
        };
        let as_index = |id: StateID| -> usize { id.as_usize() >> stride2 };

        let mut incoming = StateSet::empty();
        let mut scratch1 = StateSet::empty();
        let mut scratch2 = StateSet::empty();
        let mut newparts = vec![];

        // This loop is basically Hopcroft's algorithm. Everything else is just
        // shuffling data around to fit our representation.
        while let Some(set) = self.waiting.pop() {
            for b in self.dfa.byte_classes().iter() {
                self.find_incoming_to(b, &set, &mut incoming);
                // If incoming is empty, then the intersection with any other
                // set must also be empty. So 'newparts' just ends up being
                // 'self.partitions'. So there's no need to go through the loop
                // below.
                //
                // This actually turns out to be rather large optimization. On
                // the order of making minimization 4-5x faster. It's likely
                // that the vast majority of all states have very few incoming
                // transitions.
                if incoming.is_empty() {
                    continue;
                }

                for p in 0..self.partitions.len() {
                    self.partitions[p].intersection(&incoming, &mut scratch1);
                    if scratch1.is_empty() {
                        newparts.push(self.partitions[p].clone());
                        continue;
                    }

                    self.partitions[p].subtract(&incoming, &mut scratch2);
                    if scratch2.is_empty() {
                        newparts.push(self.partitions[p].clone());
                        continue;
                    }

                    let (x, y) =
                        (scratch1.deep_clone(), scratch2.deep_clone());
                    newparts.push(x.clone());
                    newparts.push(y.clone());
                    match self.find_waiting(&self.partitions[p]) {
                        Some(i) => {
                            self.waiting[i] = x;
                            self.waiting.push(y);
                        }
                        None => {
                            if x.len() <= y.len() {
                                self.waiting.push(x);
                            } else {
                                self.waiting.push(y);
                            }
                        }
                    }
                }
                newparts = mem::replace(&mut self.partitions, newparts);
                newparts.clear();
            }
        }

        // At this point, we now have a minimal partitioning of states, where
        // each partition is an equivalence class of DFA states. Now we need to
        // use this partitioning to update the DFA to only contain one state for
        // each partition.

        // Create a map from DFA state ID to the representative ID of the
        // equivalence class to which it belongs. The representative ID of an
        // equivalence class of states is the minimum ID in that class.
        let mut state_to_part = vec![DEAD; self.dfa.state_len()];
        for p in &self.partitions {
            p.iter(|id| state_to_part[as_index(id)] = p.min());
        }

        // Generate a new contiguous sequence of IDs for minimal states, and
        // create a map from equivalence IDs to the new IDs. Thus, the new
        // minimal ID of *any* state in the unminimized DFA can be obtained
        // with minimals_ids[state_to_part[old_id]].
        let mut minimal_ids = vec![DEAD; self.dfa.state_len()];
        let mut new_index = 0;
        for state in self.dfa.states() {
            if state_to_part[as_index(state.id())] == state.id() {
                minimal_ids[as_index(state.id())] = as_state_id(new_index);
                new_index += 1;
            }
        }
        // The total number of states in the minimal DFA.
        let minimal_count = new_index;
        // Convenience function for remapping state IDs. This takes an old ID,
        // looks up its Hopcroft partition and then maps that to the new ID
        // range.
        let remap = |old| minimal_ids[as_index(state_to_part[as_index(old)])];

        // Re-map this DFA in place such that the only states remaining
        // correspond to the representative states of every equivalence class.
        for id in (0..self.dfa.state_len()).map(as_state_id) {
            // If this state isn't a representative for an equivalence class,
            // then we skip it since it won't appear in the minimal DFA.
            if state_to_part[as_index(id)] != id {
                continue;
            }
            self.dfa.remap_state(id, remap);
            self.dfa.swap_states(id, minimal_ids[as_index(id)]);
        }
        // Trim off all unused states from the pre-minimized DFA. This
        // represents all states that were merged into a non-singleton
        // equivalence class of states, and appeared after the first state
        // in each such class. (Because the state with the smallest ID in each
        // equivalence class is its representative ID.)
        self.dfa.truncate_states(minimal_count);

        // Update the new start states, which is now just the minimal ID of
        // whatever state the old start state was collapsed into. Also, we
        // collect everything before-hand to work around the borrow checker.
        // We're already allocating so much that this is probably fine. If this
        // turns out to be costly, then I guess add a `starts_mut` iterator.
        let starts: Vec<_> = self.dfa.starts().collect();
        for (old_start_id, anchored, start_type) in starts {
            self.dfa.set_start_state(
                anchored,
                start_type,
                remap(old_start_id),
            );
        }

        // Update the match state pattern ID list for multi-regexes. All we
        // need to do is remap the match state IDs. The pattern ID lists are
        // always the same as they were since match states with distinct
        // pattern ID lists are always considered distinct states.
        let mut pmap = BTreeMap::new();
        for (match_id, pattern_ids) in self.dfa.pattern_map() {
            let new_id = remap(match_id);
            pmap.insert(new_id, pattern_ids);
        }
        // This unwrap is OK because minimization never increases the number of
        // match states or patterns in those match states. Since minimization
        // runs after the pattern map has already been set at least once, we
        // know that our match states cannot error.
        self.dfa.set_pattern_map(&pmap).unwrap();

        // In order to update the ID of the maximum match state, we need to
        // find the maximum ID among all of the match states in the minimized
        // DFA. This is not necessarily the new ID of the unminimized maximum
        // match state, since that could have been collapsed with a much
        // earlier match state. Therefore, to find the new max match state,
        // we iterate over all previous match states, find their corresponding
        // new minimal ID, and take the maximum of those.
        let old = self.dfa.special().clone();
        let new = self.dfa.special_mut();
        // ... but only remap if we had match states.
        if old.matches() {
            new.min_match = StateID::MAX;
            new.max_match = StateID::ZERO;
            for i in as_index(old.min_match)..=as_index(old.max_match) {
                let new_id = remap(as_state_id(i));
                if new_id < new.min_match {
                    new.min_match = new_id;
                }
                if new_id > new.max_match {
                    new.max_match = new_id;
                }
            }
        }
        // ... same, but for start states.
        if old.starts() {
            new.min_start = StateID::MAX;
            new.max_start = StateID::ZERO;
            for i in as_index(old.min_start)..=as_index(old.max_start) {
                let new_id = remap(as_state_id(i));
                if new_id == DEAD {
                    continue;
                }
                if new_id < new.min_start {
                    new.min_start = new_id;
                }
                if new_id > new.max_start {
                    new.max_start = new_id;
                }
            }
            if new.max_start == DEAD {
                new.min_start = DEAD;
            }
        }
        new.quit_id = remap(new.quit_id);
        new.set_max();
    }