// Answer 0

#[test]
fn test_max_union_len_self_non_empty_other_infinite() {
    let lit1 = Literal(Box::from("a".as_bytes()));
    let mut self_seq = Seq::new(vec![lit1]);
    let other_seq = Seq::infinite();
    let result = self_seq.max_union_len(&other_seq);
}

#[test]
fn test_max_union_len_self_empty_other_infinite() {
    let mut self_seq = Seq::empty();
    let other_seq = Seq::infinite();
    let result = self_seq.max_union_len(&other_seq);
}

#[test]
fn test_max_union_len_self_singleton_other_infinite() {
    let lit1 = Literal(Box::from("b".as_bytes()));
    let mut self_seq = Seq::singleton(lit1);
    let other_seq = Seq::infinite();
    let result = self_seq.max_union_len(&other_seq);
}

#[test]
fn test_max_union_len_self_multiple_other_infinite() {
    let lit1 = Literal(Box::from("c".as_bytes()));
    let lit2 = Literal(Box::from("d".as_bytes()));
    let mut self_seq = Seq::new(vec![lit1, lit2]);
    let other_seq = Seq::infinite();
    let result = self_seq.max_union_len(&other_seq);
}

#[test]
fn test_max_union_len_self_large_other_infinite() {
    let mut self_seq = Seq::new(vec![Literal(Box::from("e".as_bytes()))]); // 1 element
    // Additional elements to represent bigger numbers
    for i in 0..100 {
        self_seq.push(Literal(Box::from(format!("str{}", i).as_bytes())));
    }
    let other_seq = Seq::infinite();
    let result = self_seq.max_union_len(&other_seq);
}

