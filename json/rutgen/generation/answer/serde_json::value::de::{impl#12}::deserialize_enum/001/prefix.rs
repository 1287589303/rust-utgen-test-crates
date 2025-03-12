// Answer 0

#[test]
fn test_deserialize_enum_with_multiple_keys() {
    use serde_json::json;
    let v = json!({
        "variant_key": "variant_value",
        "extra_key": "extra_value",
    });

    let result: Result<_, _> = v.deserialize_any(MyVisitor);
}

#[test]
fn test_deserialize_enum_with_empty_object() {
    use serde_json::json;
    let v = json!({});

    let result: Result<_, _> = v.deserialize_any(MyVisitor);
}

#[test]
fn test_deserialize_enum_with_single_key() {
    use serde_json::json;
    let v = json!({
        "variant_key": "variant_value",
    });

    let result: Result<_, _> = v.deserialize_any(MyVisitor);
}

#[test]
fn test_deserialize_enum_with_multiple_variants() {
    use serde_json::json;
    let v = json!({
        "first_variant": "value1",
        "second_variant": "value2",
    });

    let result: Result<_, _> = v.deserialize_any(MyVisitor);
}

struct MyVisitor;

impl<'de> serde::de::Visitor<'de> for MyVisitor {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with a single key")
    }

    fn visit_enum<V>(self, _variant: V) -> Result<Self::Value, serde::de::Error>
    where
        V: serde::de::EnumAccess<'de>,
    {
        Err(serde::de::Error::invalid_value(serde::de::Unexpected::Map, &"map with a single key"))
    }
}

