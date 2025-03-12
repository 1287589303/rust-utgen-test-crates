pub fn get_visited_capacity(&self) -> usize {
        const DEFAULT: usize = 256 * (1 << 10); // 256 KB
        self.visited_capacity.unwrap_or(DEFAULT)
    }