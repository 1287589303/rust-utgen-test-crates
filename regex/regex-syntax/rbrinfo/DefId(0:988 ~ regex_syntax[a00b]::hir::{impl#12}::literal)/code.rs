pub fn literal(&self) -> Option<Vec<u8>> {
        let rs = self.ranges();
        if rs.len() == 1 && rs[0].start == rs[0].end {
            Some(rs[0].start.encode_utf8(&mut [0; 4]).to_string().into_bytes())
        } else {
            None
        }
    }