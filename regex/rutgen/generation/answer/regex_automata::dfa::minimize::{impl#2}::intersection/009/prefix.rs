// Answer 0

#[test]
fn test_intersection_non_empty_disjoint_sets() {
    let mut set_a = StateSet::empty();
    let mut set_b = StateSet::empty();
    let mut dest_set = StateSet::empty();

    // Adding unique StateID values to set_a
    set_a.add(StateID(1));
    set_a.add(StateID(2));
    set_a.add(StateID(3));

    // Adding unique but distinct StateID values to set_b
    set_b.add(StateID(4));
    set_b.add(StateID(5));
    set_b.add(StateID(6));

    set_a.intersection(&set_b, &mut dest_set);
}

#[test]
fn test_intersection_with_last_element_disjoint() {
    let mut set_a = StateSet::empty();
    let mut set_b = StateSet::empty();
    let mut dest_set = StateSet::empty();

    // Adding unique StateID values to set_a
    set_a.add(StateID(1));
    set_a.add(StateID(3)); // Note the gap to ensure a < b condition
    set_a.add(StateID(5));

    // Adding other unique values to set_b, ensuring they are not in set_a
    set_b.add(StateID(2));
    set_b.add(StateID(4));
    set_b.add(StateID(6));

    set_a.intersection(&set_b, &mut dest_set);
}

#[test]
fn test_intersection_with_empty_dest_set() {
    let mut set_a = StateSet::empty();
    let mut set_b = StateSet::empty();
    let mut dest_set = StateSet::empty();

    // Adding unique StateID values to set_a
    set_a.add(StateID(1));
    set_a.add(StateID(3));
    set_a.add(StateID(5));

    // Adding unique but distinct values to set_b, ensuring:
    // a < b to satisfy precondition for test
    set_b.add(StateID(0));
    set_b.add(StateID(2));
    set_b.add(StateID(4));

    set_a.intersection(&set_b, &mut dest_set);
}

