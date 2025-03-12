// Answer 0

#[test]
fn test_patch_empty_to_union() {
    let mut builder = Builder::new();
    let state_id_empty = builder.add(State::Empty { next: StateID(SmallIndex::default()) }).unwrap();
    let state_id_union = builder.add(State::Union { alternates: vec![] }).unwrap();

    builder.patch(state_id_empty, state_id_union).unwrap();
}

#[test]
fn test_patch_empty_to_union_reverse() {
    let mut builder = Builder::new();
    let state_id_empty = builder.add(State::Empty { next: StateID(SmallIndex::default()) }).unwrap();
    let state_id_union_reverse = builder.add(State::UnionReverse { alternates: vec![] }).unwrap();

    builder.patch(state_id_empty, state_id_union_reverse).unwrap();
}

#[test]
fn test_patch_with_memory_increase() {
    let mut builder = Builder::new();
    builder.set_size_limit(Some(1024)).unwrap();  // Set a memory limit

    let state_id_empty = builder.add(State::Empty { next: StateID(SmallIndex::default()) }).unwrap();
    let state_id_union = builder.add(State::Union { alternates: vec![] }).unwrap();

    builder.patch(state_id_empty, state_id_union).unwrap();
}

#[test]
#[should_panic]
fn test_patch_spars_state() {
    let mut builder = Builder::new();
    let state_id_sparse = builder.add(State::Sparse { transitions: vec![] }).unwrap();
    let state_id_empty = builder.add(State::Empty { next: StateID(SmallIndex::default()) }).unwrap();

    builder.patch(state_id_sparse, state_id_empty).unwrap(); // This should panic
}

