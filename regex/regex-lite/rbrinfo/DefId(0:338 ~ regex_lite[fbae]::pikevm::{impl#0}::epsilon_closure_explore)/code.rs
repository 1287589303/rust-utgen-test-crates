fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        mut sid: StateID,
    ) {
        // We can avoid pushing some state IDs on to our stack in precisely
        // the cases where a 'push(x)' would be immediately followed by a 'x
        // = pop()'. This is achieved by this outer-loop. We simply set 'sid'
        // to be the next state ID we want to explore once we're done with
        // our initial exploration. In practice, this avoids a lot of stack
        // thrashing.
        loop {
            // Record this state as part of our next set of active states. If
            // we've already explored it, then no need to do it again.
            if !next.set.insert(sid) {
                return;
            }
            match *self.nfa.state(sid) {
                State::Fail
                | State::Match { .. }
                | State::Char { .. }
                | State::Ranges { .. } => {
                    next.slot_table.for_state(sid).copy_from_slice(curr_slots);
                    return;
                }
                State::Goto { target, look: None } => {
                    sid = target;
                }
                State::Goto { target, look: Some(look) } => {
                    if !look.is_match(haystack, at) {
                        return;
                    }
                    sid = target;
                }
                State::Splits { ref targets, reverse: false } => {
                    sid = match targets.get(0) {
                        None => return,
                        Some(&sid) => sid,
                    };
                    stack.extend(
                        targets[1..]
                            .iter()
                            .copied()
                            .rev()
                            .map(FollowEpsilon::Explore),
                    );
                }
                State::Splits { ref targets, reverse: true } => {
                    sid = match targets.last() {
                        None => return,
                        Some(&sid) => sid,
                    };
                    stack.extend(
                        targets[..targets.len() - 1]
                            .iter()
                            .copied()
                            .map(FollowEpsilon::Explore),
                    );
                }
                State::Capture { target, slot } => {
                    // There's no need to do anything with slots that
                    // ultimately won't be copied into the caller-provided
                    // 'Captures' value. So we just skip dealing with them at
                    // all.
                    if slot.as_usize() < curr_slots.len() {
                        stack.push(FollowEpsilon::RestoreCapture {
                            slot,
                            offset: curr_slots[slot.as_usize()],
                        });
                        // OK because length of a slice must fit into an isize.
                        curr_slots[slot.as_usize()] =
                            Some(NonMaxUsize::new(at).unwrap());
                    }
                    sid = target;
                }
            }
        }
    }