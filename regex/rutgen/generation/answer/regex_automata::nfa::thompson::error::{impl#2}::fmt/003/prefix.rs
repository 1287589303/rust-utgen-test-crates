// Answer 0

#[test]
fn test_exceeded_size_limit_1() {
    let error = BuildError::exceeded_size_limit(1);
    let _ = format!("{}", error);
}

#[test]
fn test_exceeded_size_limit_1024() {
    let error = BuildError::exceeded_size_limit(1024);
    let _ = format!("{}", error);
}

#[test]
fn test_exceeded_size_limit_1048576() {
    let error = BuildError::exceeded_size_limit(1048576);
    let _ = format!("{}", error);
}

#[test]
fn test_exceeded_size_limit_1073741824() {
    let error = BuildError::exceeded_size_limit(1073741824);
    let _ = format!("{}", error);
}

#[test]
fn test_exceeded_size_limit_max() {
    let error = BuildError::exceeded_size_limit(2_usize.pow(30));
    let _ = format!("{}", error);
}

