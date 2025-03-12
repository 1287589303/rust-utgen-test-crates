// Answer 0

#[test]
fn test_index_set_default() {
    let index_set: IndexSet<(), ()> = IndexSet::default();
}

#[test]
fn test_index_set_default_empty() {
    let index_set: IndexSet<(), ()> = IndexSet::default();
    let map = index_set.map; // This should be an empty IndexMap
}

