pub fn try_insert(
        &mut self,
        pid: PatternID,
    ) -> Result<bool, PatternSetInsertError> {
        if pid.as_usize() >= self.capacity() {
            return Err(PatternSetInsertError {
                attempted: pid,
                capacity: self.capacity(),
            });
        }
        if self.which[pid] {
            return Ok(false);
        }
        self.len += 1;
        self.which[pid] = true;
        Ok(true)
    }