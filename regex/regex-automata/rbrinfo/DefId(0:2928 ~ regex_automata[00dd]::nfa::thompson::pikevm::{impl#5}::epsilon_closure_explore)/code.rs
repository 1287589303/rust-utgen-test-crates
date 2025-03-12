fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        input: &Input<'_>,
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
            instrument!(|c| c.record_set_insert(sid));
            // Record this state as part of our next set of active states. If
            // we've already explored it, then no need to do it again.
            if !next.set.insert(sid) {
                return;
            }
            match *self.nfa.state(sid) {
                State::Fail
                | State::Match { .. }
                | State::ByteRange { .. }
                | State::Sparse { .. }
                | State::Dense { .. } => {
                    next.slot_table.for_state(sid).copy_from_slice(curr_slots);
                    return;
                }
                State::Look { look, next } => {
                    // OK because we don't permit building a searcher with a
                    // Unicode word boundary if the requisite Unicode data is
                    // unavailable.
                    if !self.nfa.look_matcher().matches_inline(
                        look,
                        input.haystack(),
                        at,
                    ) {
                        return;
                    }
                    sid = next;
                }
                State::Union { ref alternates } => {
                    sid = match alternates.get(0) {
                        None => return,
                        Some(&sid) => sid,
                    };
                    instrument!(|c| {
                        for &alt in &alternates[1..] {
                            c.record_stack_push(alt);
                        }
                    });
                    stack.extend(
                        alternates[1..]
                            .iter()
                            .copied()
                            .rev()
                            .map(FollowEpsilon::Explore),
                    );
                }
                State::BinaryUnion { alt1, alt2 } => {
                    sid = alt1;
                    instrument!(|c| c.record_stack_push(sid));
                    stack.push(FollowEpsilon::Explore(alt2));
                }
                State::Capture { next, slot, .. } => {
                    // There's no need to do anything with slots that
                    // ultimately won't be copied into the caller-provided
                    // 'Captures' value. So we just skip dealing with them at
                    // all.
                    if slot.as_usize() < curr_slots.len() {
                        instrument!(|c| c.record_stack_push(sid));
                        stack.push(FollowEpsilon::RestoreCapture {
                            slot,
                            offset: curr_slots[slot],
                        });
                        // OK because length of a slice must fit into an isize.
                        curr_slots[slot] = Some(NonMaxUsize::new(at).unwrap());
                    }
                    sid = next;
                }
            }
        }
    }