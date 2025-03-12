// Answer 0

#[test]
fn test_sub_non_empty_self_non_empty_other() {
    let set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    let set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(2);
        set.insert(4);
        set
    };
    let _result = &set1 - &set2;
}

#[test]
fn test_sub_non_empty_self_empty_other() {
    let set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set
    };
    let set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::default();
    let _result = &set1 - &set2;
}

#[test]
fn test_sub_equal_sets() {
    let set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(1);
        set.insert(2);
        set
    };
    let set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(1);
        set.insert(2);
        set
    };
    let _result = &set1 - &set2;
}

#[test]
fn test_sub_non_empty_self_with_empty_element_other() {
    let set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(1);
        set.insert(3);
        set
    };
    let set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(0);
        set.insert(2);
        set
    };
    let _result = &set1 - &set2;
}

#[test]
fn test_sub_edge_case_no_common_elements() {
    let set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(1);
        set.insert(3);
        set.insert(5);
        set
    };
    let set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = {
        let mut set = super::IndexSet::default();
        set.insert(2);
        set.insert(4);
        set
    };
    let _result = &set1 - &set2;
}

