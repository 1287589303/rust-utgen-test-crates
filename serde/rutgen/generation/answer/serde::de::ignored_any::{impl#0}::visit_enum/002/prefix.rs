// Answer 0

#[test]
fn test_visit_enum_valid_variant() {
    struct ValidEnumAccess;
    impl<'de> EnumAccess<'de> for ValidEnumAccess {
        type Error = ();

        fn variant<V>(self) -> Result<(V, Self), Self::Error> 
        where 
            V: Visitor<'de>, 
        {
            Ok((IgnoredAny, self))
        }
    }
    
    let _ = IgnoredAny.visit_enum(ValidEnumAccess);
}

#[test]
fn test_visit_enum_invalid_variant() {
    struct InvalidEnumAccess;
    impl<'de> EnumAccess<'de> for InvalidEnumAccess {
        type Error = ();

        fn variant<V>(self) -> Result<(V, Self), Self::Error> 
        where 
            V: Visitor<'de>, 
        {
            Err(())
        }
    }
    
    let result = IgnoredAny.visit_enum(InvalidEnumAccess);
    let _ = result.unwrap_err();
}

#[test]
fn test_visit_enum_empty_variant() {
    struct EmptyEnumAccess;
    impl<'de> EnumAccess<'de> for EmptyEnumAccess {
        type Error = ();

        fn variant<V>(self) -> Result<(V, Self), Self::Error> 
        where 
            V: Visitor<'de>, 
        {
            Ok((IgnoredAny, self))
        }
    }
    
    let _ = IgnoredAny.visit_enum(EmptyEnumAccess);
}

#[test]
fn test_visit_enum_complex_variant() {
    struct ComplexEnumAccess;
    impl<'de> EnumAccess<'de> for ComplexEnumAccess {
        type Error = ();

        fn variant<V>(self) -> Result<(V, Self), Self::Error> 
        where 
            V: Visitor<'de>, 
        {
            Ok((IgnoredAny, self))
        }
    }
    
    let _ = IgnoredAny.visit_enum(ComplexEnumAccess);
}

