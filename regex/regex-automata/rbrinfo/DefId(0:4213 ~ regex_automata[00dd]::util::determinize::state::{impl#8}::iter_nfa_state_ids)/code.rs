fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {
        let mut sids = &self.0[self.pattern_offset_end()..];
        let mut prev = 0i32;
        while !sids.is_empty() {
            let (delta, nr) = read_vari32(sids);
            sids = &sids[nr..];
            let sid = prev + delta;
            prev = sid;
            // This is OK since we only ever serialize valid StateIDs to
            // states. And since state IDs can never exceed an isize, they must
            // always be able to fit into a usize, and thus cast is OK.
            f(StateID::new_unchecked(sid.as_usize()))
        }
    }