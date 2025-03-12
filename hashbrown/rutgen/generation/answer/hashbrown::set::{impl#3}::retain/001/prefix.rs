// Answer 0

#[test]
fn test_retain_even_numbers_empty_set() {
    let mut set: HashSet<i32> = HashSet::new();
    set.retain(|&k| k % 2 == 0);
}

#[test]
fn test_retain_even_numbers_no_evens() {
    let xs = [1, 3, 5, 7, 9];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
}

#[test]
fn test_retain_even_numbers_all_evens() {
    let xs = [0, 2, 4, 6, 8, 10];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
}

#[test]
fn test_retain_even_numbers_mixed() {
    let xs = [0, 1, 2, 3, 4, 5, 6];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
}

#[test]
fn test_retain_even_numbers_with_boundaries() {
    let xs = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut set: HashSet<i32> = xs.into_iter().collect();
    set.retain(|&k| k % 2 == 0);
}

