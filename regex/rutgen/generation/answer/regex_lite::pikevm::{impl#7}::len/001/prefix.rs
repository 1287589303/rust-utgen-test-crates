// Answer 0

#[test]
fn test_len_zero_capacity() {
    let set = SparseSet::new(0);
    let length = set.len();
}

#[test]
fn test_len_non_empty_set() {
    let mut set = SparseSet::new(5);
    set.insert(StateID::from(1));
    set.insert(StateID::from(2));
    let length = set.len();
}

#[test]
fn test_len_after_inserts() {
    let mut set = SparseSet::new(10);
    for id in 0..5 {
        let _ = set.insert(StateID::from(id));
    }
    let length = set.len();
}

#[test]
fn test_len_after_clear() {
    let mut set = SparseSet::new(5);
    set.insert(StateID::from(1));
    set.insert(StateID::from(2));
    set.clear();
    let length = set.len();
}

#[test]
fn test_len_after_resizing() {
    let mut set = SparseSet::new(3);
    set.insert(StateID::from(0));
    set.insert(StateID::from(1));
    set.resize(10);
    let length = set.len(); 
}

