// Answer 0

#[test]
fn test_osa_distance_case1() {
    let a = "abcd";
    let b = "abce";
    let _result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case2() {
    let a = "hello";
    let b = "hallo";
    let _result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case3() {
    let a = "kitten";
    let b = "sittin";
    let _result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case4() {
    let a = "test";
    let b = "tset";
    let _result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case5() {
    let a = "rust";
    let b = "ruts";
    let _result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case6() {
    let a = "abcd";
    let b = "dcba";
    let _result = osa_distance(a, b);
}

