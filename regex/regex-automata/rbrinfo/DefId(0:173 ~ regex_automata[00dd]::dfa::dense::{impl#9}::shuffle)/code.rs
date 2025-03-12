pub(crate) fn shuffle(
        &mut self,
        mut matches: BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), BuildError> {
        // The determinizer always adds a quit state and it is always second.
        self.special.quit_id = self.to_state_id(1);
        // If all we have are the dead and quit states, then we're done and
        // the DFA will never produce a match.
        if self.state_len() <= 2 {
            self.special.set_max();
            return Ok(());
        }

        // Collect all our non-DEAD start states into a convenient set and
        // confirm there is no overlap with match states. In the classicl DFA
        // construction, start states can be match states. But because of
        // look-around, we delay all matches by a byte, which prevents start
        // states from being match states.
        let mut is_start: BTreeSet<StateID> = BTreeSet::new();
        for (start_id, _, _) in self.starts() {
            // If a starting configuration points to a DEAD state, then we
            // don't want to shuffle it. The DEAD state is always the first
            // state with ID=0. So we can just leave it be.
            if start_id == DEAD {
                continue;
            }
            assert!(
                !matches.contains_key(&start_id),
                "{:?} is both a start and a match state, which is not allowed",
                start_id,
            );
            is_start.insert(start_id);
        }

        // We implement shuffling by a sequence of pairwise swaps of states.
        // Since we have a number of things referencing states via their
        // IDs and swapping them changes their IDs, we need to record every
        // swap we make so that we can remap IDs. The remapper handles this
        // book-keeping for us.
        let mut remapper = Remapper::new(self);

        // Shuffle matching states.
        if matches.is_empty() {
            self.special.min_match = DEAD;
            self.special.max_match = DEAD;
        } else {
            // The determinizer guarantees that the first two states are the
            // dead and quit states, respectively. We want our match states to
            // come right after quit.
            let mut next_id = self.to_state_id(2);
            let mut new_matches = BTreeMap::new();
            self.special.min_match = next_id;
            for (id, pids) in matches {
                remapper.swap(self, next_id, id);
                new_matches.insert(next_id, pids);
                // If we swapped a start state, then update our set.
                if is_start.contains(&next_id) {
                    is_start.remove(&next_id);
                    is_start.insert(id);
                }
                next_id = self.tt.next_state_id(next_id);
            }
            matches = new_matches;
            self.special.max_match = cmp::max(
                self.special.min_match,
                self.tt.prev_state_id(next_id),
            );
        }

        // Shuffle starting states.
        {
            let mut next_id = self.to_state_id(2);
            if self.special.matches() {
                next_id = self.tt.next_state_id(self.special.max_match);
            }
            self.special.min_start = next_id;
            for id in is_start {
                remapper.swap(self, next_id, id);
                next_id = self.tt.next_state_id(next_id);
            }
            self.special.max_start = cmp::max(
                self.special.min_start,
                self.tt.prev_state_id(next_id),
            );
        }

        // Finally remap all transitions in our DFA.
        remapper.remap(self);
        self.set_pattern_map(&matches)?;
        self.special.set_max();
        self.special.validate().expect("special state ranges should validate");
        self.special
            .validate_state_len(self.state_len(), self.stride2())
            .expect(
                "special state ranges should be consistent with state length",
            );
        Ok(())
    }