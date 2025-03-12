// Answer 0

#[test]
fn test_longest_common_suffix_multiple_literals_no_common_suffix() {
    let lit1 = Literal::exact(b"hello".to_vec());
    let lit2 = Literal::exact(b"world".to_vec());
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_varied_suffixes() {
    let lit1 = Literal::exact(b"prefix1_suffix".to_vec());
    let lit2 = Literal::exact(b"prefix2_diff".to_vec());
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_one_empty_literal() {
    let lit1 = Literal::exact(b"nonempty".to_vec());
    let lit2 = Literal::exact(b"".to_vec());
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_common_prefix_no_suffix() {
    let lit1 = Literal::exact(b"shared_prefix1".to_vec());
    let lit2 = Literal::exact(b"shared_prefix2".to_vec());
    let seq = Seq::new(vec![lit1, lit2]);
    seq.longest_common_suffix();
}

