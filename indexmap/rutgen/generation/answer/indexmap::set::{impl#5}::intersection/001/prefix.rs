// Answer 0

#[test]
fn test_intersection_with_integers() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: IndexSet<i32, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };
    let mut set_b: IndexSet<i32, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };

    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);
    
    set_b.insert(2);
    set_b.insert(3);
    set_b.insert(4);

    let intersection = set_a.intersection(&set_b);
}

#[test]
fn test_intersection_with_strings() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: IndexSet<String, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };
    let mut set_b: IndexSet<String, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };

    set_a.insert("apple".to_string());
    set_a.insert("banana".to_string());
    set_a.insert("cherry".to_string());

    set_b.insert("banana".to_string());
    set_b.insert("cherry".to_string());
    set_b.insert("date".to_string());

    let intersection = set_a.intersection(&set_b);
}

#[test]
fn test_intersection_with_floats() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: IndexSet<f64, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };
    let mut set_b: IndexSet<f64, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };

    set_a.insert(1.1);
    set_a.insert(2.2);
    set_a.insert(3.3);

    set_b.insert(2.2);
    set_b.insert(3.3);
    set_b.insert(4.4);

    let intersection = set_a.intersection(&set_b);
}

#[test]
fn test_intersection_with_empty_and_non_empty() {
    struct HashBuilder;
    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set_a: IndexSet<i32, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };
    let mut set_b: IndexSet<i32, HashBuilder> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: HashBuilder } };

    set_b.insert(10);
    set_b.insert(20);

    let intersection = set_a.intersection(&set_b);
}

