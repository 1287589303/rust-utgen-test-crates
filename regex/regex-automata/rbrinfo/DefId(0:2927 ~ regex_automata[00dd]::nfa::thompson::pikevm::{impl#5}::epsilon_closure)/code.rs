fn epsilon_closure(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        sid: StateID,
    ) {
        instrument!(|c| {
            c.record_closure(sid);
            c.record_stack_push(sid);
        });
        stack.push(FollowEpsilon::Explore(sid));
        while let Some(frame) = stack.pop() {
            match frame {
                FollowEpsilon::RestoreCapture { slot, offset: pos } => {
                    curr_slots[slot] = pos;
                }
                FollowEpsilon::Explore(sid) => {
                    self.epsilon_closure_explore(
                        stack, curr_slots, next, input, at, sid,
                    );
                }
            }
        }
    }