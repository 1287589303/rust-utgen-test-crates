// Answer 0

#[derive(Clone)]
struct TestStruct;

impl TestStruct {
    fn to_tokens(&self, tokens: &mut std::iter::Once<TestStruct>) {
        tokens.extend(std::iter::once(self.clone()));
    }
}

#[test]
fn test_to_tokens() {
    let mut tokens = std::iter::once(TestStruct);
    let test_struct = TestStruct;

    test_struct.to_tokens(&mut tokens);

    let mut collected: Vec<TestStruct> = tokens.collect();
    assert_eq!(collected.len(), 1);
}

#[test]
fn test_to_tokens_empty() {
    let mut tokens: std::iter::Once<TestStruct> = std::iter::once(TestStruct);
    let empty_struct = TestStruct;

    empty_struct.to_tokens(&mut tokens);

    let collected: Vec<TestStruct> = tokens.collect();
    assert_eq!(collected.len(), 1);
}

