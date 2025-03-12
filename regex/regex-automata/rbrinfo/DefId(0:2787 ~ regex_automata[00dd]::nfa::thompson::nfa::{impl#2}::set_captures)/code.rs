pub(super) fn set_captures(
        &mut self,
        captures: &[Vec<Option<Arc<str>>>],
    ) -> Result<(), GroupInfoError> {
        self.group_info = GroupInfo::new(
            captures.iter().map(|x| x.iter().map(|y| y.as_ref())),
        )?;
        Ok(())
    }