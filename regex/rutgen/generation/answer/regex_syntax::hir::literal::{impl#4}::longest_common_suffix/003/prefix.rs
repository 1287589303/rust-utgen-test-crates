// Answer 0

#[test]
fn test_longest_common_suffix_non_empty_suffix() {
    let lit1 = Literal::exact(vec![b'f', b'o', b'o']);
    let lit2 = Literal::exact(vec![b'b', b'a', b'r']);
    let lit3 = Literal::exact(vec![b'o', b'o', b'f']);
    let mut seq = Seq::new(vec![lit1.clone(), lit2.clone(), lit3.clone()]);
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_empty_lits() {
    let lit1 = Literal::exact(vec![b'f', b'o', b'o']);
    let mut seq = Seq::new(vec![lit1.clone(), Literal::exact(vec![])]);
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_no_common_suffix() {
    let lit1 = Literal::exact(vec![b'f', b'o', b'o']);
    let lit2 = Literal::exact(vec![b'a', b'b', b'c']);
    let mut seq = Seq::new(vec![lit1.clone(), lit2.clone(), Literal::exact(vec![])]);
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_only_empty() {
    let lit1 = Literal::exact(vec![]);
    let mut seq = Seq::new(vec![lit1.clone(), lit1.clone()]);
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_infinite() {
    let mut seq = Seq::infinite();
    let result = seq.longest_common_suffix();
}

#[test]
fn test_longest_common_suffix_empty_seq() {
    let mut seq = Seq::empty();
    let result = seq.longest_common_suffix();
}

