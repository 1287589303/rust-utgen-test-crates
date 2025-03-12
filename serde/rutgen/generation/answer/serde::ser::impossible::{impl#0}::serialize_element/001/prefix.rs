// Answer 0

#[test]
fn test_serialize_element_empty_struct() {
    struct EmptyStruct;

    let mut serializer = Impossible::<(), Error> { void: Void, ok: PhantomData, error: PhantomData };
    let value = EmptyStruct;
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_nested_struct() {
    struct NestedStruct {
        field: u32,
    }

    struct OuterStruct {
        nested: NestedStruct,
    }

    let mut serializer = Impossible::<(), Error> { void: Void, ok: PhantomData, error: PhantomData };
    let value = OuterStruct { nested: NestedStruct { field: 42 } };
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_tuple() {
    let mut serializer = Impossible::<(), Error> { void: Void, ok: PhantomData, error: PhantomData };
    let value = (1, "test");
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_enum() {
    enum MyEnum {
        Variant1,
        Variant2(u32),
    }

    let mut serializer = Impossible::<(), Error> { void: Void, ok: PhantomData, error: PhantomData };
    let value = MyEnum::Variant2(10);
    let _ = serializer.serialize_element(&value);
}

#[test]
fn test_serialize_element_with_future() {
    use futures::future::ready;

    let mut serializer = Impossible::<(), Error> { void: Void, ok: PhantomData, error: PhantomData };
    let value = ready(3);
    let _ = serializer.serialize_element(&value);
}

