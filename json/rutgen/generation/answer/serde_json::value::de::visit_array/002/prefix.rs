// Answer 0

#[test]
fn test_visit_array_non_empty() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;
        
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![Value::Null, Value::Bool(true)])
        }
    }

    let input_array = vec![Value::Null, Value::Bool(true), Value::Number(Number::from(10))];
    let visitor = TestVisitor;

    let _result = visit_array(input_array, visitor);
}

#[test]
fn test_visit_array_with_multiple_types() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;
        
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![Value::String("test".to_string()), Value::Number(Number::from(20))])
        }
    }

    let input_array = vec![Value::String("test".to_string()), Value::Number(Number::from(20)), Value::Array(vec![Value::Bool(false)])];
    let visitor = TestVisitor;

    let _result = visit_array(input_array, visitor);
}

#[test]
fn test_visit_array_empty_elements_remaining() {
    struct TestVisitor;
    
    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Value>;
        
        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![Value::Number(Number::from(1))])
        }
    }

    let input_array = vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))];
    let visitor = TestVisitor;

    let _result = visit_array(input_array, visitor);
}

