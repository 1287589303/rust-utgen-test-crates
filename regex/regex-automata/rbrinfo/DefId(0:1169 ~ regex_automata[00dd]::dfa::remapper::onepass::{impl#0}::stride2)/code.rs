fn stride2(&self) -> usize {
            // We don't do pre-multiplication for the one-pass DFA, so
            // returning 0 has the effect of making state IDs and state indices
            // equivalent.
            0
        }