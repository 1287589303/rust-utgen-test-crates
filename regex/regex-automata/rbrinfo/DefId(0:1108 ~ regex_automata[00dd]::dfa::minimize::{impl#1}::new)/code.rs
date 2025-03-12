pub fn new(dfa: &'a mut dense::OwnedDFA) -> Minimizer<'a> {
        let in_transitions = Minimizer::incoming_transitions(dfa);
        let partitions = Minimizer::initial_partitions(dfa);
        let waiting = partitions.clone();
        Minimizer { dfa, in_transitions, partitions, waiting }
    }