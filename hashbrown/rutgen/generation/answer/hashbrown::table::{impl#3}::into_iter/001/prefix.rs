// Answer 0

#[test]
fn test_into_iter_empty_table() {
    struct MyAllocator;
    impl Allocator for MyAllocator {
        // Implement the required allocator methods here.
    }

    let mut table: HashTable<i32, MyAllocator> = HashTable::new_in(MyAllocator);
    let iter = table.into_iter();
}

#[test]
fn test_into_iter_partial_table() {
    struct MyAllocator;
    impl Allocator for MyAllocator {
        // Implement the required allocator methods here.
    }

    let mut table: HashTable<i32, MyAllocator> = HashTable::with_capacity_in(10, MyAllocator);
    table.insert_unique(1, 42, |v| v.clone() as u64);
    let iter = table.into_iter();
}

#[test]
fn test_into_iter_full_table() {
    struct MyAllocator;
    impl Allocator for MyAllocator {
        // Implement the required allocator methods here.
    }

    let mut table: HashTable<i32, MyAllocator> = HashTable::with_capacity_in(10, MyAllocator);
    for i in 0..10 {
        table.insert_unique(i as u64, i as i32, |v| v.clone() as u64);
    }
    let iter = table.into_iter();
}

