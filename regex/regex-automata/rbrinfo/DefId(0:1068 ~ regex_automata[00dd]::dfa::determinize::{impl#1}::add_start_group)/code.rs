fn add_start_group(
        &mut self,
        anchored: Anchored,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), BuildError> {
        let nfa_start = match anchored {
            Anchored::No => self.nfa.start_unanchored(),
            Anchored::Yes => self.nfa.start_anchored(),
            Anchored::Pattern(pid) => {
                self.nfa.start_pattern(pid).expect("valid pattern ID")
            }
        };

        // When compiling start states, we're careful not to build additional
        // states that aren't necessary. For example, if the NFA has no word
        // boundary assertion, then there's no reason to have distinct start
        // states for 'NonWordByte' and 'WordByte' starting configurations.
        // Instead, the 'WordByte' starting configuration can just point
        // directly to the start state for the 'NonWordByte' config.
        //
        // Note though that we only need to care about assertions in the prefix
        // of an NFA since this only concerns the starting states. (Actually,
        // the most precisely thing we could do it is look at the prefix
        // assertions of each pattern when 'anchored == Anchored::Pattern',
        // and then only compile extra states if the prefix is non-empty.) But
        // we settle for simplicity here instead of absolute minimalism. It is
        // somewhat rare, after all, for multiple patterns in the same regex to
        // have different prefix look-arounds.

        let (id, is_new) =
            self.add_one_start(nfa_start, Start::NonWordByte)?;
        self.dfa.set_start_state(anchored, Start::NonWordByte, id);
        if is_new {
            dfa_state_ids.push(id);
        }

        if !self.nfa.look_set_prefix_any().contains_word() {
            self.dfa.set_start_state(anchored, Start::WordByte, id);
        } else {
            let (id, is_new) =
                self.add_one_start(nfa_start, Start::WordByte)?;
            self.dfa.set_start_state(anchored, Start::WordByte, id);
            if is_new {
                dfa_state_ids.push(id);
            }
        }
        if !self.nfa.look_set_prefix_any().contains_anchor() {
            self.dfa.set_start_state(anchored, Start::Text, id);
            self.dfa.set_start_state(anchored, Start::LineLF, id);
            self.dfa.set_start_state(anchored, Start::LineCR, id);
            self.dfa.set_start_state(
                anchored,
                Start::CustomLineTerminator,
                id,
            );
        } else {
            let (id, is_new) = self.add_one_start(nfa_start, Start::Text)?;
            self.dfa.set_start_state(anchored, Start::Text, id);
            if is_new {
                dfa_state_ids.push(id);
            }

            let (id, is_new) = self.add_one_start(nfa_start, Start::LineLF)?;
            self.dfa.set_start_state(anchored, Start::LineLF, id);
            if is_new {
                dfa_state_ids.push(id);
            }

            let (id, is_new) = self.add_one_start(nfa_start, Start::LineCR)?;
            self.dfa.set_start_state(anchored, Start::LineCR, id);
            if is_new {
                dfa_state_ids.push(id);
            }

            let (id, is_new) =
                self.add_one_start(nfa_start, Start::CustomLineTerminator)?;
            self.dfa.set_start_state(
                anchored,
                Start::CustomLineTerminator,
                id,
            );
            if is_new {
                dfa_state_ids.push(id);
            }
        }

        Ok(())
    }