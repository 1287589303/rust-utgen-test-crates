pub fn subspan<R: RangeBounds<usize>>(this: &Literal, range: R) -> Option<Span> {
    this.subspan(range)
}