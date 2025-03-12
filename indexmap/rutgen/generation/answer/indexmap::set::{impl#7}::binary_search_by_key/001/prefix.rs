// Answer 0

#[test]
fn test_binary_search_by_key_value_present() {
    let mut set = super::IndexSet::<i32, ()>::new();
    set.insert(1);
    set.insert(3);
    set.insert(5);
    set.insert(7);
    
    let result = set.binary_search_by_key(&3, |&x| x);
}

#[test]
fn test_binary_search_by_key_value_not_present_lower() {
    let mut set = super::IndexSet::<i32, ()>::new();
    set.insert(2);
    set.insert(4);
    set.insert(6);
    set.insert(8);
    
    let result = set.binary_search_by_key(&1, |&x| x);
}

#[test]
fn test_binary_search_by_key_value_not_present_upper() {
    let mut set = super::IndexSet::<i32, ()>::new();
    set.insert(2);
    set.insert(4);
    set.insert(6);
    
    let result = set.binary_search_by_key(&10, |&x| x);
}

#[test]
fn test_binary_search_by_key_value_present_duplicate() {
    let mut set = super::IndexSet::<i32, ()>::new();
    set.insert(3);
    set.insert(3);
    set.insert(5);
    
    let result = set.binary_search_by_key(&3, |&x| x);
}

#[test]
fn test_binary_search_by_key_bottom_middle() {
    let mut set = super::IndexSet::<i32, ()>::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    
    let result = set.binary_search_by_key(&3, |&x| x);
}

#[test]
fn test_binary_search_by_key_sorted_random() {
    let mut set = super::IndexSet::<i32, ()>::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);
    set.insert(40);
    
    let result = set.binary_search_by_key(&25, |&x| x);
}

