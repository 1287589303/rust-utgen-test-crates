fn next(&mut self) -> Option<char> {
        self.chars.by_ref().find(|&c| !ascii_tab_or_new_line(c))
    }