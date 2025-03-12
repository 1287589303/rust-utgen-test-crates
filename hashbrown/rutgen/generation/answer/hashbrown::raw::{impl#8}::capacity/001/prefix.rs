// Answer 0

#[test]
fn test_capacity_with_zero_items_and_zero_growth_left() {
    let raw_table = RawTable::<u8>::new_in(Global);
    raw_table.table.items = 0;
    raw_table.table.growth_left = 0;
    let _ = raw_table.capacity();
}

#[test]
fn test_capacity_with_zero_items_and_max_growth_left() {
    let raw_table = RawTable::<u8>::new_in(Global);
    raw_table.table.items = 0;
    raw_table.table.growth_left = usize::MAX;
    let _ = raw_table.capacity();
}

#[test]
fn test_capacity_with_max_items_and_zero_growth_left() {
    let raw_table = RawTable::<u8>::new_in(Global);
    raw_table.table.items = usize::MAX;
    raw_table.table.growth_left = 0;
    let _ = raw_table.capacity();
}

#[test]
fn test_capacity_with_max_items_and_max_growth_left() {
    let raw_table = RawTable::<u8>::new_in(Global);
    raw_table.table.items = usize::MAX;
    raw_table.table.growth_left = usize::MAX - 1; // must not exceed usize::MAX
    let _ = raw_table.capacity();
}

#[test]
fn test_capacity_with_various_items_and_zero_growth_left() {
    for items in [0, 1, 2, 3, 4, 5, 10, 100, 1000, 10_000, usize::MAX - 1].iter() {
        let raw_table = RawTable::<u8>::new_in(Global);
        raw_table.table.items = *items;
        raw_table.table.growth_left = 0;
        let _ = raw_table.capacity();
    }
}

#[test]
fn test_capacity_with_various_growth_left_and_zero_items() {
    for growth_left in [0, 1, 2, 3, 4, 5, 10, 100, 1000, 10_000, usize::MAX - 1].iter() {
        let raw_table = RawTable::<u8>::new_in(Global);
        raw_table.table.items = 0;
        raw_table.table.growth_left = *growth_left;
        let _ = raw_table.capacity();
    }
}

#[test]
fn test_capacity_with_various_items_and_growth_left() {
    for items in [0, 1, 2, 3, 4, 5, 10, 100, 1000, 10_000, usize::MAX - 1].iter() {
        for growth_left in [0, 1, 2, 3, 4, 5, 10, 100, 1000, 10_000, usize::MAX - 1].iter() {
            if items + growth_left <= &usize::MAX {
                let raw_table = RawTable::<u8>::new_in(Global);
                raw_table.table.items = *items;
                raw_table.table.growth_left = *growth_left;
                let _ = raw_table.capacity();
            }
        }
    }
}

