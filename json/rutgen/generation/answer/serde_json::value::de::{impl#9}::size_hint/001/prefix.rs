// Answer 0

#[test]
fn test_size_hint_lower_bound_none_upper_bound() {
    let values = vec![Value::Null];
    let iter = values.clone().into_iter();
    let deserializer = SeqDeserializer { iter };
    deserializer.size_hint();
}

#[test]
fn test_size_hint_lower_bound_unequal_upper_bound() {
    let values = vec![Value::Bool(true), Value::Bool(false)];
    let iter = values.clone().into_iter();
    let deserializer = SeqDeserializer { iter };
    deserializer.size_hint();
}

