// Answer 0

#[test]
fn test_shift_insert_new_value_within_bounds() {
    let mut set: IndexSet<char, RandomState> = IndexSet::new();
    assert_eq!(set.shift_insert(0, '*'), true);
    assert_eq!(set.shift_insert(1, '+'), true);
    assert_eq!(set.shift_insert(2, '-'), true);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_shift_insert_existing_value_at_valid_index() {
    let mut set: IndexSet<char, RandomState> = IndexSet::new();
    set.shift_insert(0, '*');
    set.shift_insert(1, '+');
    set.shift_insert(2, '-');
    
    assert_eq!(set.shift_insert(1, '*'), false);
    assert_eq!(set.shift_insert(2, '+'), false);
    assert_eq!(set.len(), 3);
}

#[test]
fn test_shift_insert_new_value_at_end() {
    let mut set: IndexSet<char, RandomState> = IndexSet::new();
    set.shift_insert(0, '*');
    set.shift_insert(1, '+');
    
    assert_eq!(set.shift_insert(2, '-'), true);
    assert_eq!(set.len(), 3);
}

#[test]
#[should_panic]
fn test_shift_insert_existing_value_out_of_bounds() {
    let mut set: IndexSet<char, RandomState> = IndexSet::new();
    set.shift_insert(0, '*');
    
    // This is an invalid index for moving an existing value!
    set.shift_insert(set.len(), '*');
}

#[test]
fn test_shift_insert_existing_value_at_boundary() {
    let mut set: IndexSet<char, RandomState> = IndexSet::new();
    set.shift_insert(0, '*');
    set.shift_insert(1, '+');
    
    assert_eq!(set.shift_insert(1, '*'), false); // Moving '*' to index 1
    assert_eq!(set.shift_insert(2, '+'), false); // Moving '+' to index 2
    assert_eq!(set.len(), 2);
}

