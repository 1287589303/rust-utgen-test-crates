fn from(mut iter: PercentEncode<'a>) -> Self {
        match iter.next() {
            None => "".into(),
            Some(first) => match iter.next() {
                None => first.into(),
                Some(second) => {
                    let mut string = first.to_owned();
                    string.push_str(second);
                    string.extend(iter);
                    string.into()
                }
            },
        }
    }