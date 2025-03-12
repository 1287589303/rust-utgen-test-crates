// Answer 0

#[test]
fn test_osa_distance_case_1() {
    let a = "abc";
    let b = "bca";
    let result = osa_distance(a, b);
    assert_eq!(result, 3);
}

#[test]
fn test_osa_distance_case_2() {
    let a = "ab";
    let b = "cab";
    let result = osa_distance(a, b);
    assert_eq!(result, 2);
}

#[test]
fn test_osa_distance_case_3() {
    let a = "abcde";
    let b = "abdec";
    let result = osa_distance(a, b);
    assert_eq!(result, 2);
}

#[test]
fn test_osa_distance_case_4() {
    let a = "hello";
    let b = "ehllo";
    let result = osa_distance(a, b);
    assert_eq!(result, 1);
}

#[test]
fn test_osa_distance_case_5() {
    let a = "test";
    let b = "tset";
    let result = osa_distance(a, b);
    assert_eq!(result, 1);
}

