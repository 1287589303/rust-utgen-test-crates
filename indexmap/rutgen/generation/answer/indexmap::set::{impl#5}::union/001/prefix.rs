// Answer 0

#[test]
fn test_union_with_non_empty_sets_unique_elements() {
    use std::collections::hash_map::RandomState;
    
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(4);
    set2.insert(5);
    set2.insert(6);

    let union_iter = set1.union(&set2);
}

#[test]
fn test_union_with_non_empty_sets_common_elements() {
    use std::collections::hash_map::RandomState;
    
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    let union_iter = set1.union(&set2);
}

#[test]
fn test_union_with_non_empty_sets_duplicates() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(2);
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    let union_iter = set1.union(&set2);
}

#[test]
fn test_union_with_non_empty_sets_order_variations() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };

    set1.insert(3);
    set1.insert(1);
    set1.insert(2);
    set2.insert(1);
    set2.insert(4);
    set2.insert(5);

    let union_iter = set1.union(&set2);
}

#[test]
fn test_union_with_identical_sets() {
    use std::collections::hash_map::RandomState;
    
    let mut set1: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };
    let mut set2: IndexSet<i32, RandomState> = IndexSet { map: IndexMap { core: IndexMapCore::new(), hash_builder: RandomState::default() } };

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);

    let union_iter = set1.union(&set2);
}

