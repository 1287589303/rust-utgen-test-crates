// Answer 0

#[test]
fn test_new_empty_iterator() {
    let ranges: Vec<ClassRange> = Vec::new();
    let class = Class::new(ranges);
}

#[test]
fn test_new_single_range() {
    let range = ClassRange { start: 'a', end: 'a' };
    let class = Class::new(vec![range]);
}

#[test]
fn test_new_multiple_adjacent_ranges() {
    let ranges = vec![
        ClassRange { start: 'a', end: 'c' },
        ClassRange { start: 'd', end: 'f' },
    ];
    let class = Class::new(ranges);
}

#[test]
fn test_new_overlapping_ranges() {
    let ranges = vec![
        ClassRange { start: 'a', end: 'd' },
        ClassRange { start: 'c', end: 'f' },
    ];
    let class = Class::new(ranges);
}

#[test]
fn test_new_random_order_ranges() {
    let ranges = vec![
        ClassRange { start: 'z', end: 'z' },
        ClassRange { start: 'b', end: 'b' },
        ClassRange { start: 'a', end: 'a' },
    ];
    let class = Class::new(ranges);
}

#[test]
fn test_new_maximum_ranges() {
    let ranges: Vec<ClassRange> = (0..=127)
        .map(|i| ClassRange { start: char::from(i as u8), end: char::from(i as u8) })
        .collect();
    let class = Class::new(ranges);
}

