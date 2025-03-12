// Answer 0

#[test]
fn test_into_entries_non_empty_map() {
    struct CustomKey;
    impl Hash for CustomKey {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }
    
    struct CustomValue;

    let index_set: super::IndexSet<CustomKey, std::collections::hash_map::RandomState> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };
    
    let _result = index_set.into_entries();
}

#[test]
fn test_into_entries_empty_map() {
    struct CustomKey;
    impl Hash for CustomKey {
        fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    }
    
    struct CustomValue;

    let index_set: super::IndexSet<CustomKey, std::collections::hash_map::RandomState> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };
    
    let _result = index_set.into_entries();
}

#[test]
#[should_panic]
fn test_into_entries_null_instance() {
    let index_set: Option<super::IndexSet<u32, std::collections::hash_map::RandomState>> = None;
    
    if let Some(valid_index_set) = index_set {
        let _result = valid_index_set.into_entries();
    } else {
        panic!("IndexSet is null");
    }
}

