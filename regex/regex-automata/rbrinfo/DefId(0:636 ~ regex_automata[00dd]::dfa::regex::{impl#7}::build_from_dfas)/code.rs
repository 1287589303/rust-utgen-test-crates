pub fn build_from_dfas<A: Automaton>(
        &self,
        forward: A,
        reverse: A,
    ) -> Regex<A> {
        Regex { forward, reverse }
    }