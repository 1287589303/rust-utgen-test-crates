// Answer 0

#[test]
fn test_collect_str_empty() {
    struct TestDisplay;
    impl Display for TestDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.write_str("")
        }
    }
    
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&TestDisplay);
}

#[test]
fn test_collect_str_simple() {
    struct TestDisplay {
        value: &'static str,
    }
    impl Display for TestDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.write_str(self.value)
        }
    }
    
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&TestDisplay { value: "test" });
}

#[test]
fn test_collect_str_numeric() {
    struct TestDisplay {
        value: i32,
    }
    impl Display for TestDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(f, "{}", self.value)
        }
    }
    
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&TestDisplay { value: 123 });
}

#[test]
fn test_collect_str_special_characters() {
    struct TestDisplay {
        value: &'static str,
    }
    impl Display for TestDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.write_str(self.value)
        }
    }
    
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&TestDisplay { value: "special@#$" });
}

#[test]
fn test_collect_str_large_string() {
    struct TestDisplay {
        value: String,
    }
    impl Display for TestDisplay {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.write_str(&self.value)
        }
    }
    
    let large_string = "x".repeat(10_000);
    let serializer = MapKeySerializer;
    let result = serializer.collect_str(&TestDisplay { value: large_string });
}

