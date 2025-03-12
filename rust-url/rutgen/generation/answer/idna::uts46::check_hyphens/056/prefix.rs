// Answer 0

#[test]
fn test_check_hyphens_case1() {
    let mut had_errors = false;
    let mut mut_label: &mut [char] = &mut ['-', 'a', '-', '-'];
    let allow_third_fourth = false;
    let fail_fast = false;
    check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case2() {
    let mut had_errors = false;
    let mut mut_label: &mut [char] = &mut ['-', 'b', '-', '-'];
    let allow_third_fourth = false;
    let fail_fast = true;
    check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_case3() {
    let mut had_errors = false;
    let mut mut_label: &mut [char] = &mut ['-', 'c', '-', '-'];
    let allow_third_fourth = false;
    let fail_fast = false;
    check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

