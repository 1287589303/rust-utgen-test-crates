// Answer 0

#[test]
fn test_max_literal_len_none() {
    let seq = Seq { literals: None };
    let result = seq.max_literal_len();
}

#[test]
fn test_max_literal_len_empty_vec() {
    let seq = Seq { literals: Some(vec![]) };
    let result = seq.max_literal_len();
}

