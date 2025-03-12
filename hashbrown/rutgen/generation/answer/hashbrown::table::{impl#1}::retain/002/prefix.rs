// Answer 0

#[test]
fn test_retain_with_even_values() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    for x in 1..=6 {
        table.insert_unique(hasher_fn(&x), x, hasher_fn);
    }
    
    table.retain(|&mut x| x % 2 == 0);
    
    let len = table.len();
}

#[test]
#[should_panic]
fn test_retain_empty_on_all_odds() {
    use hashbrown::{HashTable, DefaultHashBuilder};
    
    let mut table = HashTable::new();
    let hasher = DefaultHashBuilder::default();
    let hasher_fn = |val: &_| hasher.hash_one(val);
    
    for x in 1..=5 {
        table.insert_unique(hasher_fn(&x), x, hasher_fn);
    }
    
    table.retain(|&mut x| x % 2 == 0);
    
    let len = table.len();
}

