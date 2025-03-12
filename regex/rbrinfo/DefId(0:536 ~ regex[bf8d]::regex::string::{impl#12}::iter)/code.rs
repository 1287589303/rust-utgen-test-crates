pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {
        SubCaptureMatches { haystack: self.haystack, it: self.caps.iter() }
    }