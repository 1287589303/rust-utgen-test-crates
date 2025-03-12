// Answer 0

#[test]
fn test_serialize_element_bool() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = true; // boolean value
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_u8() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = 255u8; // maximum u8 value
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_u16() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = 65535u16; // maximum u16 value
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_u32() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = 4294967295u32; // maximum u32 value
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_string() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = String::from("test"); // string value
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_vec_u8() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = vec![1, 2, 3]; // vector of u8
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_none() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value = Option::<u8>::None; // None value
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_empty_vec() {
    struct ErrorStub;
    impl ser::Error for ErrorStub {}

    let mut serializer = SerializeTuple::<ErrorStub> { elements: Vec::new(), error: PhantomData };

    let value: Vec<u8> = Vec::new(); // empty vector
    let _ = serializer.serialize_element(&value);
}

