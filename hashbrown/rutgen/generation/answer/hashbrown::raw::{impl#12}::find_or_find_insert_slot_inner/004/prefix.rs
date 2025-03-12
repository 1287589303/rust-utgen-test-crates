// Answer 0

#[test]
fn test_find_or_find_insert_slot_inner_case_1() {
    struct TestAllocator;
    
    let allocator = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 8; // Should be a power of two

    unsafe {
        let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);

        // Fill the buckets with FULL elements (dummy hash values)
        for i in 0..capacity {
            let hash = (i as u64) * 2; // Use even numbers as hashes
            raw_table.set_ctrl(i, Tag::full(hash));
        }

        // Ensure there's at least one empty bucket
        raw_table.set_ctrl(0, Tag::EMPTY);

        let eq = |index: usize| index == capacity; // No matching index should be found
        let result = raw_table.find_or_find_insert_slot_inner(0, &mut eq);

        // The function should return an Err with an InsertSlot
        drop(result);
    }
}

#[test]
fn test_find_or_find_insert_slot_inner_case_2() {
    struct TestAllocator;
    
    let allocator = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 4; // Power of two

    unsafe {
        let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);

        // Fill the buckets with FULL elements
        for i in 0..capacity {
            let hash = (i as u64) * 3; // Use multipliers
            raw_table.set_ctrl(i, Tag::full(hash));
        }
        
        // Ensure that at least one deletion occurs to create an empty bucket
        raw_table.set_ctrl(0, Tag::DELETED);

        let eq = |index: usize| index == 42; // Use an index that doesn't exist
        let result = raw_table.find_or_find_insert_slot_inner(42, &mut eq);

        drop(result);
    }
}

#[test]
fn test_find_or_find_insert_slot_inner_case_3() {
    struct TestAllocator;
    
    let allocator = TestAllocator;
    let table_layout = TableLayout::default();
    let capacity = 16; // Higher power of two

    unsafe {
        let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);

        // Fill buckets with FULL elements
        for i in 0..capacity {
            let hash = (i as u64) + 1; // Use sequential hashes
            raw_table.set_ctrl(i, Tag::full(hash));
        }
        
        // Ensure that at least one bucket is empty
        raw_table.set_ctrl(capacity - 1, Tag::EMPTY);

        let eq = |index: usize| index == 100; // No match should be found
        let result = raw_table.find_or_find_insert_slot_inner(100, &mut eq);

        drop(result);
    }
}

