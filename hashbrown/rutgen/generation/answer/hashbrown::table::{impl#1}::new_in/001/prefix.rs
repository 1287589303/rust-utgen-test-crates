// Answer 0

#[test]
fn test_hash_table_new_in_with_valid_allocator() {
    use bumpalo::Bump;

    let bump = Bump::new();
    let table: HashTable<i32, _> = HashTable::new_in(&bump);
}

#[test]
fn test_hash_table_new_in_with_default_allocator() {
    use std::alloc::Global;

    let global_allocator = Global;
    let table: HashTable<i32, _> = HashTable::new_in(global_allocator);
}

