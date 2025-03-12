// Answer 0

#[test]
fn test_extract_if_even_numbers() {
    let mut set: HashSet<i32> = (0..10).collect();
    let extracted = set.extract_if(|v| v % 2 == 0);
}

#[test]
fn test_extract_if_odd_numbers() {
    let mut set: HashSet<i32> = (1..10).collect();
    let extracted = set.extract_if(|v| v % 2 != 0);
}

#[test]
fn test_extract_if_all_true() {
    let mut set: HashSet<i32> = (0..5).collect();
    let extracted = set.extract_if(|_| true);
}

#[test]
fn test_extract_if_none_true() {
    let mut set: HashSet<i32> = (0..5).collect();
    let extracted = set.extract_if(|_| false);
}

#[test]
fn test_extract_if_empty_set() {
    let mut set: HashSet<i32> = HashSet::new();
    let extracted = set.extract_if(|_| true);
}

