fn next_back(&mut self) -> Option<PatternID> {
        while let Some((index, &yes)) = self.it.next_back() {
            if yes {
                // Only valid 'PatternID' values can be inserted into the set
                // and construction of the set panics if the capacity would
                // permit storing invalid pattern IDs. Thus, 'yes' is only true
                // precisely when 'index' corresponds to a valid 'PatternID'.
                return Some(PatternID::new_unchecked(index));
            }
        }
        None
    }