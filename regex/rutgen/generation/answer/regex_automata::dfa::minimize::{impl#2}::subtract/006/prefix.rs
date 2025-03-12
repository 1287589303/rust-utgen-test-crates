// Answer 0

#[test]
fn test_subtract_with_identical_ids() {
    let state_id_1 = StateID(SmallIndex::from(1));
    let state_id_2 = StateID(SmallIndex::from(1)); // Duplicate ID for self
    
    let state_id_3 = StateID(SmallIndex::from(2));
    let state_id_4 = StateID(SmallIndex::from(3));
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id_1);
    self_set.add(state_id_2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id_3);
    other_set.add(state_id_4);
    
    let mut dest_set = StateSet::empty();
    
    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_with_overlapping_and_extra_ids() {
    let state_id_1 = StateID(SmallIndex::from(2));
    let state_id_2 = StateID(SmallIndex::from(2)); // Duplicate ID for self
    
    let state_id_3 = StateID(SmallIndex::from(3));
    let state_id_4 = StateID(SmallIndex::from(4));
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id_1);
    self_set.add(state_id_2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id_3);
    other_set.add(state_id_4);
    
    let mut dest_set = StateSet::empty();
    
    self_set.subtract(&other_set, &mut dest_set);
}

#[test]
fn test_subtract_no_remaining_ids() {
    let state_id_1 = StateID(SmallIndex::from(3));
    let state_id_2 = StateID(SmallIndex::from(3)); // Duplicate ID for self
    
    let state_id_3 = StateID(SmallIndex::from(1));
    let state_id_4 = StateID(SmallIndex::from(2));
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id_1);
    self_set.add(state_id_2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id_3);
    other_set.add(state_id_4);
    
    let mut dest_set = StateSet::empty();
    
    self_set.subtract(&other_set, &mut dest_set);
}

