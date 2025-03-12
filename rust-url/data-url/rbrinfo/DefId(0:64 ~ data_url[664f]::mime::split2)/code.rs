fn split2(s: &str, separator: char) -> (&str, Option<&str>) {
    let mut iter = s.splitn(2, separator);
    let first = iter.next().unwrap();
    (first, iter.next())
}