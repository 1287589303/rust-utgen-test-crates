// Answer 0

#[test]
fn test_longest_common_prefix_with_none_literals() {
    let seq = Seq::empty();
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_with_empty_literals() {
    let seq = Seq::new(vec![].into_iter());
    seq.longest_common_prefix();
}

#[test]
fn test_longest_common_prefix_with_infinite() {
    let seq = Seq::infinite();
    seq.longest_common_prefix();
}

