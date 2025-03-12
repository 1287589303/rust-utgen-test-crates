// Answer 0

#[test]
fn test_subtract_non_overlapping() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    // Populate self_set with StateIDs that are greater than those in other_set
    self_set.add(StateID(2));
    self_set.add(StateID(3));
    self_set.add(StateID(4));

    // Populate other_set with lower StateIDs
    other_set.add(StateID(0));
    other_set.add(StateID(1));

    // Call the subtract function
    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_exhaustion() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    // Populate self_set with unique StateIDs
    self_set.add(StateID(5));
    self_set.add(StateID(6));
    self_set.add(StateID(7));

    // Populate other_set with lower overlapping StateIDs
    other_set.add(StateID(1));
    other_set.add(StateID(2));
    other_set.add(StateID(3));

    // Call the subtract function
    self_set.subtract(&other_set, &mut dest_set);
}

