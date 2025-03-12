pub fn literal(&self) -> Option<Vec<u8>> {
        let rs = self.ranges();
        if rs.len() == 1 && rs[0].start == rs[0].end {
            Some(vec![rs[0].start])
        } else {
            None
        }
    }