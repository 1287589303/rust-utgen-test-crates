// Answer 0

#[test]
fn test_bernoulli_p_always_true() {
    let bernoulli = Bernoulli { p_int: u64::MAX };
    let _result = bernoulli.p();
}

