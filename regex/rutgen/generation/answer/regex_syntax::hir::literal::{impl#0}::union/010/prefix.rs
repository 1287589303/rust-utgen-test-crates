// Answer 0

#[test]
fn test_union_with_valid_inputs() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Prefix)
        .limit_class(1)
        .limit_repeat(2)
        .limit_literal_len(3)
        .limit_total(4);

    let mut seq1 = Seq::new(vec![b"abc", b"def", b"ghi"]);
    let mut seq2 = Seq::new(vec![b"jkl", b"mno", b"pqr"]);

    extractor.union(seq1.clone(), &mut seq2);
}

#[test]
fn test_union_with_exceeding_seq1_length() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Suffix)
        .limit_class(1)
        .limit_repeat(2)
        .limit_literal_len(3)
        .limit_total(2);

    let mut seq1 = Seq::new(vec![b"abcd", b"efgh"]);
    let mut seq2 = Seq::new(vec![b"ijkl"]);

    extractor.union(seq1.clone(), &mut seq2);
}

