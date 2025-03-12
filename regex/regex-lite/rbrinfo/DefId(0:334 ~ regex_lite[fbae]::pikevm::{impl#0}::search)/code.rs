pub(crate) fn search(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        earliest: bool,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {
        cache.setup_search(slots.len());
        if start > end {
            return false;
        }
        // Why do we even care about this? Well, in our `slots` representation,
        // we use usize::MAX as a sentinel to indicate "no match." This isn't
        // problematic so long as our haystack doesn't have a maximal length.
        // Byte slices are guaranteed by Rust to have a length that fits into
        // isize, and so this assert should always pass. But we put it here to
        // make our assumption explicit.
        assert!(
            haystack.len() < core::usize::MAX,
            "byte slice lengths must be less than usize MAX",
        );

        let Cache { ref mut stack, ref mut curr, ref mut next } = cache;
        let start_id = self.nfa().start();
        let anchored = self.nfa().is_start_anchored();
        let mut matched = false;
        // Yes, our search doesn't end at `end`, but includes it. This is
        // necessary because matches are delayed by one byte. The delay is used
        // to handle look-behind assertions. In the case of the PikeVM, the
        // delay is implemented by not considering a match to exist until it
        // is visited in `nexts`. Technically, we know a match exists in the
        // previous iteration via `epsilon_closure`.
        let mut at = start;
        while at <= end {
            // If we have no states left to visit, then there are some cases
            // where we know we can quit early or even skip ahead.
            if curr.set.is_empty() {
                // We have a match so we can quit.
                if matched {
                    break;
                }
                // If we're running an anchored search and we've advanced
                // beyond the start position with no other states to try, then
                // we will never observe a match and thus can stop.
                if anchored && at > start {
                    break;
                }
            }
            // Instead of using a hypothetical unanchored start state in the
            // NFA (which doesn't exist, but we could add it), we actually
            // always use its anchored starting state. As a result, when doing
            // an unanchored search, we need to simulate our own '(?s:.)*?'
            // prefix, to permit a match to appear anywhere.
            //
            // Now, we don't *have* to do things this way. We could create
            // a proper unanchored start state in the NFA and do one
            // `epsilon_closure` call from that starting state before the main
            // loop here. And that is just as correct. However, it turns out to
            // be slower than our approach here because it slightly increases
            // the cost of processing each byte by requiring us to visit
            // more NFA states to deal with the additional NFA states in the
            // unanchored prefix. By simulating it explicitly here, we lower
            // those costs substantially. The cost is itself small, but it adds
            // up for large haystacks.
            //
            // In order to simulate the '(?s:.)*?' prefix---which is not
            // greedy---we are careful not to perform an epsilon closure on
            // the start state if we already have a match. Namely, if we
            // did otherwise, we would never reach a terminating condition
            // because there would always be additional states to process.
            if !matched {
                // Since we are adding to the 'curr' active states and since
                // this is for the start ID, we use a slots slice that is
                // guaranteed to have the right length but where every element
                // is absent. This is exactly what we want, because this
                // epsilon closure is responsible for simulating an unanchored
                // '(?s:.)*?' prefix. It is specifically outside of any
                // capturing groups, and thus, using slots that are always
                // absent is correct.
                //
                // Note though that we can't just use `&mut []` here, since
                // this epsilon closure may traverse through `Capture` states
                // transitions, and thus must be able to write offsets to the
                // slots given which are later copied to slot values in `curr`.
                let slots = next.slot_table.all_absent();
                self.epsilon_closure(
                    stack, slots, curr, haystack, at, start_id,
                );
            }
            let (ch, len) = utf8::decode_lossy(&haystack[at..]);
            if self.nexts(stack, curr, next, haystack, at, ch, len, slots) {
                matched = true;
            }
            // Unless the caller asked us to return early, we need to mush
            // on to see if we can extend our match. (But note that 'nexts'
            // will quit right after seeing a match, as is consistent with
            // leftmost-first match priority.)
            if (earliest && matched) || len == 0 {
                break;
            }
            core::mem::swap(curr, next);
            next.set.clear();
            at += len;
        }
        matched
    }