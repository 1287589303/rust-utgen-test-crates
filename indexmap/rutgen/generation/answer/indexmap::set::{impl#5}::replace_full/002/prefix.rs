// Answer 0

#[test]
fn test_replace_full_no_replacement() {
    struct HashBuilder;

    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_set: super::IndexSet<u32, HashBuilder> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                indices: vec![],
                entries: vec![],
            },
            hash_builder: HashBuilder,
        },
    };

    // Insert values that will not be equal to the value we will pass to replace_full
    index_set.insert(1);
    index_set.insert(2);
    index_set.insert(3);

    // The value we are trying to replace that does not exist in the set
    let (index, replaced_value) = index_set.replace_full(4);
}

#[test]
fn test_replace_full_with_replacement() {
    struct HashBuilder;

    impl BuildHasher for HashBuilder {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut index_set: super::IndexSet<u32, HashBuilder> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore {
                indices: vec![],
                entries: vec![],
            },
            hash_builder: HashBuilder,
        },
    };

    // Insert a value we are going to replace later
    index_set.insert(5);

    // Now we will replace it with the same value which will return None
    let (index, replaced_value) = index_set.replace_full(5);
}

