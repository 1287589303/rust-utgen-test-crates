// Answer 0

#[test]
fn test_fmt_empty_sparse_set() {
    let sparse_set = SparseSet::new(0);
    let mut formatter = core::fmt::Formatter::new();
    sparse_set.fmt(&mut formatter);
}

#[test]
fn test_fmt_single_element_sparse_set() {
    let mut sparse_set = SparseSet::new(1);
    let id = StateID::new(0);
    sparse_set.insert(id);
    let mut formatter = core::fmt::Formatter::new();
    sparse_set.fmt(&mut formatter);
}

#[test]
fn test_fmt_multiple_elements_sparse_set() {
    let max_state_id = StateID::max_value();
    let mut sparse_set = SparseSet::new(max_state_id.into());
    for i in 0..=max_state_id {
        sparse_set.insert(StateID::new(i));
    }
    let mut formatter = core::fmt::Formatter::new();
    sparse_set.fmt(&mut formatter);
} 

#[test]
fn test_fmt_non_empty_sparse_set() {
    let mut sparse_set = SparseSet::new(1);
    let id1 = StateID::new(0);
    let id2 = StateID::new(1);
    sparse_set.insert(id1);
    sparse_set.insert(id2);
    let mut formatter = core::fmt::Formatter::new();
    sparse_set.fmt(&mut formatter);
}

#[test]
fn test_fmt_empty_set_after_clear() {
    let mut sparse_set = SparseSet::new(3);
    sparse_set.insert(StateID::new(0));
    sparse_set.clear();
    let mut formatter = core::fmt::Formatter::new();
    sparse_set.fmt(&mut formatter);
}

