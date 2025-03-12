// Answer 0

#[test]
fn test_swap_remove_present_integer() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let result = set.swap_remove(&2);
}

#[test]
fn test_swap_remove_not_present_integer() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    set.insert(1);
    set.insert(2);
    let result = set.swap_remove(&3);
}

#[test]
fn test_swap_remove_empty_set() {
    let mut set: IndexSet<i32, RandomState> = IndexSet::new();
    let result = set.swap_remove(&1);
}

#[test]
fn test_swap_remove_present_string() {
    let mut set: IndexSet<String, RandomState> = IndexSet::new();
    set.insert("foo".to_string());
    set.insert("bar".to_string());
    let result = set.swap_remove(&"bar".to_string());
}

#[test]
fn test_swap_remove_not_present_string() {
    let mut set: IndexSet<String, RandomState> = IndexSet::new();
    set.insert("foo".to_string());
    set.insert("bar".to_string());
    let result = set.swap_remove(&"baz".to_string());
}

#[test]
fn test_swap_remove_equivalent_structs() {
    #[derive(Hash, PartialEq, Eq)]
    struct Wrapper {
        value: i32,
    }
    
    let mut set: IndexSet<Wrapper, RandomState> = IndexSet::new();
    set.insert(Wrapper { value: 1 });
    set.insert(Wrapper { value: 2 });
    let result = set.swap_remove(&Wrapper { value: 1 });
}

#[test]
fn test_swap_remove_empty_set_struct() {
    #[derive(Hash, PartialEq, Eq)]
    struct Wrapper {
        value: i32,
    }

    let mut set: IndexSet<Wrapper, RandomState> = IndexSet::new();
    let result = set.swap_remove(&Wrapper { value: 1 });
}

