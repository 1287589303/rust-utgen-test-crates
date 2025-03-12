// Answer 0

#[test]
fn test_encode_into_case_1() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input = "ö";
    let mut output = String::new();
    let result = encode_into(input.chars(), &mut output);
}

#[test]
fn test_encode_into_case_2() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = true;
    }

    let input = "abc"; // basic_length == 0
    let mut output = String::new();
    let result = encode_into(input.chars(), &mut output);
}

#[test]
fn test_encode_into_case_3() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input = "ÿñ"; // processed < input_length
    let mut output = String::new();
    let result = encode_into(input.chars(), &mut output);
} 

#[test]
fn test_encode_into_case_4() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input = "õ"; // c equals code_point
    let mut output = String::new();
    let result = encode_into(input.chars(), &mut output);
}

#[test]
fn test_encode_into_case_5() {
    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let input = "ü"; // results in exceeding bias + T_MAX
    let mut output = String::new();
    let result = encode_into(input.chars(), &mut output);
}

