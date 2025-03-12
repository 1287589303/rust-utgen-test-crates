// Answer 0

#[test]
fn test_visit_array_empty_array() {
    struct FailingVisitor;

    impl<'de> Visitor<'de> for FailingVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Err(Error { err: Box::new(ErrorImpl {}) })
        }
    }

    let array: Vec<Value> = vec![];
    let visitor = FailingVisitor;

    let _result = visit_array(array, visitor);
}

#[test]
fn test_visit_array_single_element() {
    struct FailingVisitor;

    impl<'de> Visitor<'de> for FailingVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Err(Error { err: Box::new(ErrorImpl {}) })
        }
    }

    let array: Vec<Value> = vec![Value::Bool(true)];
    let visitor = FailingVisitor;

    let _result = visit_array(array, visitor);
}

#[test]
fn test_visit_array_multiple_elements() {
    struct FailingVisitor;

    impl<'de> Visitor<'de> for FailingVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Err(Error { err: Box::new(ErrorImpl {}) })
        }
    }

    let array: Vec<Value> = vec![Value::Bool(true), Value::Number(Number::from(10))];
    let visitor = FailingVisitor;

    let _result = visit_array(array, visitor);
}

#[test]
fn test_visit_array_complex_structure() {
    struct FailingVisitor;

    impl<'de> Visitor<'de> for FailingVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Err(Error { err: Box::new(ErrorImpl {}) })
        }
    }

    let array: Vec<Value> = vec![
        Value::Array(vec![Value::Number(Number::from(1))]),
        Value::String("test".to_owned()),
    ];
    let visitor = FailingVisitor;

    let _result = visit_array(array, visitor);
}

