pub fn matches_byte(&self, byte: u8) -> Option<StateID> {
        for t in self.transitions.iter() {
            if t.start > byte {
                break;
            } else if t.matches_byte(byte) {
                return Some(t.next);
            }
        }
        None

        /*
        // This is an alternative implementation that uses binary search. In
        // some ad hoc experiments, like
        //
        //   regex-cli find match pikevm -b -p '\b\w+\b' non-ascii-file
        //
        // I could not observe any improvement, and in fact, things seemed to
        // be a bit slower. I can see an improvement in at least one benchmark:
        //
        //   regex-cli find match pikevm -b -p '\pL{100}' all-codepoints-utf8
        //
        // Where total search time goes from 3.2s to 2.4s when using binary
        // search.
        self.transitions
            .binary_search_by(|t| {
                if t.end < byte {
                    core::cmp::Ordering::Less
                } else if t.start > byte {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Equal
                }
            })
            .ok()
            .map(|i| self.transitions[i].next)
        */
    }