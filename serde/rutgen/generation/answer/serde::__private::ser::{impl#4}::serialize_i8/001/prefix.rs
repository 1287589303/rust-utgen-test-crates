// Answer 0

#[test]
fn test_serialize_i8_negative() {
    struct DummyMap;
    impl SerializeMap for DummyMap {
        type Ok = ();
        type Error = Error;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<Self::Ok, Self::Error> 
        where K: Serialize, V: Serialize { Err(Error) }
        fn serialize_key<K>(&mut self, _key: K) -> Result<Self::Ok, Self::Error> 
        where K: Serialize { Err(Error) }
    }

    let mut map = DummyMap;
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i8(-128);
}

#[test]
fn test_serialize_i8_zero() {
    struct DummyMap;
    impl SerializeMap for DummyMap {
        type Ok = ();
        type Error = Error;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<Self::Ok, Self::Error> 
        where K: Serialize, V: Serialize { Err(Error) }
        fn serialize_key<K>(&mut self, _key: K) -> Result<Self::Ok, Self::Error> 
        where K: Serialize { Err(Error) }
    }

    let mut map = DummyMap;
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i8(0);
}

#[test]
fn test_serialize_i8_positive() {
    struct DummyMap;
    impl SerializeMap for DummyMap {
        type Ok = ();
        type Error = Error;
        fn serialize_entry<K, V>(&mut self, _key: K, _value: V) -> Result<Self::Ok, Self::Error> 
        where K: Serialize, V: Serialize { Err(Error) }
        fn serialize_key<K>(&mut self, _key: K) -> Result<Self::Ok, Self::Error> 
        where K: Serialize { Err(Error) }
    }

    let mut map = DummyMap;
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_i8(127);
}

