pub fn memory_usage(&self) -> usize {
        use core::mem::size_of as s;

        s::<GroupInfoInner>()
            + self.0.slot_ranges.len() * s::<(SmallIndex, SmallIndex)>()
            + self.0.name_to_index.len() * s::<CaptureNameMap>()
            + self.0.index_to_name.len() * s::<Vec<Option<Arc<str>>>>()
            + self.0.memory_extra
    }