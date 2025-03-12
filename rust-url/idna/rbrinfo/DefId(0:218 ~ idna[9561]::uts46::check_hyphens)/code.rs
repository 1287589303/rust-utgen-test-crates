fn check_hyphens(
    mut_label: &mut [char],
    allow_third_fourth: bool,
    fail_fast: bool,
    had_errors: &mut bool,
) -> bool {
    if let Some(first) = mut_label.first_mut() {
        if *first == '-' {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            *first = '\u{FFFD}';
        }
    }
    if let Some(last) = mut_label.last_mut() {
        if *last == '-' {
            if fail_fast {
                return true;
            }
            *had_errors = true;
            *last = '\u{FFFD}';
        }
    }
    if allow_third_fourth {
        return false;
    }
    if mut_label.len() >= 4 && mut_label[2] == '-' && mut_label[3] == '-' {
        if fail_fast {
            return true;
        }
        *had_errors = true;
        mut_label[2] = '\u{FFFD}';
        mut_label[3] = '\u{FFFD}';
    }
    false
}