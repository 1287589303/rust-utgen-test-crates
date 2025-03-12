// Answer 0

#[cfg(test)]
fn test_if_any_valid_decoding_case_1() {
    let input = b"hello%20world";
    let percent_decode = PercentDecode { bytes: input.iter() };
    let _result = percent_decode.if_any();
}

#[cfg(test)]
fn test_if_any_valid_decoding_case_2() {
    let input = b"data%3Fquery%3Dvalue";
    let percent_decode = PercentDecode { bytes: input.iter() };
    let _result = percent_decode.if_any();
}

#[cfg(test)]
fn test_if_any_valid_decoding_case_3() {
    let input = b"encode%7Athis%21";
    let percent_decode = PercentDecode { bytes: input.iter() };
    let _result = percent_decode.if_any();
}

