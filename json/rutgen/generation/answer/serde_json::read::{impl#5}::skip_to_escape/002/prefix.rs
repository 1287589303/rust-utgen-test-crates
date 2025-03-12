// Answer 0

#[test]
fn test_skip_to_escape_control_character() {
    let slice = b"abc\x00def"; // contains control character
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    let forbid_control_characters = true;
    
    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_escape_character() {
    let slice = b"abc\\def"; // contains escape character
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    let forbid_control_characters = true;

    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_quote_character() {
    let slice = b"abc\"def"; // contains quote character
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    let forbid_control_characters = true;

    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_multiple_escape_characters() {
    let slice = b"abc\\def\\ghi"; // contains multiple escape characters
    let mut reader = SliceRead::new(slice);
    reader.index = 0;
    let forbid_control_characters = true;

    reader.skip_to_escape(forbid_control_characters);
}

#[test]
fn test_skip_to_escape_with_index_in_range() {
    let slice = b"abcdef\\ghijkl"; // beware of escape characters and control characters
    let mut reader = SliceRead::new(slice);
    reader.index = 6; // index points to 'f'
    let forbid_control_characters = true;

    reader.skip_to_escape(forbid_control_characters);
}

