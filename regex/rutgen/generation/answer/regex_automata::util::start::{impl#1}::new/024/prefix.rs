// Answer 0

#[test]
fn test_start_byte_map_with_lineterm_CR() {
    let mut lookm = LookMatcher::new();
    lookm.set_line_terminator(b'\r');

    let start_byte_map = StartByteMap::new(&lookm);

    let byte_to_test1 = b':';
    let byte_to_test2 = b'{';
    let byte_to_test3 = b'|';
    let byte_to_test4 = b'~';

    let map_value1 = start_byte_map.get(byte_to_test1);
    let map_value2 = start_byte_map.get(byte_to_test2);
    let map_value3 = start_byte_map.get(byte_to_test3);
    let map_value4 = start_byte_map.get(byte_to_test4);
}

#[test]
fn test_start_byte_map_with_lineterm_NF() {
    let mut lookm = LookMatcher::new();
    lookm.set_line_terminator(b'\x1A'); // an unusual line terminator

    let start_byte_map = StartByteMap::new(&lookm);

    let byte_to_test1 = b':';
    let byte_to_test2 = b'{';
    let byte_to_test3 = b'|';
    let byte_to_test4 = b'~';

    let map_value1 = start_byte_map.get(byte_to_test1);
    let map_value2 = start_byte_map.get(byte_to_test2);
    let map_value3 = start_byte_map.get(byte_to_test3);
    let map_value4 = start_byte_map.get(byte_to_test4);
}

