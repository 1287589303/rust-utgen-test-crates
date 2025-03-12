pub fn build_from_nfa(
        &self,
        nfa: &thompson::NFA,
    ) -> Result<OwnedDFA, BuildError> {
        let mut quitset = self.config.quitset.unwrap_or(ByteSet::empty());
        if self.config.get_unicode_word_boundary()
            && nfa.look_set_any().contains_word_unicode()
        {
            for b in 0x80..=0xFF {
                quitset.add(b);
            }
        }
        let classes = if !self.config.get_byte_classes() {
            // DFAs will always use the equivalence class map, but enabling
            // this option is useful for debugging. Namely, this will cause all
            // transitions to be defined over their actual bytes instead of an
            // opaque equivalence class identifier. The former is much easier
            // to grok as a human.
            ByteClasses::singletons()
        } else {
            let mut set = nfa.byte_class_set().clone();
            // It is important to distinguish any "quit" bytes from all other
            // bytes. Otherwise, a non-quit byte may end up in the same
            // class as a quit byte, and thus cause the DFA to stop when it
            // shouldn't.
            //
            // Test case:
            //
            //   regex-cli find match dense --unicode-word-boundary \
            //     -p '^#' -p '\b10\.55\.182\.100\b' -y @conn.json.1000x.log
            if !quitset.is_empty() {
                set.add_set(&quitset);
            }
            set.byte_classes()
        };

        let mut dfa = DFA::initial(
            classes,
            nfa.pattern_len(),
            self.config.get_starts(),
            nfa.look_matcher(),
            self.config.get_starts_for_each_pattern(),
            self.config.get_prefilter().map(|p| p.clone()),
            quitset,
            Flags::from_nfa(&nfa),
        )?;
        determinize::Config::new()
            .match_kind(self.config.get_match_kind())
            .quit(quitset)
            .dfa_size_limit(self.config.get_dfa_size_limit())
            .determinize_size_limit(self.config.get_determinize_size_limit())
            .run(nfa, &mut dfa)?;
        if self.config.get_minimize() {
            dfa.minimize();
        }
        if self.config.get_accelerate() {
            dfa.accelerate();
        }
        // The state shuffling done before this point always assumes that start
        // states should be marked as "special," even though it isn't the
        // default configuration. State shuffling is complex enough as it is,
        // so it's simpler to just "fix" our special state ID ranges to not
        // include starting states after-the-fact.
        if !self.config.get_specialize_start_states() {
            dfa.special.set_no_special_start_states();
        }
        // Look for and set the universal starting states.
        dfa.set_universal_starts();
        Ok(dfa)
    }