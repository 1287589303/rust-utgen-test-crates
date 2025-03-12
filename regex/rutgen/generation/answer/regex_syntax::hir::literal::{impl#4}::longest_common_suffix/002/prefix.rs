// Answer 0

#[test]
fn test_longest_common_suffix_some_empty() {
    let lit1 = Literal::exact(b"hello_world_suffix");
    let lit2 = Literal::exact(b"goodbye_world_suffix");
    let mut seq = Seq::new(vec![lit1, lit2]);
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_some_empty_aligned() {
    let lit1 = Literal::exact(b"test123_alpha");
    let lit2 = Literal::exact(b"example123_alpha");
    let mut seq = Seq::new(vec![lit1, lit2]);
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_some_empty_multiple() {
    let lit1 = Literal::exact(b"sample_test_case");
    let lit2 = Literal::exact(b"mock_test_case");
    let mut seq = Seq::new(vec![lit1, lit2]);
    let result = seq.longest_common_suffix();
}

