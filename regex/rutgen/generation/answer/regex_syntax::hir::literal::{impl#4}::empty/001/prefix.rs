// Answer 0

#[test]
fn test_empty_sequence() {
    let seq = Seq::empty();
}

#[test]
fn test_empty_sequence_content() {
    let seq = Seq::empty();
    let literals = seq.literals();
}

#[test]
fn test_empty_sequence_length() {
    let seq = Seq::empty();
    let length = seq.len();
}

