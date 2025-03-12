// Answer 0

#[test]
fn test_intersection_self_empty_other_empty() {
    let self_set = StateSet::empty();
    let other_set = StateSet::empty();
    let mut dest_set = StateSet::empty();
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_self_empty_other_single() {
    let self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    other_set.add(StateID(1));
    let mut dest_set = StateSet::empty();
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_self_empty_other_multiple() {
    let self_set = StateSet::empty();
    let mut other_set = StateSet::empty();
    other_set.add(StateID(1));
    other_set.add(StateID(2));
    other_set.add(StateID(3));
    let mut dest_set = StateSet::empty();
    self_set.intersection(&other_set, &mut dest_set);
}

