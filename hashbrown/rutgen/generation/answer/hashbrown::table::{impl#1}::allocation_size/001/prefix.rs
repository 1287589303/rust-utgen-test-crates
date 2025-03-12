// Answer 0

#[test]
fn test_allocation_size_empty() {
    let table: HashTable<i32> = HashTable::new_in(Global);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_single_element() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    table.insert(1, 42, |x| *x);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_max_capacity() {
    let max_capacity: usize = 1024;
    let mut table: HashTable<i32> = HashTable::with_capacity_in(max_capacity, Global);
    for i in 0..max_capacity {
        table.insert(i as u64, i as i32, |x| *x);
    }
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_after_resizing() {
    let initial_capacity: usize = 256;
    let mut table: HashTable<i32> = HashTable::with_capacity_in(initial_capacity, Global);
    for i in 0..initial_capacity {
        table.insert(i as u64, i as i32, |x| *x);
    }
    table.shrink_to_fit(|x| *x);
    let size = table.allocation_size();
}

#[test]
fn test_allocation_size_different_allocator() {
    struct CustomAllocator;
    impl Allocator for CustomAllocator { /* Implementation here */ }
    
    let table: HashTable<i32, CustomAllocator> = HashTable::new_in(CustomAllocator);
    let size = table.allocation_size();
}

