// Answer 0

#[test]
fn test_subtract_with_non_empty_sets_a_less_than_b() {
    let mut self_set = StateSet::empty();
    let state_a1 = StateID(1);
    let state_a2 = StateID(2);
    self_set.add(state_a1);

    let mut other_set = StateSet::empty();
    let state_b1 = StateID(3);
    let state_b2 = StateID(4);
    other_set.add(state_b1);
    other_set.add(state_b2);

    let mut dest_set = StateSet::empty();
    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_empty_self_set() {
    let mut self_set = StateSet::empty();
    
    let mut other_set = StateSet::empty();
    let state_b1 = StateID(1);
    let state_b2 = StateID(2);
    other_set.add(state_b1);
    other_set.add(state_b2);

    let mut dest_set = StateSet::empty();
    self_set.subtract(&other_set, &mut dest_set);
}

