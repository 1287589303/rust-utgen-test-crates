// Answer 0

#[test]
fn test_insert_occupied_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use core::hash::{Hash, Hasher};

    struct MyKey(String);
    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write(self.0.as_bytes());
        }
    }
    impl From<&str> for MyKey {
        fn from(s: &str) -> MyKey {
            MyKey(s.to_owned())
        }
    }

    let mut map: HashMap<MyKey, u32, DefaultHasher> = HashMap::new();
    map.insert(MyKey::from("key1"), 10);
    
    let entry_ref = map.entry_ref(MyKey::from("key1")).unwrap();
    let result_entry = entry_ref.insert(20);

    // The actual test would go here, but we only focus on the input setup
}

#[test]
fn test_insert_vacant_entry() {
    use hashbrown::HashMap;
    use std::collections::hash_map::DefaultHasher;
    use core::hash::{Hash, Hasher};

    struct MyKey(String);
    impl Hash for MyKey {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write(self.0.as_bytes());
        }
    }
    impl From<&str> for MyKey {
        fn from(s: &str) -> MyKey {
            MyKey(s.to_owned())
        }
    }

    let mut map: HashMap<MyKey, u32, DefaultHasher> = HashMap::new();

    let entry_ref = map.entry_ref(MyKey::from("key2")).unwrap();
    let result_entry = entry_ref.insert(30);
    
    // The actual test would go here, but we only focus on the input setup
}

