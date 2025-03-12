// Answer 0

#[test]
fn test_check_hyphens_both_ends_hyphen_fail_fast() {
    let mut label: &mut [char] = &mut ['-', 'a', 'b', 'c'];
    let allow_third_fourth = false;
    let fail_fast = true;
    let mut had_errors = false;
    let result = check_hyphens(label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_both_ends_hyphen_no_fail_fast() {
    let mut label: &mut [char] = &mut ['-', 'a', 'b', 'c'];
    let allow_third_fourth = false;
    let fail_fast = false;
    let mut had_errors = false;
    let result = check_hyphens(label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_replacement_hyphens_no_fail_fast() {
    let mut label: &mut [char] = &mut ['-', 'a', 'b', 'c'];
    let allow_third_fourth = false;
    let fail_fast = false;
    let mut had_errors = false;
    let result = check_hyphens(label, allow_third_fourth, fail_fast, &mut had_errors);
}

