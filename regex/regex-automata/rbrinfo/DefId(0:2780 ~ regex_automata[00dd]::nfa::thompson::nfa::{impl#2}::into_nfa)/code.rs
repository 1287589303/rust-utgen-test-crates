pub(super) fn into_nfa(mut self) -> NFA {
        self.byte_classes = self.byte_class_set.byte_classes();
        // Do epsilon closure from the start state of every pattern in order
        // to compute various properties such as look-around assertions and
        // whether the empty string can be matched.
        let mut stack = vec![];
        let mut seen = SparseSet::new(self.states.len());
        for &start_id in self.start_pattern.iter() {
            stack.push(start_id);
            seen.clear();
            // let mut prefix_all = LookSet::full();
            let mut prefix_any = LookSet::empty();
            while let Some(sid) = stack.pop() {
                if !seen.insert(sid) {
                    continue;
                }
                match self.states[sid] {
                    State::ByteRange { .. }
                    | State::Dense { .. }
                    | State::Fail => continue,
                    State::Sparse(_) => {
                        // This snippet below will rewrite this sparse state
                        // as a dense state. By doing it here, we apply this
                        // optimization to all hot "sparse" states since these
                        // are the states that are reachable from the start
                        // state via an epsilon closure.
                        //
                        // Unfortunately, this optimization did not seem to
                        // help much in some very limited ad hoc benchmarking.
                        //
                        // I left the 'Dense' state type in place in case we
                        // want to revisit this, but I suspect the real way
                        // to make forward progress is a more fundamental
                        // rearchitecting of how data in the NFA is laid out.
                        // I think we should consider a single contiguous
                        // allocation instead of all this indirection and
                        // potential heap allocations for every state. But this
                        // is a large re-design and will require API breaking
                        // changes.
                        // self.memory_extra -= self.states[sid].memory_usage();
                        // let trans = DenseTransitions::from_sparse(sparse);
                        // self.states[sid] = State::Dense(trans);
                        // self.memory_extra += self.states[sid].memory_usage();
                        continue;
                    }
                    State::Match { .. } => self.has_empty = true,
                    State::Look { look, next } => {
                        prefix_any = prefix_any.insert(look);
                        stack.push(next);
                    }
                    State::Union { ref alternates } => {
                        // Order doesn't matter here, since we're just dealing
                        // with look-around sets. But if we do richer analysis
                        // here that needs to care about preference order, then
                        // this should be done in reverse.
                        stack.extend(alternates.iter());
                    }
                    State::BinaryUnion { alt1, alt2 } => {
                        stack.push(alt2);
                        stack.push(alt1);
                    }
                    State::Capture { next, .. } => {
                        stack.push(next);
                    }
                }
            }
            self.look_set_prefix_any =
                self.look_set_prefix_any.union(prefix_any);
        }
        NFA(Arc::new(self))
    }