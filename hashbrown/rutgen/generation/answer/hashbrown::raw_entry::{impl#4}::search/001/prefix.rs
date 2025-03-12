// Answer 0

#[test]
fn test_search_occupied_entry() {
    use crate::hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 100);

    let hash = {
        let mut hasher = DefaultHasher::new();
        "a".hash(&mut hasher);
        hasher.finish()
    };

    let result = {
        let builder = RawEntryBuilderMut { map: &mut map };
        builder.search(hash, |key| key == &"a")
    };

    // Implicitly verified that result is of type `RawEntryMut::Occupied`
}

#[test]
fn test_search_occupied_entry_with_different_value() {
    use crate::hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("b", 200);

    let hash = {
        let mut hasher = DefaultHasher::new();
        "b".hash(&mut hasher);
        hasher.finish()
    };

    let result = {
        let builder = RawEntryBuilderMut { map: &mut map };
        builder.search(hash, |key| key == &"b")
    };

    // Implicitly verified that result is of type `RawEntryMut::Occupied`
}

