pub fn build_from_dfas(&self, forward: DFA, reverse: DFA) -> Regex {
        Regex { forward, reverse }
    }