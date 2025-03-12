// Answer 0

#[test]
fn test_values_default() {
    let _values: Values<(), ()> = Values::default();
}

#[test]
fn test_values_default_with_types() {
    struct Key;
    struct Value;

    let _values: Values<Key, Value> = Values::default();
}

#[test]
fn test_values_default_with_different_types() {
    struct StringKey;
    struct IntValue;

    let _values: Values<StringKey, IntValue> = Values::default();
}

