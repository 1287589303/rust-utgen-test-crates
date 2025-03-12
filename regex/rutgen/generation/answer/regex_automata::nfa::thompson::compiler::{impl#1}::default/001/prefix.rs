// Answer 0

#[test]
fn test_default_which_captures() {
    let captures = WhichCaptures::default();
    let expected = WhichCaptures::All;
    // Function call to be tested
    assert_eq!(captures, expected);
}

