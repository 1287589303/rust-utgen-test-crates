// Answer 0

#[test]
fn test_extend_with_zero_elements() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    let input: Vec<&i32> = Vec::new();
    index_set.extend(input);
}

#[test]
fn test_extend_with_one_element() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    let value = 42;
    let input: Vec<&i32> = vec![&value];
    index_set.extend(input);
}

#[test]
fn test_extend_with_duplicates() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    let value1 = 10;
    let value2 = 20;
    let input: Vec<&i32> = vec![&value1, &value1, &value2, &value2];
    index_set.extend(input);
}

#[test]
fn test_extend_with_large_collection() {
    let mut index_set: IndexSet<i32, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    let input: Vec<&i32> = (0..1000).map(|x| &x).collect();
    index_set.extend(input);
}

#[test]
fn test_extend_with_empty_input() {
    let mut index_set: IndexSet<String, RandomState> = IndexSet {
        map: IndexMap {
            core: IndexMapCore::new(),
            hash_builder: RandomState::new(),
        },
    };
    let input: Vec<&String> = Vec::new();
    index_set.extend(input);
}

