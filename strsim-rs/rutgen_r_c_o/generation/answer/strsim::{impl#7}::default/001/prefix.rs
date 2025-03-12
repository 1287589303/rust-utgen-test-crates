// Answer 0

#[test]
fn test_default_hybrid_growing_hashmap_char_with_integer() {
    struct TestValueType(i32);

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType(0)
        }
    }

    impl Clone for TestValueType {
        fn clone(&self) -> Self {
            TestValueType(self.0)
        }
    }

    impl Copy for TestValueType {}

    impl PartialEq for TestValueType {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let result: HybridGrowingHashmapChar<TestValueType> = HybridGrowingHashmapChar::default();
}

#[test]
fn test_default_hybrid_growing_hashmap_char_with_floating_point() {
    struct TestValueType(f64);

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType(0.0)
        }
    }

    impl Clone for TestValueType {
        fn clone(&self) -> Self {
            TestValueType(self.0)
        }
    }

    impl Copy for TestValueType {}

    impl PartialEq for TestValueType {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let result: HybridGrowingHashmapChar<TestValueType> = HybridGrowingHashmapChar::default();
}

#[test]
fn test_default_hybrid_growing_hashmap_char_with_character() {
    struct TestValueType(char);

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType('a')
        }
    }

    impl Clone for TestValueType {
        fn clone(&self) -> Self {
            TestValueType(self.0)
        }
    }

    impl Copy for TestValueType {}

    impl PartialEq for TestValueType {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    let result: HybridGrowingHashmapChar<TestValueType> = HybridGrowingHashmapChar::default();
}

