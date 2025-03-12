// Answer 0

#[test]
fn test_intersection_self_non_empty_other_empty() {
    let mut self_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    for i in 1..=100 {
        self_set.add(StateID(i));
    }

    let other_set = StateSet::empty();

    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_self_non_empty_other_empty_boundary_case() {
    let mut self_set = StateSet::empty();
    let mut dest_set = StateSet::empty();
    
    self_set.add(StateID(1));

    let other_set = StateSet::empty();

    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_self_single_element_other_empty() {
    let mut self_set = StateSet::empty();
    let mut dest_set = StateSet::empty();

    self_set.add(StateID(42));

    let other_set = StateSet::empty();

    self_set.intersection(&other_set, &mut dest_set);
}

