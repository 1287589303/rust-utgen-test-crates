pub(crate) fn validate(&self) -> Result<(), DeserializeError> {
        // Check that both ends of the range are DEAD or neither are.
        if self.min_match == DEAD && self.max_match != DEAD {
            err!("min_match is DEAD, but max_match is not");
        }
        if self.min_match != DEAD && self.max_match == DEAD {
            err!("max_match is DEAD, but min_match is not");
        }
        if self.min_accel == DEAD && self.max_accel != DEAD {
            err!("min_accel is DEAD, but max_accel is not");
        }
        if self.min_accel != DEAD && self.max_accel == DEAD {
            err!("max_accel is DEAD, but min_accel is not");
        }
        if self.min_start == DEAD && self.max_start != DEAD {
            err!("min_start is DEAD, but max_start is not");
        }
        if self.min_start != DEAD && self.max_start == DEAD {
            err!("max_start is DEAD, but min_start is not");
        }

        // Check that ranges are well formed.
        if self.min_match > self.max_match {
            err!("min_match should not be greater than max_match");
        }
        if self.min_accel > self.max_accel {
            err!("min_accel should not be greater than max_accel");
        }
        if self.min_start > self.max_start {
            err!("min_start should not be greater than max_start");
        }

        // Check that ranges are ordered with respect to one another.
        if self.matches() && self.quit_id >= self.min_match {
            err!("quit_id should not be greater than min_match");
        }
        if self.accels() && self.quit_id >= self.min_accel {
            err!("quit_id should not be greater than min_accel");
        }
        if self.starts() && self.quit_id >= self.min_start {
            err!("quit_id should not be greater than min_start");
        }
        if self.matches() && self.accels() && self.min_accel < self.min_match {
            err!("min_match should not be greater than min_accel");
        }
        if self.matches() && self.starts() && self.min_start < self.min_match {
            err!("min_match should not be greater than min_start");
        }
        if self.accels() && self.starts() && self.min_start < self.min_accel {
            err!("min_accel should not be greater than min_start");
        }

        // Check that max is at least as big as everything else.
        if self.max < self.quit_id {
            err!("quit_id should not be greater than max");
        }
        if self.max < self.max_match {
            err!("max_match should not be greater than max");
        }
        if self.max < self.max_accel {
            err!("max_accel should not be greater than max");
        }
        if self.max < self.max_start {
            err!("max_start should not be greater than max");
        }

        Ok(())
    }