// Answer 0

#[test]
fn test_next_back_valid_id_minimum() {
    let patset = PatternSet::new(vec![0.into()]).unwrap();
    let it = (0..1);
    let mut iter = SetMatchesIntoIter { patset, it };
    let result = iter.next_back();
}

#[test]
fn test_next_back_valid_id_mid() {
    let patset = PatternSet::new(vec![1.into(), 2.into(), 3.into()]).unwrap();
    let it = (0..4);
    let mut iter = SetMatchesIntoIter { patset, it };
    let result = iter.next_back();
}

#[test]
fn test_next_back_valid_id_maximum() {
    let patset = PatternSet::new(vec![0.into(), 4.into()]).unwrap();
    let it = (0..5);
    let mut iter = SetMatchesIntoIter { patset, it };
    let result = iter.next_back();
}

#[test]
fn test_next_back_multiple_matches() {
    let patset = PatternSet::new(vec![2.into(), 4.into(), 6.into()]).unwrap();
    let it = (0..7);
    let mut iter = SetMatchesIntoIter { patset, it };
    let result = iter.next_back();
}

