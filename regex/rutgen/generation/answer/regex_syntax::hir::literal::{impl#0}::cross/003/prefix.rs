// Answer 0

#[test]
fn test_cross_suffix_limit_exceed() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Suffix)
        .limit_total(10);

    let seq1 = Seq::new(vec![vec![b'a'], vec![b'b']]);
    let mut seq2 = Seq::new(vec![vec![b'c'], vec![b'd']]);

    let result = extractor.cross(seq1, &mut seq2);
}

#[test]
fn test_cross_suffix_limit_exact() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Suffix)
        .limit_total(4);

    let seq1 = Seq::new(vec![vec![b'a'], vec![b'b']]);
    let mut seq2 = Seq::new(vec![vec![b'c']]);

    let result = extractor.cross(seq1, &mut seq2);
}

#[test]
fn test_cross_suffix_limit_under() {
    let mut extractor = Extractor::new()
        .kind(ExtractKind::Suffix)
        .limit_total(5);

    let seq1 = Seq::new(vec![vec![b'a']]);
    let mut seq2 = Seq::new(vec![vec![b'b'], vec![b'c']]);

    let result = extractor.cross(seq1, &mut seq2);
}

