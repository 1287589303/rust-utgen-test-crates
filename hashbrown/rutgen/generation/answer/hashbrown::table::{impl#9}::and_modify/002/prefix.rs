// Answer 0

#[test]
fn test_and_modify_existing_entry() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"example"), ("example", 10), hasher);

    table
        .entry(
            hasher(&"example"),
            |&(x, _)| x == "example",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 5);
}

#[test]
fn test_and_modify_multiple_entries() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"item1"), ("item1", 20), hasher);
    table.insert_unique(hasher(&"item2"), ("item2", 30), hasher);

    table
        .entry(
            hasher(&"item1"),
            |&(x, _)| x == "item1",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 10);

    table
        .entry(
            hasher(&"item2"),
            |&(x, _)| x == "item2",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 15);
}

#[test]
fn test_and_modify_with_custom_value() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"custom"), ("custom", 1), hasher);

    table
        .entry(
            hasher(&"custom"),
            |&(x, _)| x == "custom",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v *= 2);
}

#[test]
fn test_and_modify_with_zero() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"zero"), ("zero", 0), hasher);

    table
        .entry(
            hasher(&"zero"),
            |&(x, _)| x == "zero",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v += 100);
}

#[test]
fn test_and_modify_creates_new_value_if_entry_already_modified() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    use std::hash::BuildHasher;

    let mut table: HashTable<(&str, u32)> = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher = |val: &_| hasher.hash_one(val);

    table.insert_unique(hasher(&"dynamic"), ("dynamic", 5), hasher);

    table
        .entry(
            hasher(&"dynamic"),
            |&(x, _)| x == "dynamic",
            |(k, _)| hasher(&k),
        )
        .and_modify(|(_, v)| *v -= 3)
        .or_insert(("dynamic", 10));
}

