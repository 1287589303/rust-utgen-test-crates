// Answer 0

#[test]
fn test_swap_remove_entry_key_does_not_exist() {
    struct CustomKey;
    struct CustomValue;

    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, _: &mut H) {}
    }

    impl PartialEq for CustomKey {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }

    impl Equivalent<CustomKey> for CustomKey {
        fn equivalent(&self, _: &CustomKey) -> bool {
            false
        }
    }

    let mut index_map: IndexMap<CustomKey, CustomValue, RandomState> = IndexMap::new();
    
    let key_to_remove = CustomKey;
    let result = index_map.swap_remove_entry(&key_to_remove);
}

#[test]
fn test_swap_remove_entry_on_empty_map() {
    struct CustomKey;
    struct CustomValue;

    impl Hash for CustomKey {
        fn hash<H: Hasher>(&self, _: &mut H) {}
    }

    impl PartialEq for CustomKey {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }

    impl Equivalent<CustomKey> for CustomKey {
        fn equivalent(&self, _: &CustomKey) -> bool {
            false
        }
    }

    let mut index_map: IndexMap<CustomKey, CustomValue, RandomState> = IndexMap::new();
    
    let key_to_remove = CustomKey;
    let result = index_map.swap_remove_entry(&key_to_remove);
}

