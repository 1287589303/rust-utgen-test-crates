// Answer 0

#[test]
fn test_from_iter_empty() {
    let v: Vec<i32> = vec![];
    let x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_integers() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_booleans() {
    let v: Vec<bool> = vec![true, false, true];
    let x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_strings() {
    let v: Vec<&str> = vec!["a", "b", "c"];
    let x: Value = v.into_iter().collect();
}

#[test]
fn test_from_iter_mixed() {
    let v: Vec<Box<dyn std::fmt::Display>> = vec!["text".to_owned().into_boxed_str(), 42.into()];
    let x: Value = v.into_iter().map(|v| v.to_string()).collect();
}

