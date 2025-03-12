// Answer 0

#[test]
fn test_next_value_seed_with_bool() {
    let content = Content::Bool(true);
    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: Some(content),
        fields: &[],
        _marker: PhantomData,
    };
    let seed = BoolSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_u8() {
    let content = Content::U8(255);
    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: Some(content),
        fields: &[],
        _marker: PhantomData,
    };
    let seed = U8Seed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_string() {
    let content = Content::String(String::from("test"));
    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: Some(content),
        fields: &[],
        _marker: PhantomData,
    };
    let seed = StringSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: Some(content),
        fields: &[],
        _marker: PhantomData,
    };
    let seed = SeqSeed;
    let _ = access.next_value_seed(seed);
}

#[test]
fn test_next_value_seed_with_map() {
    let content = Content::Map(vec![
        (Content::String(String::from("key1")), Content::U32(123)),
        (Content::String(String::from("key2")), Content::U32(456)),
    ]);
    let mut access = FlatStructAccess {
        iter: &mut [].iter_mut(),
        pending_content: Some(content),
        fields: &[],
        _marker: PhantomData,
    };
    let seed = MapSeed;
    let _ = access.next_value_seed(seed);
}

