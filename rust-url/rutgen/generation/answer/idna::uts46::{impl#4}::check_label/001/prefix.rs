// Answer 0

#[test]
fn test_check_label_hyphens_check_fail_fast_false() {
    let uts46 = Uts46::new();
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['-', 'a', 'b', 'c', '-'];
    
    let result = uts46.check_label(
        Hyphens::Check,
        label,
        false,
        &mut had_errors,
        false,
        false,
    );
}

#[test]
fn test_check_label_hyphens_check_first_last_fail_fast_false() {
    let uts46 = Uts46::new();
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['-', 'a', 'b', 'c', '-'];
    
    let result = uts46.check_label(
        Hyphens::CheckFirstLast,
        label,
        false,
        &mut had_errors,
        false,
        false,
    );
}

#[test]
fn test_check_label_hyphens_check_first_last_fail_fast_true() {
    let uts46 = Uts46::new();
    let mut had_errors = false;
    let mut label: &mut [char] = &mut ['-', 'x', 'y', 'z', '-'];
    
    let result = uts46.check_label(
        Hyphens::CheckFirstLast,
        label,
        true,
        &mut had_errors,
        false,
        false,
    );
}

