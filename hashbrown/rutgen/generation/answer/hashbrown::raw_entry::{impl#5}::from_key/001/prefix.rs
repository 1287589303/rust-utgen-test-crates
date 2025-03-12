// Answer 0

#[test]
fn test_from_key_existing_entry() {
    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "a";
    let _ = map.raw_entry().from_key(&key);
}

#[test]
fn test_from_key_another_existing_entry() {
    let map: HashMap<&str, u32> = [("a", 100), ("b", 200), ("c", 300)].into();
    let key = "b";
    let _ = map.raw_entry().from_key(&key);
}

#[test]
fn test_from_key_non_existent_entry() {
    let map: HashMap<&str, u32> = [("a", 100), ("b", 200)].into();
    let key = "nonexistent";
    let _ = map.raw_entry().from_key(&key);
}

#[test]
fn test_from_key_empty_string() {
    let map: HashMap<&str, u32> = [("", 0), ("key", 1)].into();
    let key = "";
    let _ = map.raw_entry().from_key(&key);
}

#[test]
fn test_from_key_long_string() {
    let map: HashMap<&str, u32> = [("long_key_string", 42)].into();
    let key = "long_key_string";
    let _ = map.raw_entry().from_key(&key);
}

#[test]
fn test_from_key_equivalent_type() {
    struct MyKey(String);
    impl Hash for MyKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            state.write(self.0.as_bytes());
        }
    }
    impl Equivalent<&str> for MyKey {
        fn equivalent(&self, other: &str) -> bool {
            self.0 == other
        }
    }

    let map: HashMap<MyKey, u32> = [(MyKey("a".into()), 100)].into();
    let key = MyKey("a".into());
    let _ = map.raw_entry().from_key(&key);
}

