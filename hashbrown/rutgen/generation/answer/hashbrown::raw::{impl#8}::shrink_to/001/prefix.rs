// Answer 0

#[test]
fn test_shrink_to_with_non_zero_min_size_and_empty_table() {
    let mut table = RawTable::new_in(Global);
    let min_size = 1;
    unsafe {
        table.shrink_to(min_size, |value| value.hash());
    }
}

#[test]
fn test_shrink_to_with_non_zero_min_size_and_filled_table() {
    let mut table = RawTable::with_capacity_in(10, Global);
    let min_size = 5;
    unsafe {
        for i in 0..5 {
            table.insert(i.hash(), i, |v| v.hash());
        }
        table.shrink_to(min_size, |value| value.hash());
    }
}

#[test]
fn test_shrink_to_with_large_min_size() {
    let mut table = RawTable::new_in(Global);
    let min_size = usize::MAX;
    unsafe {
        table.shrink_to(min_size, |value| value.hash());
    }
}

#[test]
fn test_shrink_to_with_min_size_greater_than_current_items() {
    let mut table = RawTable::with_capacity_in(4, Global);
    let items_to_insert = 2;
    let min_size = 3;
    unsafe {
        for i in 0..items_to_insert {
            table.insert(i.hash(), i, |v| v.hash());
        }
        table.shrink_to(min_size, |value| value.hash());
    }
} 

#[test]
fn test_shrink_to_with_min_size_equal_to_current_items() {
    let mut table = RawTable::with_capacity_in(6, Global);
    let items_to_insert = 3;
    let min_size = 3;
    unsafe {
        for i in 0..items_to_insert {
            table.insert(i.hash(), i, |v| v.hash());
        }
        table.shrink_to(min_size, |value| value.hash());
    }
}

