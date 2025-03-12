// Answer 0

#[test]
fn test_fmt_multiple_elements_map() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    struct ExpectedInMap(usize);

    let mut formatter = TestFormatter {};
    
    let test_cases = vec![2, 3, 10, 100];

    for &count in &test_cases {
        let element = ExpectedInMap(count);
        let _ = element.fmt(&mut formatter);
    }
}

#[test]
fn test_fmt_large_number_of_elements_map() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    struct ExpectedInMap(usize);

    let mut formatter = TestFormatter {};
    
    let test_cases = vec![1000, 5000, 10000, 100000];

    for &count in &test_cases {
        let element = ExpectedInMap(count);
        let _ = element.fmt(&mut formatter);
    }
}

