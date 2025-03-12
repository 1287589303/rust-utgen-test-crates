// Answer 0

#[test]
fn test_serialize_map_some() {
    struct MockMap;
    
    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;
        
        fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
        
        fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<Self::Ok, Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_map(Some(10));
}

#[test]
fn test_serialize_map_none() {
    struct MockMap;
    
    impl SerializeMap for MockMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }
        
        fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<Self::Ok, Self::Error>
        where
            K: Serialize,
            V: Serialize,
        {
            Ok(())
        }
        
        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }
    
    let mut map = MockMap;
    let serializer = FlatMapSerializer(&mut map);
    let result = serializer.serialize_map(None);
}

