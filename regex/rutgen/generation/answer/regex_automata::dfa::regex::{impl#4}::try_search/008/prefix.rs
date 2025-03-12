// Answer 0

#[test]
fn test_try_search_fwd_some() {
    let input_data: Vec<u8> = b"hello world".to_vec();
    let input = Input::new(&input_data).span(0..input_data.len()).anchored(Anchored::No);
    let regex = Regex::new("hello").unwrap();
    let result = regex.try_search(&input);
}

#[test]
fn test_try_search_fwd_none() {
    let input_data: Vec<u8> = b"goodbye".to_vec();
    let input = Input::new(&input_data).span(0..input_data.len()).anchored(Anchored::No);
    let regex = Regex::new("hello").unwrap();
    let result = regex.try_search(&input);
}

#[test]
fn test_try_search_fwd_anchored() {
    let input_data: Vec<u8> = b"hello world".to_vec();
    let input = Input::new(&input_data).span(0..input_data.len()).anchored(Anchored::Yes);
    let regex = Regex::new("hello").unwrap();
    let result = regex.try_search(&input);
}

#[test]
fn test_try_search_empty_input() {
    let input_data: Vec<u8> = Vec::new();
    let input = Input::new(&input_data).span(0..0).anchored(Anchored::No);
    let regex = Regex::new(".*").unwrap();
    let result = regex.try_search(&input);
}

