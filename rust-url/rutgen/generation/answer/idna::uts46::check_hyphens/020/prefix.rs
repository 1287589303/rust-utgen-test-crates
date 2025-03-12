// Answer 0

#[test]
fn test_check_hyphens_first_is_not_hyphen_fail_fast_true_allow_third_fourth_false() {
    let mut mut_label: &mut [char] = &mut ['a', 'b', 'c', 'd'];
    let allow_third_fourth = false;
    let fail_fast = true;
    let mut had_errors = false;
    let result = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_first_is_not_hyphen_fail_fast_true_allow_third_fourth_true() {
    let mut mut_label: &mut [char] = &mut ['1', '2', '3', '4'];
    let allow_third_fourth = true;
    let fail_fast = true;
    let mut had_errors = false;
    let result = check_hyphens(mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

