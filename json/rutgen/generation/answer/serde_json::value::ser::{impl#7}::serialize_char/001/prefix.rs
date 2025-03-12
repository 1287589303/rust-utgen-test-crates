// Answer 0

#[test]
fn test_serialize_char_null() {
    let serializer = MapKeySerializer;
    let value = '\0';
    let _ = serializer.serialize_char(value);
}

#[test]
fn test_serialize_char_first_unicode() {
    let serializer = MapKeySerializer;
    let value = '\u{0000}';
    let _ = serializer.serialize_char(value);
}

#[test]
fn test_serialize_char_last_unicode() {
    let serializer = MapKeySerializer;
    let value = '\u{10FFFF}';
    let _ = serializer.serialize_char(value);
}

#[test]
fn test_serialize_char_boundary_characters() {
    let serializer = MapKeySerializer;
    let value = '\u{D7FF}';
    let _ = serializer.serialize_char(value);
}

#[test]
fn test_serialize_char_non_graphic() {
    let serializer = MapKeySerializer;
    let value = '\u{0009}'; // Horizontal Tab
    let _ = serializer.serialize_char(value);
}

#[test]
fn test_serialize_char_control_character() {
    let serializer = MapKeySerializer;
    let value = '\u{001B}'; // Escape character
    let _ = serializer.serialize_char(value);
}

#[test]
fn test_serialize_char_spacing_character() {
    let serializer = MapKeySerializer;
    let value = ' ';
    let _ = serializer.serialize_char(value);
}

