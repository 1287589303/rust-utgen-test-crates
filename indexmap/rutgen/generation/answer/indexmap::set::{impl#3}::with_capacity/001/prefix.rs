// Answer 0

#[test]
fn test_index_set_with_capacity_zero() {
    let index_set = IndexSet::<u32, RandomState>::with_capacity(0);
}

#[test]
fn test_index_set_with_capacity_one() {
    let index_set = IndexSet::<u32, RandomState>::with_capacity(1);
}

#[test]
fn test_index_set_with_capacity_small() {
    let index_set = IndexSet::<u32, RandomState>::with_capacity(10);
}

#[test]
fn test_index_set_with_capacity_large() {
    let index_set = IndexSet::<u32, RandomState>::with_capacity(1000);
}

#[test]
fn test_index_set_with_capacity_max() {
    let index_set = IndexSet::<u32, RandomState>::with_capacity(usize::MAX);
}

