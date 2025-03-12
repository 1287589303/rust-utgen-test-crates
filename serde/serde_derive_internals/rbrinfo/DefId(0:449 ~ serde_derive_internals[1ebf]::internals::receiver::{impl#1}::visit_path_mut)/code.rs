fn visit_path_mut(&mut self, path: &mut Path) {
        for segment in &mut path.segments {
            self.visit_path_arguments_mut(&mut segment.arguments);
        }
    }