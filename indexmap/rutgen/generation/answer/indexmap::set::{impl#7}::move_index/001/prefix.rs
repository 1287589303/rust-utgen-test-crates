// Answer 0

#[test]
fn test_move_index_valid_range_upward() {
    let mut index_set: IndexSet<usize, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.move_index(1, 2);
}

#[test]
fn test_move_index_valid_range_downward() {
    let mut index_set: IndexSet<usize, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.move_index(2, 1);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_from() {
    let mut index_set: IndexSet<usize, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.move_index(3, 1);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_to() {
    let mut index_set: IndexSet<usize, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.move_index(1, 3);
}

#[test]
fn test_move_index_to_same_position() {
    let mut index_set: IndexSet<usize, RandomState> = IndexSet::new();
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);
    index_set.move_index(1, 1);
}

