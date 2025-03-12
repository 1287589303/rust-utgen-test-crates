// Answer 0

#[test]
fn test_with_capacity_zero() {
    let map: crate::HashMap<&str, i32> = crate::HashMap::with_capacity(0);
}

#[test]
fn test_with_capacity_one() {
    let map: crate::HashMap<&str, i32> = crate::HashMap::with_capacity(1);
}

#[test]
fn test_with_capacity_ten() {
    let map: crate::HashMap<&str, i32> = crate::HashMap::with_capacity(10);
}

#[test]
fn test_with_capacity_max_usize() {
    let map: crate::HashMap<&str, i32> = crate::HashMap::with_capacity(std::usize::MAX);
}

