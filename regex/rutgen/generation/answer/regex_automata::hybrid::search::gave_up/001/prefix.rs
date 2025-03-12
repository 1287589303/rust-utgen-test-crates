// Answer 0

#[test]
fn test_gave_up_min_offset() {
    let offset = 0;
    let error = gave_up(offset);
}

#[test]
fn test_gave_up_small_offset() {
    let offset = 1;
    let error = gave_up(offset);
}

#[test]
fn test_gave_up_max_offset() {
    let offset = std::usize::MAX;
    let error = gave_up(offset);
}

