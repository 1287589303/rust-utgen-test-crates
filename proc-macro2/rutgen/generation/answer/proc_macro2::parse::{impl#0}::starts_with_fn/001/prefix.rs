// Answer 0

#[test]
fn starts_with_fn_non_empty_cursor_with_always_true_function() {
    let cursor = Cursor { rest: "hello" };
    let result = cursor.starts_with_fn(|_| true);
}

#[test]
fn starts_with_fn_non_empty_cursor_with_always_false_function() {
    let cursor = Cursor { rest: "hello" };
    let result = cursor.starts_with_fn(|_| false);
}

#[test]
fn starts_with_fn_non_empty_cursor_with_specific_character_function() {
    let cursor = Cursor { rest: "hello" };
    let result = cursor.starts_with_fn(|ch| ch == 'h');
}

#[test]
fn starts_with_fn_non_empty_cursor_with_single_character() {
    let cursor = Cursor { rest: "a" };
    let result = cursor.starts_with_fn(|ch| ch == 'a');
}

#[test]
fn starts_with_fn_empty_cursor() {
    let cursor = Cursor { rest: "" };
    let result = cursor.starts_with_fn(|_| true);
}

#[test]
fn starts_with_fn_cursor_with_no_matching_characters() {
    let cursor = Cursor { rest: "hello" };
    let result = cursor.starts_with_fn(|ch| ch == 'z');
}

