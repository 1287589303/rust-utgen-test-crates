// Answer 0

#[test]
fn test_or_default_with_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct HasherBuilder;

    impl BuildHasher for HasherBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    map.insert("existing_key".to_string(), Some(10));

    {
        let entry_ref = map.entry_ref("existing_key");
        let value_ref = entry_ref.or_default();
        // Here, value_ref should be a mutable reference to Some(10)
        // We don’t assert, only calling the function as per request
    }
}

#[test]
fn test_or_default_with_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::BuildHasher;

    struct HasherBuilder;

    impl BuildHasher for HasherBuilder {
        type Hasher = DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            DefaultHasher::new()
        }
    }

    let mut map: HashMap<String, Option<u32>> = HashMap::new();

    {
        let entry_ref = map.entry_ref("new_key");
        let value_ref = entry_ref.or_default();
        // Here, value_ref should be a mutable reference to None (after inserting)
        // We don’t assert, only calling the function as per request
    }
}

