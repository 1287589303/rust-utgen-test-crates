// Answer 0

#[test]
fn test_is_disjoint_both_empty() {
    let set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    let set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_one_empty() {
    let set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    let mut set_b = hashbrown::HashSet::new();
    set_b.insert(1);
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_no_common_elements() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_b.insert(3);
    set_b.insert(4);
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_with_common_element() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_a.insert(1);
    set_a.insert(2);
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    set_b.insert(2);
    set_b.insert(3);
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_max_capacity() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    
    for i in 0..1000 {
        set_a.insert(i);
    }
    for j in 1000..2000 {
        set_b.insert(j);
    }
    set_a.is_disjoint(&set_b);
}

#[test]
fn test_is_disjoint_max_capacity_with_common() {
    let mut set_a: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    let mut set_b: hashbrown::HashSet<i32> = hashbrown::HashSet::new();
    
    for i in 0..1000 {
        set_a.insert(i);
    }
    set_b.insert(500);
    set_b.insert(600);
    set_a.is_disjoint(&set_b);
}

