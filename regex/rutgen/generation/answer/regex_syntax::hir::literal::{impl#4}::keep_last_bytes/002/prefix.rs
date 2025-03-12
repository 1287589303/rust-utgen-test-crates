// Answer 0

#[test]
fn test_keep_last_bytes_non_empty_literals_exact() {
    let mut seq = Seq::new(vec![
        Literal::exact("hello"),
        Literal::exact("world"),
        Literal::exact("rust"),
    ]);
    seq.keep_last_bytes(2);
}

#[test]
fn test_keep_last_bytes_non_empty_literals_inexact() {
    let mut seq = Seq::new(vec![
        Literal::exact("example"),
        Literal::exact("testing"),
        Literal::exact("strings"),
    ]);
    seq.keep_last_bytes(4);
}

#[test]
fn test_keep_last_bytes_zero_length() {
    let mut seq = Seq::new(vec![
        Literal::exact("trimmed"),
        Literal::exact("to"),
        Literal::exact("zero"),
    ]);
    seq.keep_last_bytes(0);
}

#[test]
fn test_keep_last_bytes_exceeding_length() {
    let mut seq = Seq::new(vec![
        Literal::exact("short"),
        Literal::exact("longer_than"),
        Literal::exact("maximum"),
    ]);
    seq.keep_last_bytes(15);
}

#[test]
fn test_keep_last_bytes_literals_same_length() {
    let mut seq = Seq::new(vec![
        Literal::exact("abc"),
        Literal::exact("def"),
        Literal::exact("ghi"),
    ]);
    seq.keep_last_bytes(3);
}

#[test]
fn test_keep_last_bytes_mixed_length() {
    let mut seq = Seq::new(vec![
        Literal::exact("short"),
        Literal::exact("a bit longer"),
        Literal::exact("tiny"),
    ]);
    seq.keep_last_bytes(1);
}

