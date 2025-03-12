fn intersects(r1: Utf8Range, r2: Utf8Range) -> bool {
    !(r1.end < r2.start || r2.end < r1.start)
}