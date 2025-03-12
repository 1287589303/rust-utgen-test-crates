// Answer 0

#[test]
fn test_is_canonical_strictly_increasing_non_contiguous() {
    let ranges = vec![
        ClassRange { start: 'a', end: 'b' },
        ClassRange { start: 'c', end: 'd' },
        ClassRange { start: 'e', end: 'f' },
    ];
    let class = Class::new(ranges);
    class.is_canonical();
}

#[test]
fn test_is_canonical_boundary_case() {
    let ranges = vec![
        ClassRange { start: '1', end: '2' },
        ClassRange { start: '4', end: '5' },
        ClassRange { start: '7', end: '8' },
    ];
    let class = Class::new(ranges);
    class.is_canonical();
} 

#[test]
fn test_is_canonical_with_upper_and_lower_case() {
    let ranges = vec![
        ClassRange { start: 'A', end: 'A' },
        ClassRange { start: 'C', end: 'C' },
        ClassRange { start: 'E', end: 'E' },
    ];
    let class = Class::new(ranges);
    class.is_canonical();
} 

#[test]
fn test_is_canonical_different_characters() {
    let ranges = vec![
        ClassRange { start: '!', end: '!' },
        ClassRange { start: '#', end: '#' },
        ClassRange { start: '%', end: '%' },
    ];
    let class = Class::new(ranges);
    class.is_canonical();
} 

#[test]
fn test_is_canonical_large_gaps() {
    let ranges = vec![
        ClassRange { start: 'a', end: 'a' },
        ClassRange { start: 'z', end: 'z' },
        ClassRange { start: '{', end: '{' },
    ];
    let class = Class::new(ranges);
    class.is_canonical();
} 

