// Answer 0

#[test]
fn test_osa_distance_case_1() {
    let a = "ab";
    let b = "cba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_2() {
    let a = "abcd";
    let b = "dcba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_3() {
    let a = "abcde";
    let b = "edcba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_4() {
    let a = "xyz";
    let b = "yxzabc";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_5() {
    let a = "123456";
    let b = "6543210";
    let result = osa_distance(a, b);
}

