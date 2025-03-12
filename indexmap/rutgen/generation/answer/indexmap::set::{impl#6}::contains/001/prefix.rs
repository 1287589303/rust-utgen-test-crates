// Answer 0

#[test]
fn test_contains_with_integer() {
    struct MyHashBuilder;
    
    let mut set: IndexSet<i32, MyHashBuilder> = IndexSet {
        map: IndexMap { core: IndexMapCore::new(), hash_builder: MyHashBuilder },
    };

    set.insert(1);
    set.insert(2);
    assert!(set.contains(&1));
    assert!(set.contains(&2));
    assert!(!set.contains(&3));
}

#[test]
fn test_contains_with_string() {
    struct MyHashBuilder;
    
    let mut set: IndexSet<String, MyHashBuilder> = IndexSet {
        map: IndexMap { core: IndexMapCore::new(), hash_builder: MyHashBuilder },
    };

    set.insert("hello".to_string());
    set.insert("world".to_string());
    assert!(set.contains(&"hello".to_string()));
    assert!(set.contains(&"world".to_string()));
    assert!(!set.contains(&"rust".to_string()));
}

#[test]
fn test_contains_empty_set() {
    struct MyHashBuilder;
    
    let set: IndexSet<i32, MyHashBuilder> = IndexSet {
        map: IndexMap { core: IndexMapCore::new(), hash_builder: MyHashBuilder },
    };

    assert!(!set.contains(&1));    
}

#[test]
fn test_contains_with_different_types() {
    struct MyHashBuilder;
    
    let mut set: IndexSet<i32, MyHashBuilder> = IndexSet {
        map: IndexMap { core: IndexMapCore::new(), hash_builder: MyHashBuilder },
    };

    set.insert(1);
    assert!(set.contains(&1));
    // This test assumes that different types will not match
    assert!(!set.contains(&1.0));
}

#[test]
fn test_contains_large_set() {
    struct MyHashBuilder;
    
    let mut set: IndexSet<i32, MyHashBuilder> = IndexSet {
        map: IndexMap { core: IndexMapCore::new(), hash_builder: MyHashBuilder },
    };

    for i in 0..1000 {
        set.insert(i);
    }
    
    for i in 0..1000 {
        assert!(set.contains(&i));
    }
    assert!(!set.contains(&1001));
}

#[test]
fn test_contains_custom_hasher() {
    struct MyCustomHasher;
    
    let mut set: IndexSet<i32, MyCustomHasher> = IndexSet {
        map: IndexMap { core: IndexMapCore::new(), hash_builder: MyCustomHasher },
    };

    set.insert(10);
    set.insert(20);
    assert!(set.contains(&10));
    assert!(set.contains(&20));
    assert!(!set.contains(&30));
}

