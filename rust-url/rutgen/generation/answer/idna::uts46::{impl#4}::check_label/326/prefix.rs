// Answer 0

#[test]
fn test_check_label_boundary_case() {
    let mut had_errors = false;
    let mut label: [char; 200] = ['\u{200D}'; 200];
    let uts46 = Uts46::new();
    let result = uts46.check_label(
        Hyphens::CheckFirstLast,
        &mut label,
        false,
        &mut had_errors,
        false,
        true,
    );
}

#[test]
fn test_check_label_fail_fast_false_case() {
    let mut had_errors = false;
    let mut label: [char; 200] = ['\u{200D}'; 200];
    let uts46 = Uts46::new();
    let result = uts46.check_label(
        Hyphens::CheckFirstLast,
        &mut label,
        false,
        &mut had_errors,
        false,
        true,
    );
}

