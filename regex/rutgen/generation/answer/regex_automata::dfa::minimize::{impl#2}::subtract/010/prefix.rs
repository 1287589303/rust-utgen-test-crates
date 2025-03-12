// Answer 0

#[test]
fn test_subtract_with_stable_elements() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let state_id_3 = StateID(SmallIndex(3));
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id_1);
    self_set.add(state_id_2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id_1);
    other_set.add(state_id_3);
    
    let mut dest_set = StateSet::empty();
    
    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_identical_top_elements() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    let mut self_set = StateSet::empty();
    self_set.add(state_id_1);
    self_set.add(state_id_2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id_1);
    other_set.add(state_id_2);

    let mut dest_set = StateSet::empty();
    
    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_multiple_identical_elements() {
    let state_id_1 = StateID(SmallIndex(1));
    let state_id_2 = StateID(SmallIndex(2));
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id_1);
    self_set.add(state_id_2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id_1);
    other_set.add(state_id_1); // Multiple entries of same element

    let mut dest_set = StateSet::empty();
    
    self_set.subtract(&other_set, &mut dest_set);
}

