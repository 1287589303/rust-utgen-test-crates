// Answer 0

#[test]
fn test_check_hyphens_first_last_hyphen_fail_fast() {
    let mut had_errors = false;
    let mut mut_label = ['-', 'a', 'b', 'c'];
    let allow_third_fourth = false;
    let fail_fast = true;

    check_hyphens(&mut mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

#[test]
fn test_check_hyphens_first_last_hyphen_no_fail_fast() {
    let mut had_errors = false;
    let mut mut_label = ['-', 'a', 'b', 'c'];
    let allow_third_fourth = false;
    let fail_fast = false;

    check_hyphens(&mut mut_label, allow_third_fourth, fail_fast, &mut had_errors);
}

