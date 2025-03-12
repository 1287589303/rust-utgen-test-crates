// Answer 0

#[test]
fn test_append_non_overlapping_keys() {
    let mut a = IndexMap::<i32, &str, RandomState>::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::<i32, &str, RandomState>::from([(3, "c"), (4, "d")]);
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    a.len();
    b.len();
    b.capacity();
}

#[test]
fn test_append_overlapping_keys() {
    let mut a = IndexMap::<i32, &str, RandomState>::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::<i32, &str, RandomState>::from([(2, "c"), (3, "d")]);
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    a.len();
    b.len();
    b.capacity();
}

#[test]
fn test_append_empty_map() {
    let mut a = IndexMap::<i32, &str, RandomState>::from([(1, "a")]);
    let mut b: IndexMap<i32, &str, RandomState> = IndexMap::new();
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    a.len();
    b.len();
    b.capacity();
}

#[test]
fn test_append_single_entry_map() {
    let mut a = IndexMap::<i32, &str, RandomState>::from([(1, "a"), (2, "b")]);
    let mut b = IndexMap::<i32, &str, RandomState>::from([(3, "c")]);
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    a.len();
    b.len();
    b.capacity();
}

#[test]
fn test_append_large_maps() {
    let mut a = IndexMap::<i32, &str, RandomState>::new();
    for i in 0..1000 {
        a.insert(i, "value");
    }
    
    let mut b = IndexMap::<i32, &str, RandomState>::new();
    for i in 1000..2000 {
        b.insert(i, "value");
    }
    
    let old_capacity = b.capacity();
    a.append(&mut b);
    
    a.len();
    b.len();
    b.capacity();
}

