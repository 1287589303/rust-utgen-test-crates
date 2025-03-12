// Answer 0

#[test]
fn test_check_hyphens_case_1() {
    let mut label: &mut [char] = &mut ['-', 'a', 'b', '-', 'c'];
    let allow_third_fourth = true;
    let fail_fast = false;
    let mut had_errors = false;
    check_hyphens(label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case_2() {
    let mut label: &mut [char] = &mut ['-', '1', '2', '-', '3'];
    let allow_third_fourth = true;
    let fail_fast = false;
    let mut had_errors = false;
    check_hyphens(label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case_3() {
    let mut label: &mut [char] = &mut ['-', '!', '@', '-', '#'];
    let allow_third_fourth = true;
    let fail_fast = false;
    let mut had_errors = false;
    check_hyphens(label, allow_third_fourth, fail_fast, &mut had_errors);
}

