// Answer 0

#[test]
fn test_reserve_zero_items() {
    let alloc = Global;
    let mut table: RawTable<i32, Global> = RawTable::with_capacity_in(0, alloc);
    let additional = 0;
    let hasher = |value: &i32| *value as u64;
    table.reserve(additional, hasher);
}

#[test]
fn test_reserve_exact_growth_left() {
    let alloc = Global;
    let mut table: RawTable<i32, Global> = RawTable::with_capacity_in(10, alloc);
    table.table.growth_left = 10;
    let additional = table.table.growth_left;
    let hasher = |value: &i32| *value as u64;
    table.reserve(additional, hasher);
}

#[test]
fn test_reserve_less_than_growth_left() {
    let alloc = Global;
    let mut table: RawTable<i32, Global> = RawTable::with_capacity_in(10, alloc);
    table.table.growth_left = 10;
    let additional = 5;
    let hasher = |value: &i32| *value as u64;
    table.reserve(additional, hasher);
}

