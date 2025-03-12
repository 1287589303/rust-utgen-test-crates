// Answer 0

#[test]
fn test_check_hyphens_case_1() {
    let mut_label: &mut [char] = &mut ['-', 'a', '-', '-', 'b'];
    let allow_third_fourth = false;
    let fail_fast = false;
    let mut had_errors = false;
    let _ = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case_2() {
    let mut_label: &mut [char] = &mut ['-', 'x', '-', '-', 'y'];
    let allow_third_fourth = false;
    let fail_fast = false;
    let mut had_errors = false;
    let _ = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case_3() {
    let mut_label: &mut [char] = &mut ['-', '1', '-', '-', '2'];
    let allow_third_fourth = false;
    let fail_fast = false;
    let mut had_errors = false;
    let _ = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case_4() {
    let mut_label: &mut [char] = &mut ['-', '!', '-', '-', 'b'];
    let allow_third_fourth = false;
    let fail_fast = false;
    let mut had_errors = false;
    let _ = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

