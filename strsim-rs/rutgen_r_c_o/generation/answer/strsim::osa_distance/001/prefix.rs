// Answer 0

#[test]
fn test_osa_distance_case_1() {
    let a = "abcd";
    let b = "abdc";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_2() {
    let a = "hello";
    let b = "holle";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_3() {
    let a = "rust";
    let b = "ruts";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_4() {
    let a = "ab";
    let b = "ba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_5() {
    let a = "testing";
    let b = "setitng";
    let result = osa_distance(a, b);
}

