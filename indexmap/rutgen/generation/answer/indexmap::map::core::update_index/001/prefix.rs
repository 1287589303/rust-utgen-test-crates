// Answer 0

#[test]
fn test_update_index_valid_case() {
    let mut table: Indices = hash_table::HashTable::new();
    table.insert(0, 1);
    table.insert(1, 2);
    let hash = HashValue(0);
    update_index(&mut table, hash, 1, 0);
}

#[test]
fn test_update_index_boundary_case() {
    let mut table: Indices = hash_table::HashTable::new();
    table.insert(0, 3);
    let hash = HashValue(0);
    update_index(&mut table, hash, 0, 1);
}

#[test]
fn test_update_index_replacement_case() {
    let mut table: Indices = hash_table::HashTable::new();
    table.insert(0, 5);
    table.insert(1, 10);
    let hash = HashValue(0);
    update_index(&mut table, hash, 0, 1);
}

#[should_panic(expected = "index not found")]
#[test]
fn test_update_index_old_not_found() {
    let mut table: Indices = hash_table::HashTable::new();
    table.insert(0, 0);
    table.insert(1, 1);
    let hash = HashValue(0);
    update_index(&mut table, hash, 10, 1);
}

#[should_panic(expected = "index not found")]
#[test]
fn test_update_index_empty_table() {
    let mut table: Indices = hash_table::HashTable::new();
    let hash = HashValue(0);
    update_index(&mut table, hash, 0, 1);
}

