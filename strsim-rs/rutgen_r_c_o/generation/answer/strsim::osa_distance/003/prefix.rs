// Answer 0

#[test]
fn test_osa_distance_case1() {
    let a = "abc";
    let b = "def";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case2() {
    let a = "abcd";
    let b = "efgh";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case3() {
    let a = "ab";
    let b = "cd";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case4() {
    let a = "xyz";
    let b = "ABC";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_empty_a() {
    let a = "";
    let b = "xyz";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_empty_b() {
    let a = "abc";
    let b = "";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_with_transposition() {
    let a = "ab";
    let b = "ba";
    let result = osa_distance(a, b);
}

#[test]
fn test_osa_distance_case_with_repeated_chars() {
    let a = "aabb";
    let b = "bbaa";
    let result = osa_distance(a, b);
}

