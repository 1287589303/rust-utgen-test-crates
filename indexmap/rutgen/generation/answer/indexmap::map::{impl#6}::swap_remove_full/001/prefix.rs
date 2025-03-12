// Answer 0

#[test]
fn test_swap_remove_full_single_entry_none_pop() {
    #[derive(Hash, Eq, PartialEq)]
    struct Key(u32);
    
    #[derive(Debug)]
    struct Value(u32);
    
    let mut map: IndexMap<Key, Value, RandomState> = IndexMap::new();
    map.insert(Key(1), Value(42));
    
    let result = map.swap_remove_full(&Key(1));
}

#[test]
fn test_swap_remove_full_single_entry_none_pop_equivalent() {
    #[derive(Hash, Eq, PartialEq)]
    struct Key(u32);
    
    #[derive(Debug)]
    struct Value(u32);
    
    let mut map: IndexMap<Key, Value, RandomState> = IndexMap::new();
    map.insert(Key(10), Value(99));
    
    let result = map.swap_remove_full(&Key(10));
}

