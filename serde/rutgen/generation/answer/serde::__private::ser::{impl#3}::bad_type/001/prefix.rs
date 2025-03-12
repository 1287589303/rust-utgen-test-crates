// Answer 0

#[test]
fn test_bad_type_boolean() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Boolean);
}

#[test]
fn test_bad_type_integer() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Integer);
}

#[test]
fn test_bad_type_float() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Float);
}

#[test]
fn test_bad_type_char() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Char);
}

#[test]
fn test_bad_type_string() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::String);
}

#[test]
fn test_bad_type_byte_array() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::ByteArray);
}

#[test]
fn test_bad_type_optional() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Optional);
}

#[test]
fn test_bad_type_sequence() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Sequence);
}

#[test]
fn test_bad_type_tuple() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::Tuple);
}

#[test]
fn test_bad_type_tuple_struct() {
    struct DummySerializer;
    impl SerializeMap for DummySerializer {
        type Ok = ();
        type Error = serde::ser::Error;
        fn serialize_entry<K, V>(&mut self, key: K, value: V) -> Result<(), Self::Error> 
        where 
            K: Serialize, 
            V: Serialize { 
            Ok(()) 
        }
        fn end(self) -> Result<Self::Ok, Self::Error> { 
            Ok(()) 
        }
    }

    let mut serializer = DummySerializer;
    let error = FlatMapSerializer(&mut serializer).bad_type(Unsupported::TupleStruct);
}

