// Answer 0

#[test]
fn test_check_hyphens_case_1() {
    let mut mut_label: &mut [char] = &mut ['-', 'a', 'b'];
    let allow_third_fourth = false;
    let fail_fast = true;
    let mut had_errors = false;

    let result = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
    assert!(result); // Expected to return true
}

#[test]
fn test_check_hyphens_case_2() {
    let mut mut_label: &mut [char] = &mut ['-', 'c', 'd', 'e'];
    let allow_third_fourth = true;
    let fail_fast = true;
    let mut had_errors = false;

    let result = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
    assert!(result); // Expected to return true
}

#[test]
fn test_check_hyphens_case_3() {
    let mut mut_label: &mut [char] = &mut ['-', 'y', 'z', 'x', 'w'];
    let allow_third_fourth = false;
    let fail_fast = true;
    let mut had_errors = false;

    let result = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
    assert!(result); // Expected to return true
}

#[test]
fn test_check_hyphens_case_4() {
    let mut mut_label: &mut [char] = &mut ['-', '1', '2', '3'];
    let allow_third_fourth = false;
    let fail_fast = true;
    let mut had_errors = false;

    let result = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
    assert!(result); // Expected to return true
}

