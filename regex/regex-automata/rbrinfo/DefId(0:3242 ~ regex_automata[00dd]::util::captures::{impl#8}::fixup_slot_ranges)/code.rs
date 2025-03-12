fn fixup_slot_ranges(&mut self) -> Result<(), GroupInfoError> {
        use crate::util::primitives::IteratorIndexExt;
        // Since we know number of patterns fits in PatternID and
        // PatternID::MAX < isize::MAX, it follows that multiplying by 2 will
        // never overflow usize.
        let offset = self.pattern_len().checked_mul(2).unwrap();
        for (pid, &mut (ref mut start, ref mut end)) in
            self.slot_ranges.iter_mut().with_pattern_ids()
        {
            let group_len = 1 + ((end.as_usize() - start.as_usize()) / 2);
            let new_end = match end.as_usize().checked_add(offset) {
                Some(new_end) => new_end,
                None => {
                    return Err(GroupInfoError::too_many_groups(
                        pid, group_len,
                    ))
                }
            };
            *end = SmallIndex::new(new_end).map_err(|_| {
                GroupInfoError::too_many_groups(pid, group_len)
            })?;
            // Since start <= end, if end is valid then start must be too.
            *start = SmallIndex::new(start.as_usize() + offset).unwrap();
        }
        Ok(())
    }