// Answer 0

#[test]
fn test_build_many_sparse_with_valid_and_invalid_patterns() {
    let builder = Builder::new();
    let valid_pattern = "a*b";
    let invalid_pattern = "[a-z"; // unbalanced brackets

    let patterns: Vec<&str> = vec![valid_pattern, invalid_pattern];

    let result = builder.build_many_sparse(&patterns);
}

#[test]
fn test_build_many_sparse_with_multiple_invalid_patterns() {
    let builder = Builder::new();
    let invalid_pattern1 = "[abc"; // unbalanced brackets
    let invalid_pattern2 = "a{2,1}"; // invalid quantifier
    let invalid_patterns: Vec<&str> = vec![invalid_pattern1, invalid_pattern2];

    let result = builder.build_many_sparse(&invalid_patterns);
}

#[test]
fn test_build_many_sparse_with_valid_patterns_only() {
    let builder = Builder::new();
    let valid_pattern1 = "abc";
    let valid_pattern2 = "123";

    let patterns: Vec<&str> = vec![valid_pattern1, valid_pattern2];

    let result = builder.build_many_sparse(&patterns);
}

