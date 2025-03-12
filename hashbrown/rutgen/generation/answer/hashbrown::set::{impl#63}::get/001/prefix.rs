// Answer 0

#[test]
fn test_get_existing_string_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<&str> = HashSet::new();
    set.insert("poneyland");

    match set.entry("poneyland") {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
    }
}

#[test]
fn test_get_existing_integer_entry() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(42);

    match set.entry(42) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
    }
}

#[test]
fn test_get_existing_custom_struct_entry() {
    use hashbrown::hash_set::{Entry, HashSet};
    use std::hash::{Hash, Hasher};

    #[derive(Hash, Eq, PartialEq)]
    struct CustomStruct {
        id: i32,
    }

    let mut set: HashSet<CustomStruct> = HashSet::new();
    let custom_obj = CustomStruct { id: 1 };
    set.insert(custom_obj.clone());

    match set.entry(custom_obj) {
        Entry::Vacant(_) => panic!(),
        Entry::Occupied(entry) => {
            let value = entry.get();
        }
    }
}

#[test]
fn test_get_existing_entry_max_capacity() {
    use hashbrown::hash_set::{Entry, HashSet};

    let mut set: HashSet<i32> = HashSet::new();
    for i in 0..capacity {
        set.insert(i);
    }

    for i in 0..capacity {
        match set.entry(i) {
            Entry::Vacant(_) => panic!(),
            Entry::Occupied(entry) => {
                let value = entry.get();
            }
        }
    }
}

