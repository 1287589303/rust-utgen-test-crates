fn initial(
        classes: ByteClasses,
        pattern_len: usize,
        starts: StartKind,
        lookm: &LookMatcher,
        starts_for_each_pattern: bool,
        pre: Option<Prefilter>,
        quitset: ByteSet,
        flags: Flags,
    ) -> Result<OwnedDFA, BuildError> {
        let start_pattern_len =
            if starts_for_each_pattern { Some(pattern_len) } else { None };
        Ok(DFA {
            tt: TransitionTable::minimal(classes),
            st: StartTable::dead(starts, lookm, start_pattern_len)?,
            ms: MatchStates::empty(pattern_len),
            special: Special::new(),
            accels: Accels::empty(),
            pre,
            quitset,
            flags,
        })
    }