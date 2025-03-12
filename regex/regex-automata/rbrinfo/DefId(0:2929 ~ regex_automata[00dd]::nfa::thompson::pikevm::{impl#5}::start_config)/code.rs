fn start_config(&self, input: &Input<'_>) -> Option<(bool, StateID)> {
        match input.get_anchored() {
            // Only way we're unanchored is if both the caller asked for an
            // unanchored search *and* the pattern is itself not anchored.
            Anchored::No => Some((
                self.nfa.is_always_start_anchored(),
                self.nfa.start_anchored(),
            )),
            Anchored::Yes => Some((true, self.nfa.start_anchored())),
            Anchored::Pattern(pid) => {
                Some((true, self.nfa.start_pattern(pid)?))
            }
        }
    }