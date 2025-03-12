// Answer 0

#[test]
fn test_serialize_char_standard_ascii() {
    let serializer = Serializer;
    let char_input = 'a';
    let _ = serializer.serialize_char(char_input);
}

#[test]
fn test_serialize_char_digit() {
    let serializer = Serializer;
    let char_input = '1';
    let _ = serializer.serialize_char(char_input);
}

#[test]
fn test_serialize_char_null() {
    let serializer = Serializer;
    let char_input = '\0';
    let _ = serializer.serialize_char(char_input);
}

#[test]
fn test_serialize_char_newline() {
    let serializer = Serializer;
    let char_input = '\n';
    let _ = serializer.serialize_char(char_input);
}

#[test]
fn test_serialize_char_tab() {
    let serializer = Serializer;
    let char_input = '\t';
    let _ = serializer.serialize_char(char_input);
}

#[test]
fn test_serialize_char_max_unicode() {
    let serializer = Serializer;
    let char_input = '\u{10FFFF}';
    let _ = serializer.serialize_char(char_input);
}

