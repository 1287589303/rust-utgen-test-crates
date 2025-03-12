// Answer 0

#[test]
fn test_collect_str_empty_string() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer::new() };
    let value = "";
    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_single_character() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer::new() };
    let value = "a";
    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_large_string() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer::new() };
    let value = "a".repeat(1000);
    let _ = serializer.collect_str(&value);
}

#[test]
fn test_collect_str_special_characters() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer::new() };
    let value = "Hello, world! @#%&*()";
    let _ = serializer.collect_str(&value);
}

#[test]
#[should_panic]
fn test_collect_str_none() {
    let mut serializer = MapKeySerializer { ser: &mut Serializer::new() };
    let value: Option<&str> = None;
    let _ = serializer.collect_str(&value);
}

