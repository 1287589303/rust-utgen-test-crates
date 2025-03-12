// Answer 0

#[test]
fn test_deserialize_newtype_struct_valid() {
    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, ()> {
            Ok(())
        }
    }

    let mut vec: Vec<Option<(Content<'static>, Content<'static>)>> = Vec::new();
    let deserializer = FlatMapDeserializer(&mut vec, PhantomData);
    let visitor = MyVisitor;

    let _ = deserializer.deserialize_newtype_struct("MyStruct", visitor);
}

#[test]
fn test_deserialize_newtype_struct_empty_name() {
    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, ()> {
            Ok(())
        }
    }

    let mut vec: Vec<Option<(Content<'static>, Content<'static>)>> = Vec::new();
    let deserializer = FlatMapDeserializer(&mut vec, PhantomData);
    let visitor = MyVisitor;

    let _ = deserializer.deserialize_newtype_struct("", visitor);
}

#[test]
#[should_panic]
fn test_deserialize_newtype_struct_invalid() {
    struct MyVisitor;

    impl<'de> Visitor<'de> for MyVisitor {
        type Value = ();
        fn visit_newtype_struct<V>(self, _: V) -> Result<Self::Value, ()> {
            panic!("Should not be called");
        }
    }

    let mut vec: Vec<Option<(Content<'static>, Content<'static>)>> = vec![None];
    let deserializer = FlatMapDeserializer(&mut vec, PhantomData);
    let visitor = MyVisitor;

    let _ = deserializer.deserialize_newtype_struct("InvalidStruct", visitor);
}

