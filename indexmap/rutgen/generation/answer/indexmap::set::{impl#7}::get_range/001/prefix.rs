// Answer 0

#[test]
fn test_get_range_start_negative() {
    let mut index_set: super::IndexSet<usize, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };
    let result = index_set.get_range(-1..1);
}

#[test]
fn test_get_range_start_equal_to_len() {
    let mut index_set: super::IndexSet<usize, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };
    index_set.pop();
    let result = index_set.get_range(1..2);
}

#[test]
fn test_get_range_end_less_than_start() {
    let mut index_set: super::IndexSet<usize, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };
    let result = index_set.get_range(2..1);
}

#[test]
fn test_get_range_end_equal_to_len() {
    let mut index_set: super::IndexSet<usize, ()> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::new(),
            hash_builder: std::collections::hash_map::RandomState::new(),
        },
    };
    index_set.pop();
    let result = index_set.get_range(0..1);
}

