// Answer 0

#[test]
fn test_splice_valid_range_with_replacement() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = vec![(5, 'E'), (4, 'D'), (3, 'C')];
    let removed: Vec<_> = map.splice(1..3, new).collect();
}

#[test]
fn test_splice_full_replacement() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c')]);
    let new = vec![(5, 'E'), (6, 'F')];
    let removed: Vec<_> = map.splice(0..4, new).collect();
}

#[test]
fn test_splice_no_replacement_in_range() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b'), (3, 'c'), (4, 'd')]);
    let new = vec![(5, 'E'), (6, 'F')];
    let removed: Vec<_> = map.splice(2..3, new).collect();
}

#[test]
#[should_panic]
fn test_splice_panic_start_gt_end() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = vec![(5, 'E')];
    let _removed: Vec<_> = map.splice(2..1, new).collect();
}

#[test]
#[should_panic]
fn test_splice_panic_end_gt_length() {
    let mut map = IndexMap::from([(0, '_'), (1, 'a'), (2, 'b')]);
    let new = vec![(5, 'E')];
    let _removed: Vec<_> = map.splice(0..4, new).collect();
}

