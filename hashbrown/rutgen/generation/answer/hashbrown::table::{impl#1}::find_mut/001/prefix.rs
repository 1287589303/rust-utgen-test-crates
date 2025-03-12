// Answer 0

#[test]
fn test_find_mut_existing_entry() {
    struct TestValue(u64, &'static str);
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &TestValue| val.0; // Identity hash function for simplicity
    table.insert_unique(hasher(&TestValue(1, "a")), TestValue(1, "a"), |val| hasher(val));

    if let Some(val) = table.find_mut(hasher(&TestValue(1, "a")), |val| val.0 == 1) {
        val.1 = "b";
    }
    let result = table.find(hasher(&TestValue(1, "a")), |val| val.0 == 1);
}

#[test]
fn test_find_mut_non_existing_entry() {
    struct TestValue(u64, &'static str);
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &TestValue| val.0;
    table.insert_unique(hasher(&TestValue(1, "a")), TestValue(1, "a"), |val| hasher(val));

    let result = table.find_mut(hasher(&TestValue(2, "b")), |val| val.0 == 2);
}

#[test]
fn test_find_mut_edge_case_empty() {
    struct TestValue(u64, &'static str);
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &TestValue| val.0;

    let result = table.find_mut(hasher(&TestValue(0, "none")), |val| val.0 == 0);
}

#[test]
fn test_find_mut_edge_case_max_hash() {
    struct TestValue(u64, &'static str);
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &TestValue| val.0;
    table.insert_unique(hasher(&TestValue(u64::MAX, "max")), TestValue(u64::MAX, "max"), |val| hasher(val));

    if let Some(val) = table.find_mut(hasher(&TestValue(u64::MAX, "max")), |val| val.0 == u64::MAX) {
        val.1 = "updated";
    }
    let result = table.find(hasher(&TestValue(u64::MAX, "max")), |val| val.0 == u64::MAX);
}

#[test]
fn test_find_mut_with_empty_string() {
    struct TestValue(u64, &'static str);
    let mut table = HashTable::new_in(Global);
    let hasher = |val: &TestValue| val.0;
    table.insert_unique(hasher(&TestValue(1, "")), TestValue(1, ""), |val| hasher(val));

    if let Some(val) = table.find_mut(hasher(&TestValue(1, "")), |val| val.0 == 1) {
        val.1 = "updated";
    }
    let result = table.find(hasher(&TestValue(1, "")), |val| val.0 == 1);
}

