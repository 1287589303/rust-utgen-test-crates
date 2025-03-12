// Answer 0

#[test]
fn test_rcvecbuilder_new() {
    let builder: RcVecBuilder<i32> = RcVecBuilder::new();
}

#[test]
fn test_rcvecbuilder_new_empty() {
    let builder: RcVecBuilder<String> = RcVecBuilder::new();
}

