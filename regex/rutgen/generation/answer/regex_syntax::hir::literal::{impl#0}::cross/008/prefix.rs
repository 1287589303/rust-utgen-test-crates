// Answer 0

#[test]
fn test_cross_with_suffix_kind_and_exceeding_length() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_total(5);

    let lit1 = Literal::exact(vec![b'a']);
    let lit2 = Literal::exact(vec![b'b', b'c']);
    
    let mut seq1 = Seq::singleton(lit1);
    let mut seq2 = Seq::singleton(lit2);
    
    extractor.cross(seq1.clone(), &mut seq2);
}

#[test]
fn test_cross_with_suffix_kind_and_exceeding_length_multiple_literals() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_total(9);

    let lit1 = Literal::exact(vec![b'1']);
    let lit2 = Literal::exact(vec![b'2', b'3']);
    let lit3 = Literal::exact(vec![b'4']);
    
    let mut seq1 = Seq::new(vec![lit1, lit3]);
    let mut seq2 = Seq::new(vec![lit2]);
    
    extractor.cross(seq1.clone(), &mut seq2);
}

#[test]
fn test_cross_with_suffix_kind_and_exceeding_length_empty_seq2() {
    let mut extractor = Extractor::new().kind(ExtractKind::Suffix).limit_total(7);

    let lit1 = Literal::exact(vec![b'x', b'y']);
    
    let mut seq1 = Seq::singleton(lit1);
    let mut seq2 = Seq::empty();
    
    extractor.cross(seq1.clone(), &mut seq2);
}

