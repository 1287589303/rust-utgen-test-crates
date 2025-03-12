// Answer 0

#[test]
fn test_longest_common_suffix_none_empty() {
    let seq = Seq::empty();
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_none_infinite() {
    let seq = Seq::infinite();
    let _ = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_none_no_literals() {
    let seq = Seq::new(vec![]);
    let _ = seq.longest_common_suffix();
}

