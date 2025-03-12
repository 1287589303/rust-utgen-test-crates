// Answer 0

#[test]
fn test_shrink_to_fit_empty_table() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val as u64;
    table.shrink_to_fit(hasher);
}

#[test]
fn test_shrink_to_fit_one_element() {
    let mut table = HashTable::with_capacity_in(10, Global);
    let hasher = |val: &_| val as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    table.shrink_to_fit(hasher);
}

#[test]
fn test_shrink_to_fit_multiple_elements() {
    let mut table = HashTable::with_capacity_in(20, Global);
    let hasher = |val: &_| val as u64;
    for i in 1..=5 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.shrink_to_fit(hasher);
}

#[test]
fn test_shrink_to_fit_exceeding_capacity() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val as u64;
    for i in 1..=50 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.shrink_to_fit(hasher);
}

#[test]
fn test_shrink_to_fit_capacities() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val as u64;
    for i in 1..=80 {
        table.insert_unique(hasher(&i), i, hasher);
    }
    table.shrink_to_fit(hasher);
}

