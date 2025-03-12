pub fn path_segments(&self) -> Option<str::Split<'_, char>> {
        let path = self.path();
        path.strip_prefix('/').map(|remainder| remainder.split('/'))
    }