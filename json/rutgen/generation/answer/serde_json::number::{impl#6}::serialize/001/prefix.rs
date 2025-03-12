// Answer 0

#[test]
fn test_serialize_float_positive() {
    let number = Number {
        n: N::Float(1.5),
    };
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    number.serialize(&mut serializer).unwrap();
}

#[test]
fn test_serialize_float_negative() {
    let number = Number {
        n: N::Float(-2.5),
    };
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    number.serialize(&mut serializer).unwrap();
}

#[test]
fn test_serialize_float_zero() {
    let number = Number {
        n: N::Float(0.0),
    };
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    number.serialize(&mut serializer).unwrap();
}

#[test]
fn test_serialize_float_large() {
    let number = Number {
        n: N::Float(1.0e+308),
    };
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    number.serialize(&mut serializer).unwrap();
}

#[test]
fn test_serialize_float_small() {
    let number = Number {
        n: N::Float(-1.0e+308),
    };
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    number.serialize(&mut serializer).unwrap();
}

