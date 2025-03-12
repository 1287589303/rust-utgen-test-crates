// Answer 0

#[test]
fn test_max_cross_len_non_empty_sequences() {
    let lit1 = Literal(Box::new(b"test1".to_vec()));
    let lit2 = Literal(Box::new(b"test2".to_vec()));
    
    let mut seq1 = Seq::new(vec![&lit1.bytes, &lit2.bytes]);
    let mut seq2 = Seq::new(vec![&lit1.bytes, &lit2.bytes]);

    let result = seq1.max_cross_len(&seq2);
}

#[test]
fn test_max_cross_len_one_empty_one_non_empty() {
    let lit1 = Literal(Box::new(b"example".to_vec()));

    let mut seq1 = Seq::new(vec![&lit1.bytes]);
    let mut seq2 = Seq::empty();

    let result = seq1.max_cross_len(&seq2);
}

#[test]
fn test_max_cross_len_finite_sequences() {
    let lit1 = Literal(Box::new(b"a".to_vec()));
    let lit2 = Literal(Box::new(b"b".to_vec()));

    let mut seq1 = Seq::new(vec![&lit1.bytes, &lit2.bytes]);
    let mut seq2 = Seq::new(vec![&lit1.bytes, &lit2.bytes]);

    let result = seq1.max_cross_len(&seq2);
}

#[test]
fn test_max_cross_len_large_sequences() {
    let lit1 = Literal(Box::new(b"longsequence1".to_vec()));
    let lit2 = Literal(Box::new(b"longsequence2".to_vec()));

    let mut seq1 = Seq::new(vec![&lit1.bytes; 500]); // 500 literals
    let mut seq2 = Seq::new(vec![&lit2.bytes; 500]); // 500 literals

    let result = seq1.max_cross_len(&seq2);
}

#[test]
fn test_max_cross_len_varied_lengths() {
    let lit1 = Literal(Box::new(b"short".to_vec()));
    let lit2 = Literal(Box::new(b"muchlonger".to_vec()));
    
    let mut seq1 = Seq::new(vec![&lit1.bytes; 100]); // 100 literals
    let mut seq2 = Seq::new(vec![&lit2.bytes; 200]); // 200 literals
    
    let result = seq1.max_cross_len(&seq2);
}

