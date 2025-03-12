pub fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>
    where
        P: IntoIterator<Item = G>,
        G: IntoIterator<Item = Option<N>>,
        N: AsRef<str>,
    {
        let mut group_info = GroupInfoInner {
            slot_ranges: vec![],
            name_to_index: vec![],
            index_to_name: vec![],
            memory_extra: 0,
        };
        for (pattern_index, groups) in pattern_groups.into_iter().enumerate() {
            // If we can't convert the pattern index to an ID, then the caller
            // tried to build capture info for too many patterns.
            let pid = PatternID::new(pattern_index)
                .map_err(GroupInfoError::too_many_patterns)?;

            let mut groups_iter = groups.into_iter().enumerate();
            match groups_iter.next() {
                None => return Err(GroupInfoError::missing_groups(pid)),
                Some((_, Some(_))) => {
                    return Err(GroupInfoError::first_must_be_unnamed(pid))
                }
                Some((_, None)) => {}
            }
            group_info.add_first_group(pid);
            // Now iterate over the rest, which correspond to all of the
            // (conventionally) explicit capture groups in a regex pattern.
            for (group_index, maybe_name) in groups_iter {
                // Just like for patterns, if the group index can't be
                // converted to a "small" index, then the caller has given too
                // many groups for a particular pattern.
                let group = SmallIndex::new(group_index).map_err(|_| {
                    GroupInfoError::too_many_groups(pid, group_index)
                })?;
                group_info.add_explicit_group(pid, group, maybe_name)?;
            }
        }
        group_info.fixup_slot_ranges()?;
        Ok(GroupInfo(Arc::new(group_info)))
    }