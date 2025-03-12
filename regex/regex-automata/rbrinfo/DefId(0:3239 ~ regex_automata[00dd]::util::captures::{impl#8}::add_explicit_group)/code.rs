fn add_explicit_group<N: AsRef<str>>(
        &mut self,
        pid: PatternID,
        group: SmallIndex,
        maybe_name: Option<N>,
    ) -> Result<(), GroupInfoError> {
        // We also need to check that the slot index generated for
        // this group is also valid. Although, this is a little weird
        // because we offset these indices below, at which point, we'll
        // have to recheck them. Gosh this is annoying. Note that
        // the '+2' below is OK because 'end' is guaranteed to be less
        // than isize::MAX.
        let end = &mut self.slot_ranges[pid].1;
        *end = SmallIndex::new(end.as_usize() + 2).map_err(|_| {
            GroupInfoError::too_many_groups(pid, group.as_usize())
        })?;
        if let Some(name) = maybe_name {
            let name = Arc::<str>::from(name.as_ref());
            if self.name_to_index[pid].contains_key(&*name) {
                return Err(GroupInfoError::duplicate(pid, &name));
            }
            let len = name.len();
            self.name_to_index[pid].insert(Arc::clone(&name), group);
            self.index_to_name[pid].push(Some(name));
            // Adds the memory used by the Arc<str> in both maps.
            self.memory_extra +=
                2 * (len + core::mem::size_of::<Option<Arc<str>>>());
            // And also the value entry for the 'name_to_index' map.
            // This is probably an underestimate for 'name_to_index' since
            // hashmaps/btrees likely have some non-zero overhead, but we
            // assume here that they have zero overhead.
            self.memory_extra += core::mem::size_of::<SmallIndex>();
        } else {
            self.index_to_name[pid].push(None);
            self.memory_extra += core::mem::size_of::<Option<Arc<str>>>();
        }
        // This is a sanity assert that checks that our group index
        // is in line with the number of groups added so far for this
        // pattern.
        assert_eq!(group.one_more(), self.group_len(pid));
        // And is also in line with the 'index_to_name' map.
        assert_eq!(group.one_more(), self.index_to_name[pid].len());
        Ok(())
    }