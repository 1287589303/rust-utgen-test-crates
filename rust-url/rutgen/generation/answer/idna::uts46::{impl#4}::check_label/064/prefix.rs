// Answer 0

#[test]
fn test_check_label_hyphen_check_first_last() {
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['\u{0300}', 'a', '\u{200D}']; // Combining mark first, valid characters
    let hyphens = Hyphens::CheckFirstLast;
    let fail_fast = false;
    let needs_contextj_check = true;

    let uts46 = Uts46::new();
    let result = uts46.check_label(
        hyphens,
        label,
        fail_fast,
        &mut had_errors,
        true,
        needs_contextj_check,
    );
}

#[test]
fn test_check_label_hyphen_check() {
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['\u{0300}', 'b', '\u{200D}']; // Combining mark first, valid characters
    let hyphens = Hyphens::Check;
    let fail_fast = false;
    let needs_contextj_check = true;

    let uts46 = Uts46::new();
    let result = uts46.check_label(
        hyphens,
        label,
        fail_fast,
        &mut had_errors,
        true,
        needs_contextj_check,
    );
}

