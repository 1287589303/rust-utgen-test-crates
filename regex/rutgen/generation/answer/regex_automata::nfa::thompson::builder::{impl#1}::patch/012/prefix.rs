// Answer 0

#[test]
fn test_patch_union_state_no_memory_growth() {
    let mut builder = Builder::new();
    let state_id_1 = builder.add_union(vec![]).unwrap();
    let state_id_2 = builder.add_union(vec![]).unwrap();
    let state_id_3 = builder.add_union(vec![state_id_1]).unwrap();
    builder.patch(state_id_3, state_id_2).unwrap();
}

#[test]
fn test_patch_union_state_no_memory_growth_with_existing_transitions() {
    let mut builder = Builder::new();
    let state_id_1 = builder.add_union(vec![]).unwrap();
    let state_id_2 = builder.add_range(Transition { start: 0, end: 255, next: state_id_1 }).unwrap();
    let state_id_3 = builder.add_union(vec![state_id_2]).unwrap();
    builder.patch(state_id_3, state_id_1).unwrap();
}

#[test]
fn test_patch_union_state_no_memory_growth_multiple_patches() {
    let mut builder = Builder::new();
    let state_id_1 = builder.add_union(vec![]).unwrap();
    let state_id_2 = builder.add_union(vec![]).unwrap();
    let state_id_3 = builder.add_union(vec![state_id_1]).unwrap();
    
    builder.patch(state_id_3, state_id_2).unwrap();
    builder.patch(state_id_3, state_id_1).unwrap();
}

