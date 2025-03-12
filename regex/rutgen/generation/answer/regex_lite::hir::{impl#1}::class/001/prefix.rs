// Answer 0

#[test]
fn test_class_empty_range() {
    let class = Class {
        ranges: vec![],
    };
    let result = Hir::class(class);
}

#[test]
fn test_class_single_character_range() {
    let class = Class {
        ranges: vec![ClassRange { start: 'a', end: 'a' }],
    };
    let result = Hir::class(class);
}

#[test]
fn test_class_multiple_character_ranges() {
    let class = Class {
        ranges: vec![
            ClassRange { start: 'a', end: 'c' },
            ClassRange { start: 'e', end: 'g' },
        ],
    };
    let result = Hir::class(class);
}

#[test]
fn test_class_overlapping_ranges() {
    let class = Class {
        ranges: vec![
            ClassRange { start: 'a', end: 'd' },
            ClassRange { start: 'c', end: 'f' },
        ],
    };
    let result = Hir::class(class);
}

#[test]
fn test_class_range_with_single_gap() {
    let class = Class {
        ranges: vec![
            ClassRange { start: 'a', end: 'b' },
            ClassRange { start: 'd', end: 'e' },
        ],
    };
    let result = Hir::class(class);
}

