// Answer 0

#[test]
fn test_get_or_insert_with_failure_due_to_equivalence() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct MyHashBuilder;

    impl BuildHasher for MyHashBuilder {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    #[derive(Hash, Eq, PartialEq)]
    struct MyValue {
        id: i32,
    }

    impl Equivalent<MyValue> for MyValue {
        fn equivalent(&self, other: &MyValue) -> bool {
            self.id == other.id
        }
    }

    let mut set: HashSet<MyValue, MyHashBuilder> = HashSet::new();
    set.insert(MyValue { id: 1 });

    let value = MyValue { id: 2 };
    let f = |val: &MyValue| MyValue { id: 3 }; // Creates a new value that doesn't match

    let _result = set.get_or_insert_with(&value, f);
}

#[test]
#[should_panic]
fn test_get_or_insert_with_panic_on_non_equivalence() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct MyHashBuilder;

    impl BuildHasher for MyHashBuilder {
        fn build_hasher(&self) -> DefaultHasher {
            DefaultHasher::new()
        }
    }

    #[derive(Hash, Eq, PartialEq)]
    struct MyValue {
        id: i32,
    }

    impl Equivalent<MyValue> for MyValue {
        fn equivalent(&self, other: &MyValue) -> bool {
            self.id == other.id
        }
    }

    let mut set: HashSet<MyValue, MyHashBuilder> = HashSet::new();
    set.insert(MyValue { id: 1 });

    let value = MyValue { id: 1 }; // Existing value
    let f = |val: &MyValue| MyValue { id: 2 }; // New value that doesn't match the existing one

    let _result = set.get_or_insert_with(&value, f); // This should panic
}

