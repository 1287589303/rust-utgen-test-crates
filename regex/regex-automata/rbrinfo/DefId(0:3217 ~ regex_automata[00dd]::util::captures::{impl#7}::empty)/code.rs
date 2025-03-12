pub fn empty() -> GroupInfo {
        GroupInfo::new(core::iter::empty::<[Option<&str>; 0]>())
            .expect("empty group info is always valid")
    }