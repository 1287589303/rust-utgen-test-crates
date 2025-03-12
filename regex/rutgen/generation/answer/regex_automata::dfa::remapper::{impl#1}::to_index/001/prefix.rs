// Answer 0

#[test]
fn test_to_index_with_stride2_0() {
    let mapper = IndexMapper { stride2: 0 };
    let id = StateID(0);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_1() {
    let mapper = IndexMapper { stride2: 1 };
    let id = StateID(1);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_2() {
    let mapper = IndexMapper { stride2: 2 };
    let id = StateID(3);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_3() {
    let mapper = IndexMapper { stride2: 3 };
    let id = StateID(7);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_4() {
    let mapper = IndexMapper { stride2: 4 };
    let id = StateID(15);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_5() {
    let mapper = IndexMapper { stride2: 5 };
    let id = StateID(31);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_10() {
    let mapper = IndexMapper { stride2: 10 };
    let id = StateID(1023);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_15() {
    let mapper = IndexMapper { stride2: 15 };
    let id = StateID(32767);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_20() {
    let mapper = IndexMapper { stride2: 20 };
    let id = StateID(1048575);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_30() {
    let mapper = IndexMapper { stride2: 30 };
    let id = StateID(1073741823);
    let result = mapper.to_index(id);
}

#[test]
fn test_to_index_with_stride2_32() {
    let mapper = IndexMapper { stride2: 32 };
    let id = StateID(4294967295);
    let result = mapper.to_index(id);
}

