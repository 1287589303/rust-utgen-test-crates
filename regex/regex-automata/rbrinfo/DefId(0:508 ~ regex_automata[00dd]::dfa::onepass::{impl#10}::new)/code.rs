fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition {
        let match_wins =
            if match_wins { 1 << Transition::MATCH_WINS_SHIFT } else { 0 };
        let sid = sid.as_u64() << Transition::STATE_ID_SHIFT;
        Transition(sid | match_wins | epsilons.0)
    }