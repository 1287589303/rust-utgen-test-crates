// Answer 0

#[test]
fn test_contains_with_non_empty_set() {
    let mut sparse_set = SparseSet::new(10);
    let state_id_1 = StateID(1);
    let state_id_2 = StateID(2);
    sparse_set.insert(state_id_1);
    sparse_set.insert(state_id_2);

    let contains_1 = sparse_set.contains(state_id_1);
    let contains_2 = sparse_set.contains(state_id_2);
    let contains_3 = sparse_set.contains(StateID(3));
}

#[test]
fn test_contains_with_empty_set() {
    let sparse_set = SparseSet::new(10);
    let contains_1 = sparse_set.contains(StateID(1));
}

#[test]
fn test_contains_with_single_element_set() {
    let mut sparse_set = SparseSet::new(10);
    let state_id = StateID(1);
    sparse_set.insert(state_id);

    let contains = sparse_set.contains(state_id);
    let not_contains = sparse_set.contains(StateID(2));
}

#[test]
fn test_contains_with_capacity_limit() {
    let mut sparse_set = SparseSet::new(5);
    for i in 0..5 {
        sparse_set.insert(StateID(i));
    }

    let contains_last = sparse_set.contains(StateID(4));
    let not_contains_out_of_bounds = sparse_set.contains(StateID(5));
}

#[test]
fn test_contains_with_len_zero() {
    let sparse_set = SparseSet::new(0);
    let contains = sparse_set.contains(StateID(0));
}

#[test]
fn test_contains_with_exact_len() {
    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(StateID(0));
    sparse_set.insert(StateID(1));

    let contains_first = sparse_set.contains(StateID(0));
    let contains_second = sparse_set.contains(StateID(1));
    let not_contains_third = sparse_set.contains(StateID(2));
}

