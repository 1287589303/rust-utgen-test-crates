pub fn all(group_info: GroupInfo) -> Captures {
        let slots = group_info.slot_len();
        Captures { group_info, pid: None, slots: vec![None; slots] }
    }