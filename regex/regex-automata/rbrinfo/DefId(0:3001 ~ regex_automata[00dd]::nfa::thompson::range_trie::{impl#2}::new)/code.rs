fn new(state_id: StateID, ranges: &[Utf8Range]) -> NextInsert {
        let len = ranges.len();
        assert!(len > 0);
        assert!(len <= 4);

        let mut tmp = [Utf8Range { start: 0, end: 0 }; 4];
        tmp[..len].copy_from_slice(ranges);
        NextInsert { state_id, ranges: tmp, len: u8::try_from(len).unwrap() }
    }