pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {
        SubCaptureMatches {
            caps: self,
            it: self.pikevm.nfa().capture_names().enumerate(),
        }
    }