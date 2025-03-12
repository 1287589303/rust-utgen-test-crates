// Answer 0

#[test]
fn test_or_insert_with_key_occupied_entry() {
    use hashbrown::HashMap;
    use std::borrow::Borrow;

    struct CustomHasher; // Placeholder struct for a custom hasher
    use std::hash::{Hasher, BuildHasher};

    impl Hasher for CustomHasher {
        fn write(&mut self, _bytes: &[u8]) {}
        fn finish(&self) -> u64 { 0 }
    }

    struct CustomBuildHasher;

    impl BuildHasher for CustomBuildHasher {
        type Hasher = CustomHasher;
        fn build_hasher(&self) -> Self::Hasher { CustomHasher }
    }

    let mut map: HashMap<String, usize, CustomBuildHasher> = HashMap::new();
    map.insert("occupied".to_owned(), 42); // Existing key

    match map.entry_ref("occupied") {
        EntryRef::Occupied(mut entry) => {
            let result = entry.or_insert_with_key(|key| key.len());
            // Further actions can be performed with `result`, which is `&mut usize`
            *result += 1; // Modify the value at that entry
        },
        EntryRef::Vacant(_) => panic!("Entry should be occupied"),
    }
}

#[test]
fn test_or_insert_with_key_vacant_entry() {
    use hashbrown::HashMap;
    use std::borrow::Borrow;

    struct CustomHasher; // Placeholder struct for a custom hasher
    use std::hash::{Hasher, BuildHasher};

    impl Hasher for CustomHasher {
        fn write(&mut self, _bytes: &[u8]) {}
        fn finish(&self) -> u64 { 0 }
    }

    struct CustomBuildHasher;

    impl BuildHasher for CustomBuildHasher {
        type Hasher = CustomHasher;
        fn build_hasher(&self) -> Self::Hasher { CustomHasher }
    }

    let mut map: HashMap<String, usize, CustomBuildHasher> = HashMap::new();
    
    match map.entry_ref("vacant") {
        EntryRef::Occupied(_) => panic!("Entry should be vacant"),
        EntryRef::Vacant(entry) => {
            let result = entry.or_insert_with_key(|key| key.chars().count());
            // `result` is now a mutable reference to the inserted value
            *result += 5; // Modify the value at that entry
        },
    }
}

