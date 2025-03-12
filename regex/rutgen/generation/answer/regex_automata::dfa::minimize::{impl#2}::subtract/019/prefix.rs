// Answer 0

#[test]
fn test_subtract_non_empty_sets_common_elements() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    self_set.add(StateID(1));
    self_set.add(StateID(2));
    self_set.add(StateID(3));

    other_set.add(StateID(2));
    other_set.add(StateID(4));

    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_non_empty_sets_distinct_elements() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    self_set.add(StateID(5));
    self_set.add(StateID(6));

    other_set.add(StateID(1));
    other_set.add(StateID(2));

    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_equal_corner_case() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    self_set.add(StateID(10));
    self_set.add(StateID(20));

    other_set.add(StateID(20));
    other_set.add(StateID(30));

    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_left_exclusive() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    self_set.add(StateID(7));
    self_set.add(StateID(8));
    self_set.add(StateID(9));

    other_set.add(StateID(5));
    other_set.add(StateID(6));
    other_set.add(StateID(10));

    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_only_common_elements() {
    let mut self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    self_set.add(StateID(1));
    self_set.add(StateID(1)); // Adding the same element for common case

    other_set.add(StateID(1));

    self_set.subtract(&other_set, &mut dest_set);
}

