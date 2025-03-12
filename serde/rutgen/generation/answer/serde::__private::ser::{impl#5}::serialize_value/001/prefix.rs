// Answer 0

#[test]
fn test_serialize_value_string() {
    struct TestMap {
        values: Vec<String>,
    }
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize {
            self.values.push(format!("{:?}", value));
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let mut map = TestMap { values: Vec::new() };
    let mut serializer = FlatMapSerializeMap(&mut map);
    let value: &str = "test_string";

    let _ = serializer.serialize_value(&value);
}

#[test]
fn test_serialize_value_integer() {
    struct TestMap {
        values: Vec<i32>,
    }
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize {
            if let Some(&v) = value.downcast_ref::<i32>() {
                self.values.push(v);
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let mut map = TestMap { values: Vec::new() };
    let mut serializer = FlatMapSerializeMap(&mut map);
    let value: i32 = 42;

    let _ = serializer.serialize_value(&value);
}

#[test]
fn test_serialize_value_float() {
    struct TestMap {
        values: Vec<f64>,
    }
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize {
            if let Some(&v) = value.downcast_ref::<f64>() {
                self.values.push(v);
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let mut map = TestMap { values: Vec::new() };
    let mut serializer = FlatMapSerializeMap(&mut map);
    let value: f64 = 3.14;

    let _ = serializer.serialize_value(&value);
}

#[test]
fn test_serialize_value_boolean() {
    struct TestMap {
        values: Vec<bool>,
    }
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize {
            if let Some(&v) = value.downcast_ref::<bool>() {
                self.values.push(v);
            }
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let mut map = TestMap { values: Vec::new() };
    let mut serializer = FlatMapSerializeMap(&mut map);
    let value: bool = true;

    let _ = serializer.serialize_value(&value);
}

#[test]
fn test_serialize_value_empty_structure() {
    #[derive(Debug)]
    struct EmptyStruct;

    struct TestMap {
        values: Vec<EmptyStruct>,
    }
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize {
            self.values.push(EmptyStruct);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let mut map = TestMap { values: Vec::new() };
    let mut serializer = FlatMapSerializeMap(&mut map);
    let value = EmptyStruct;

    let _ = serializer.serialize_value(&value);
}

#[test]
fn test_serialize_value_null() {
    struct TestMap {
        values: Vec<Option<()>>,
    }
    
    impl SerializeMap for TestMap {
        type Ok = ();
        type Error = ();
        
        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize { Ok(()) }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error> 
        where T: ?Sized + Serialize {
            self.values.push(None);
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> { Ok(()) }
    }

    let mut map = TestMap { values: Vec::new() };
    let mut serializer = FlatMapSerializeMap(&mut map);
    let value: Option<()> = None;

    let _ = serializer.serialize_value(&value);
}

