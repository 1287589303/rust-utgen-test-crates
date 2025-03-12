// Answer 0

#[test]
fn test_new_with_multiple_matches() {
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    use crate::dfa::{MatchStates, PatternID, StateID, BuildError};

    let mut matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
    let sid1 = StateID(0.into());
    let sid2 = StateID(1.into());

    matches.insert(sid1, vec![PatternID(0.into()), PatternID(1.into())]);
    matches.insert(sid2, vec![PatternID(2.into()), PatternID(3.into())]);
    
    let pattern_len = 4;

    let result = MatchStates::new(&matches, pattern_len);
}

#[test]
fn test_new_with_single_empty_match() {
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    use crate::dfa::{MatchStates, PatternID, StateID, BuildError};

    let matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new(); // No entries
    let pattern_len = 0;

    let result = MatchStates::new(&matches, pattern_len);
}

#[test]
fn test_new_with_single_match_exceeding_capacity() {
    use alloc::collections::BTreeMap;
    use alloc::vec::Vec;
    use crate::dfa::{MatchStates, PatternID, StateID, BuildError};

    let mut matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
    let sid = StateID(0.into());

    // Inserting more patterns than the allowable limit
    matches.insert(sid, vec![PatternID(0.into()), PatternID(1.into()), PatternID(2.into()), 
                              PatternID(3.into()), PatternID(4.into()), PatternID(5.into()), 
                              PatternID(6.into()), PatternID(7.into()), PatternID(8.into()), 
                              PatternID(9.into()), PatternID(10.into())]); // Exceeding the limit

    let pattern_len = 11;

    let result = MatchStates::new(&matches, pattern_len);
}

