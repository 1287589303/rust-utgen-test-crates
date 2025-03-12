// Answer 0

#[test]
fn test_intersection_with_matching_state_id_and_extra_in_other() {
    let state_id1 = StateID(1);
    let state_id2 = StateID(2);
    let state_id3 = StateID(3);
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id1);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id1);
    other_set.add(state_id2);
    other_set.add(state_id3);
    
    let mut dest_set = StateSet::empty();
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_with_multiple_matching_state_ids_and_extra_in_other() {
    let state_id1 = StateID(1);
    let state_id2 = StateID(2);
    let state_id3 = StateID(3);
    let state_id4 = StateID(4);
    
    let mut self_set = StateSet::empty();
    self_set.add(state_id1);
    self_set.add(state_id2);
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id1);
    other_set.add(state_id2);
    other_set.add(state_id3);
    other_set.add(state_id4);
    
    let mut dest_set = StateSet::empty();
    self_set.intersection(&other_set, &mut dest_set);
}

#[test]
fn test_intersection_with_empty_self_and_non_empty_other() {
    let state_id1 = StateID(1);
    let state_id2 = StateID(2);
    
    let mut self_set = StateSet::empty();
    
    let mut other_set = StateSet::empty();
    other_set.add(state_id1);
    other_set.add(state_id2);
    
    let mut dest_set = StateSet::empty();
    self_set.intersection(&other_set, &mut dest_set);
}

