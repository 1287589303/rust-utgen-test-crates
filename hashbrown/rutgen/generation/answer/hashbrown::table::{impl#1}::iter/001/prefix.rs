// Answer 0

#[test]
fn test_empty_table_iter() {
    let table: HashTable<i32> = HashTable::new_in(Global);
    let iter = table.iter();
}

#[test]
fn test_single_element_iter() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;
    table.insert_unique(hasher(&1), 1, hasher);
    let iter = table.iter();
}

#[test]
fn test_multiple_elements_iter() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;

    for i in 0..10 {
        table.insert_unique(hasher(&i), i, hasher);
    }

    let iter = table.iter();
}

#[test]
fn test_large_number_of_elements_iter() {
    let mut table: HashTable<i32> = HashTable::new_in(Global);
    let hasher = |val: &i32| *val as u64;

    for i in 0..100 {
        table.insert_unique(hasher(&i), i, hasher);
    }

    let iter = table.iter();
}

