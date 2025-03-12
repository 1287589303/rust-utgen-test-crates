// Answer 0

#[test]
fn test_check_label_case1() {
    let uts46 = Uts46::new();
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['a', '\u{200D}', 'b']; // contains U+200D
    let result = uts46.check_label(
        Hyphens::Check,
        label,
        true,
        &mut had_errors,
        false,
        true,
    );
}

#[test]
fn test_check_label_case2() {
    let uts46 = Uts46::new();
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['c', '\u{200C}', 'd', '\u{200D}']; // contains U+200C and U+200D
    let result = uts46.check_label(
        Hyphens::CheckFirstLast,
        label,
        true,
        &mut had_errors,
        false,
        true,
    );
}

