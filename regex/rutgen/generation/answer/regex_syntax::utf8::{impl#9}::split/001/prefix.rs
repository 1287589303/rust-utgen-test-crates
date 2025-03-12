// Answer 0

#[test]
fn test_split_overlapping_surrogate_range_1() {
    let scalar_range = ScalarRange { start: 0x0000, end: 0xFFFF };
    let result = scalar_range.split();
}

#[test]
fn test_split_overlapping_surrogate_range_2() {
    let scalar_range = ScalarRange { start: 0xD7FE, end: 0x10FFFF };
    let result = scalar_range.split();
}

#[test]
fn test_split_overlapping_surrogate_range_3() {
    let scalar_range = ScalarRange { start: 0xD700, end: 0xE000 };
    let result = scalar_range.split();
}

#[test]
fn test_split_overlapping_surrogate_range_4() {
    let scalar_range = ScalarRange { start: 0xA000, end: 0x10FFFE };
    let result = scalar_range.split();
}

