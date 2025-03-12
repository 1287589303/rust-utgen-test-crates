pub fn matches(group_info: GroupInfo) -> Captures {
        // This is OK because we know there are at least this many slots,
        // and GroupInfo construction guarantees that the number of slots fits
        // into a usize.
        let slots = group_info.pattern_len().checked_mul(2).unwrap();
        Captures { group_info, pid: None, slots: vec![None; slots] }
    }