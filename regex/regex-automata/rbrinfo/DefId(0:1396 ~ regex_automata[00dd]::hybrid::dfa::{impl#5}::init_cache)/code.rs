fn init_cache(&mut self) {
        // Why multiply by 2 here? Because we make room for both the unanchored
        // and anchored start states. Unanchored is first and then anchored.
        let mut starts_len = Start::len().checked_mul(2).unwrap();
        // ... but if we also want start states for every pattern, we make room
        // for that too.
        if self.dfa.get_config().get_starts_for_each_pattern() {
            starts_len += Start::len() * self.dfa.pattern_len();
        }
        self.cache
            .starts
            .extend(iter::repeat(self.as_ref().unknown_id()).take(starts_len));
        // This is the set of NFA states that corresponds to each of our three
        // sentinel states: the empty set.
        let dead = State::dead();
        // This sets up some states that we use as sentinels that are present
        // in every DFA. While it would be technically possible to implement
        // this DFA without explicitly putting these states in the transition
        // table, this is convenient to do to make `next_state` correct for all
        // valid state IDs without needing explicit conditionals to special
        // case these sentinel states.
        //
        // All three of these states are "dead" states. That is, all of
        // them transition only to themselves. So once you enter one of
        // these states, it's impossible to leave them. Thus, any correct
        // search routine must explicitly check for these state types. (Sans
        // `unknown`, since that is only used internally to represent missing
        // states.)
        let unk_id =
            self.add_state(dead.clone(), |id| id.to_unknown()).unwrap();
        let dead_id = self.add_state(dead.clone(), |id| id.to_dead()).unwrap();
        let quit_id = self.add_state(dead.clone(), |id| id.to_quit()).unwrap();
        assert_eq!(unk_id, self.as_ref().unknown_id());
        assert_eq!(dead_id, self.as_ref().dead_id());
        assert_eq!(quit_id, self.as_ref().quit_id());
        // The idea here is that if you start in an unknown/dead/quit state and
        // try to transition on them, then you should end up where you started.
        self.set_all_transitions(unk_id, unk_id);
        self.set_all_transitions(dead_id, dead_id);
        self.set_all_transitions(quit_id, quit_id);
        // All of these states are technically equivalent from the FSM
        // perspective, so putting all three of them in the cache isn't
        // possible. (They are distinct merely because we use their
        // identifiers as sentinels to mean something, as indicated by the
        // names.) Moreover, we wouldn't want to do that. Unknown and quit
        // states are special in that they are artificial constructions
        // this implementation. But dead states are a natural part of
        // determinization. When you reach a point in the NFA where you cannot
        // go anywhere else, a dead state will naturally arise and we MUST
        // reuse the canonical dead state that we've created here. Why? Because
        // it is the state ID that tells the search routine whether a state is
        // dead or not, and thus, whether to stop the search. Having a bunch of
        // distinct dead states would be quite wasteful!
        self.cache.states_to_id.insert(dead, dead_id);
    }