pub fn empty(group_info: GroupInfo) -> Captures {
        Captures { group_info, pid: None, slots: vec![] }
    }