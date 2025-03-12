fn new(o: Utf8Range, n: Utf8Range) -> Option<Split> {
        let range = |r: RangeInclusive<u8>| Utf8Range {
            start: *r.start(),
            end: *r.end(),
        };
        let old = |r| SplitRange::Old(range(r));
        let new = |r| SplitRange::New(range(r));
        let both = |r| SplitRange::Both(range(r));

        // Use same names as the comment above to make it easier to compare.
        let (a, b, x, y) = (o.start, o.end, n.start, n.end);

        if b < x || y < a {
            // case 1, case 2
            None
        } else if a == x && b == y {
            // case 3
            Some(Split::parts1(both(a..=b)))
        } else if a == x && b < y {
            // case 4
            Some(Split::parts2(both(a..=b), new(b + 1..=y)))
        } else if b == y && a > x {
            // case 5
            Some(Split::parts2(new(x..=a - 1), both(a..=b)))
        } else if x == a && y < b {
            // case 6
            Some(Split::parts2(both(x..=y), old(y + 1..=b)))
        } else if y == b && x > a {
            // case 7
            Some(Split::parts2(old(a..=x - 1), both(x..=y)))
        } else if a > x && b < y {
            // case 8
            Some(Split::parts3(new(x..=a - 1), both(a..=b), new(b + 1..=y)))
        } else if x > a && y < b {
            // case 9
            Some(Split::parts3(old(a..=x - 1), both(x..=y), old(y + 1..=b)))
        } else if b == x && a < y {
            // case 10
            Some(Split::parts3(old(a..=b - 1), both(b..=b), new(b + 1..=y)))
        } else if y == a && x < b {
            // case 11
            Some(Split::parts3(new(x..=y - 1), both(y..=y), old(y + 1..=b)))
        } else if b > x && b < y {
            // case 12
            Some(Split::parts3(old(a..=x - 1), both(x..=b), new(b + 1..=y)))
        } else if y > a && y < b {
            // case 13
            Some(Split::parts3(new(x..=a - 1), both(a..=y), old(y + 1..=b)))
        } else {
            unreachable!()
        }
    }