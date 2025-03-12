// Answer 0

#[test]
fn test_index_set_new() {
    let index_set: super::IndexSet<() ,()> = super::IndexSet::new();
}

#[test]
fn test_index_set_with_capacity() {
    let index_set_with_capacity: super::IndexSet<() ,()> = super::IndexSet::with_capacity(0);
}

#[test]
fn test_index_set_with_non_zero_capacity() {
    let index_set_with_capacity: super::IndexSet<() ,()> = super::IndexSet::with_capacity(10);
}

