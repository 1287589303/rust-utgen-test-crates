// Answer 0

#[test]
fn test_split_prefix_matching() {
    let input_str = "hello world";
    let mut input = Input::new_no_trim(input_str);
    let self_input = Input::new_no_trim("hello");
    let result = self_input.split_prefix(&mut input);
}

#[test]
fn test_split_prefix_non_matching() {
    let input_str = "hello world";
    let mut input = Input::new_no_trim(input_str);
    let self_input = Input::new_no_trim("world");
    let result = self_input.split_prefix(&mut input);
}

#[test]
fn test_split_prefix_empty_self() {
    let input_str = "hello world";
    let mut input = Input::new_no_trim(input_str);
    let self_input = Input::new_no_trim("");
    let result = self_input.split_prefix(&mut input);
}

#[test]
fn test_split_prefix_empty_input() {
    let mut input = Input::new_no_trim("");
    let self_input = Input::new_no_trim("test");
    let result = self_input.split_prefix(&mut input);
}

#[test]
fn test_split_prefix_exact_match() {
    let input_str = "exact match";
    let mut input = Input::new_no_trim(input_str);
    let self_input = Input::new_no_trim("exact match");
    let result = self_input.split_prefix(&mut input);
}

