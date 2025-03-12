// Answer 0

#[test]
fn test_serialize_newtype_variant_err_1() {
    struct DummyDelegate;

    impl Serializer for DummyDelegate {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeMap = ErrMap;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(Error)
        }

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct ErrMap;

    impl SerializeMap for ErrMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TaggedSerializer {
        type_ident: "type",
        variant_ident: "variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: DummyDelegate,
    };
    
    let result: Result<(), Error> = serializer.serialize_newtype_variant("name", 0, "inner_variant", &());
}

#[test]
fn test_serialize_newtype_variant_err_2() {
    struct FailingDelegate;

    impl Serializer for FailingDelegate {
        type Ok = ();
        type Error = Error;
        type SerializeSeq = Impossible<(), Error>;
        type SerializeTuple = Impossible<(), Error>;
        type SerializeTupleStruct = Impossible<(), Error>;
        type SerializeMap = ErrMap;
        type SerializeStruct = Impossible<(), Error>;
        type SerializeStructVariant = Impossible<(), Error>;

        fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            Err(Error)
        }

        fn serialize_entry<K, V>(&mut self, _: &K, _: &V) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    struct ErrMap;

    impl SerializeMap for ErrMap {
        type Ok = ();
        type Error = Error;

        fn serialize_key<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn serialize_value<T>(&mut self, _: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            Ok(())
        }
    }

    let serializer = TaggedSerializer {
        type_ident: "type",
        variant_ident: "variant",
        tag: "tag",
        variant_name: "variant_name",
        delegate: FailingDelegate,
    };

    let result: Result<(), Error> = serializer.serialize_newtype_variant("name", 0, "inner_variant", &());
}

