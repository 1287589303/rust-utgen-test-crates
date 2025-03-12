fn next(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slot_table: &mut SlotTable,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        sid: StateID,
    ) -> bool {
        match *self.nfa.state(sid) {
            State::Fail
            | State::Goto { .. }
            | State::Splits { .. }
            | State::Capture { .. } => false,
            State::Char { target, ch } => {
                if at_ch == ch && at_len > 0 {
                    let slots = curr_slot_table.for_state(sid);
                    // OK because `at_len` is always derived from the number
                    // of bytes read from `at` that make up `at_ch`. So this
                    // will never wrap.
                    let at = at.wrapping_add(at_len);
                    self.epsilon_closure(
                        stack, slots, next, haystack, at, target,
                    );
                }
                false
            }
            State::Ranges { target, ref ranges } => {
                for (start, end) in ranges.iter().copied() {
                    if start > at_ch {
                        break;
                    } else if start <= at_ch && at_ch <= end {
                        if at_len == 0 {
                            return false;
                        }
                        let slots = curr_slot_table.for_state(sid);
                        // OK because `at_len` is always derived from the
                        // number of bytes read from `at` that make up `at_ch`.
                        // So this will never wrap.
                        let at = at.wrapping_add(at_len);
                        self.epsilon_closure(
                            stack, slots, next, haystack, at, target,
                        );
                    }
                }
                false
            }
            State::Match => true,
        }
    }