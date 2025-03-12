fn backtrack(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        at: usize,
        start_id: StateID,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {
        cache.stack.push(Frame::Step { sid: start_id, at });
        while let Some(frame) = cache.stack.pop() {
            match frame {
                Frame::Step { sid, at } => {
                    if let Some(hm) = self.step(cache, input, sid, at, slots) {
                        return Some(hm);
                    }
                }
                Frame::RestoreCapture { slot, offset } => {
                    slots[slot] = offset;
                }
            }
        }
        None
    }