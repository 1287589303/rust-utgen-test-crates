// Answer 0

#[test]
fn test_intersection_equal() {
    let mut set_a = StateSet::empty();
    let mut set_b = StateSet::empty();
    let mut result_set = StateSet::empty();

    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));

    set_a.add(state_id_1);
    set_a.add(state_id_2);

    set_b.add(state_id_2);
    set_b.add(state_id_3);

    set_a.intersection(&set_b, &mut result_set);
}

#[test]
fn test_intersection_multiple_common() {
    let mut set_a = StateSet::empty();
    let mut set_b = StateSet::empty();
    let mut result_set = StateSet::empty();

    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));
    let state_id_4 = StateID(SmallIndex(4));

    set_a.add(state_id_1);
    set_a.add(state_id_2);
    set_a.add(state_id_3);

    set_b.add(state_id_2);
    set_b.add(state_id_3);
    set_b.add(state_id_4);

    set_a.intersection(&set_b, &mut result_set);
}

#[test]
fn test_intersection_identical_sets() {
    let mut set_a = StateSet::empty();
    let mut set_b = StateSet::empty();
    let mut result_set = StateSet::empty();

    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));

    set_a.add(state_id_1);
    set_a.add(state_id_2);

    set_b.add(state_id_1);
    set_b.add(state_id_2);

    set_a.intersection(&set_b, &mut result_set);
}

