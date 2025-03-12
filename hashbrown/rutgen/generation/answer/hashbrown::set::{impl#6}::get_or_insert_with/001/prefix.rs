// Answer 0

#[test]
fn test_get_or_insert_with_unique_insertion() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct EquivalentString(String);

    impl Hash for EquivalentString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for EquivalentString {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for EquivalentString {}

    let mut set: HashSet<EquivalentString, DefaultHasher> = HashSet::new();
    let value = EquivalentString("new_value".to_string());

    let new_value_ref = set.get_or_insert_with(&value, |v| {
        let new_value = format!("unique_{}", v.0);
        EquivalentString(new_value)
    });

    drop(set);
    // Using the `new_value_ref` to ensure it holds a reference to a valid element
    let _ = new_value_ref;
}

#[test]
#[should_panic]
fn test_get_or_insert_with_non_equivalent_insertion() {
    use hashbrown::HashSet;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct EquivalentString(String);

    impl Hash for EquivalentString {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }

    impl PartialEq for EquivalentString {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    impl Eq for EquivalentString {}

    let mut set: HashSet<EquivalentString, DefaultHasher> = HashSet::new();
    let value = EquivalentString("rust".to_string());

    set.get_or_insert_with(&value, |_| "".to_string());
}

