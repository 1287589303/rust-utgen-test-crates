// Answer 0

#[test]
fn test_intersection_greater_len_positive_integers_with_overlap() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(2);
    b.insert(3);
    b.insert(4);
    
    let intersection = a.intersection(&b);
    // The intersection should yield the iter for {2, 3} in a.
}

#[test]
fn test_intersection_greater_len_positive_integers_no_overlap() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(3);
    b.insert(4);
    
    let intersection = a.intersection(&b);
    // The intersection should yield the iter for an empty set.
}

#[test]
fn test_intersection_greater_len_one_element_in_common() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(5);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(5);
    b.insert(6);
    b.insert(7);
    
    let intersection = a.intersection(&b);
    // The intersection should yield the iter for {5}.
}

#[test]
fn test_intersection_greater_len_multiple_elements_some_common() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    
    let mut b: HashSet<i32> = HashSet::new();
    b.insert(3);
    b.insert(4);
    b.insert(5);
    
    let intersection = a.intersection(&b);
    // The intersection should yield the iter for {3}.
}

